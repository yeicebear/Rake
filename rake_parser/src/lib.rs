use std::fs;
use std::path::Path;
use std::ffi::{CStr, c_char, CString};
use std::collections::HashMap;
use regex::Regex;
use std::time::SystemTime;

#[derive(Clone, Debug)]
struct TaskMetadata {
    inputs: Vec<String>,
    outputs: Vec<String>,
    depends: Vec<String>,
    hash: String,
}

fn hash_files(patterns: &[String]) -> String {
    let mut hashes = Vec::new();

    for pattern in patterns {
        let mut file_hash = String::new();
        
        // Try glob pattern first
        if pattern.contains('*') || pattern.contains('?') {
            if let Ok(entries) = glob_simple(pattern) {
                for path in entries {
                    if let Ok(metadata) = fs::metadata(&path) {
                        if let Ok(modified) = metadata.modified() {
                            if let Ok(duration) = modified.duration_since(SystemTime::UNIX_EPOCH) {
                                file_hash.push_str(&duration.as_nanos().to_string());
                                file_hash.push('|');
                            }
                        }
                    }
                }
            }
        } else if Path::new(pattern).exists() {
            if let Ok(metadata) = fs::metadata(pattern) {
                if let Ok(modified) = metadata.modified() {
                    if let Ok(duration) = modified.duration_since(SystemTime::UNIX_EPOCH) {
                        file_hash.push_str(&duration.as_nanos().to_string());
                    }
                }
            }
        }
        
        if !file_hash.is_empty() {
            hashes.push(file_hash);
        }
    }

    // Combine all hashes
    let combined = hashes.join("::");
    
    // Simple hash by summing all char values
    let hash_value: u64 = combined.bytes().map(|b| b as u64).sum();
    format!("{:x}", hash_value)
}

fn glob_simple(pattern: &str) -> Result<Vec<String>, std::io::Error> {
    let mut results = Vec::new();
    let parts: Vec<&str> = pattern.split('/').collect();

    if parts.is_empty() {
        return Ok(results);
    }

    let (dir, file_pattern) = if parts.len() > 1 {
        (parts[..parts.len() - 1].join("/"), parts[parts.len() - 1])
    } else {
        (".".to_string(), parts[0])
    };

    if let Ok(entries) = fs::read_dir(&dir) {
        for entry in entries.flatten() {
            if let Ok(metadata) = entry.metadata() {
                let path = entry.path();
                let file_name = path.file_name().unwrap_or_default().to_string_lossy();

                if matches_pattern(&file_name, file_pattern) {
                    if metadata.is_file() {
                        results.push(path.to_string_lossy().to_string());
                    }
                }
            }
        }
    }

    Ok(results)
}

fn matches_pattern(filename: &str, pattern: &str) -> bool {
    if pattern == "*" {
        return true;
    }

    let re = pattern
        .replace(".", r"\.")
        .replace("*", ".*")
        .replace("?", ".");

    Regex::new(&format!("^{}$", re))
        .map(|r| r.is_match(filename))
        .unwrap_or(false)
}

fn outputs_exist(outputs: &[String]) -> bool {
    outputs.iter().all(|out| Path::new(out).exists())
}

fn task_needs_run(
    task_name: &str,
    metadata: &TaskMetadata,
    cache: &HashMap<String, String>,
) -> bool {
    // If outputs don't exist, must run
    if !outputs_exist(&metadata.outputs) {
        return true;
    }

    // If inputs have changed, must run
    let current_hash = hash_files(&metadata.inputs);
    if cache.get(task_name) != Some(&current_hash) {
        return true;
    }

    false
}

#[unsafe(no_mangle)]
pub extern "C" fn get_commands(section: *const c_char) -> *mut c_char {
    let section = unsafe {
        CStr::from_ptr(section).to_str().unwrap_or("")
    };

    let file_name = "Rakefile";

    if !Path::new(file_name).exists() {
        let error = "[ERROR] Rakefile not found";
        return CString::new(error).unwrap().into_raw();
    }

    let content = match fs::read_to_string(file_name) {
        Ok(c) => c,
        Err(_) => {
            let error = "[ERROR] Could not read Rakefile";
            return CString::new(error).unwrap().into_raw();
        }
    };

    // Load cache
    let mut cache: HashMap<String, String> = HashMap::new();
    if let Ok(cache_content) = fs::read_to_string(".rake_cache") {
        for line in cache_content.lines() {
            if let Some(pos) = line.find('=') {
                let (key, value) = line.split_at(pos);
                cache.insert(key.to_string(), value[1..].to_string());
            }
        }
    }

    // Parse Rakefile
    let re_header = Regex::new(r"\[(.*?)\]").unwrap();
    let re_inputs = Regex::new(r"inputs:\s*(.+)").unwrap();
    let re_outputs = Regex::new(r"outputs:\s*(.+)").unwrap();
    let re_depends = Regex::new(r"depends:\s*(.+)").unwrap();
    let re_command = Regex::new(r"^\d+\)\s+(.*)").unwrap();

    let mut tasks: HashMap<String, TaskMetadata> = HashMap::new();
    let mut sections: HashMap<String, Vec<String>> = HashMap::new();
    let mut current_task = String::new();
    let mut current_inputs: Vec<String> = Vec::new();
    let mut current_outputs: Vec<String> = Vec::new();
    let mut current_depends: Vec<String> = Vec::new();

    for line in content.lines() {
        let trimmed = line.trim();

        if let Some(cap) = re_header.captures(trimmed) {
            current_task = cap[1].to_string();
            current_inputs.clear();
            current_outputs.clear();
            current_depends.clear();
        } else if let Some(cap) = re_inputs.captures(trimmed) {
            current_inputs = cap[1]
                .split(',')
                .map(|s| s.trim().to_string())
                .collect();
        } else if let Some(cap) = re_outputs.captures(trimmed) {
            current_outputs = cap[1]
                .split(',')
                .map(|s| s.trim().to_string())
                .collect();
        } else if let Some(cap) = re_depends.captures(trimmed) {
            current_depends = cap[1]
                .split(',')
                .map(|s| s.trim().to_string())
                .collect();
        } else if let Some(cap) = re_command.captures(trimmed) {
            let cmd = cap[1].to_string();
            sections.entry(current_task.clone()).or_insert(Vec::new()).push(cmd);

            // Store metadata for this task
            let hash = hash_files(&current_inputs);
            tasks.insert(
                current_task.clone(),
                TaskMetadata {
                    inputs: current_inputs.clone(),
                    outputs: current_outputs.clone(),
                    depends: current_depends.clone(),
                    hash: hash.clone(),
                },
            );
        }
    }

    // Check if task needs to run
    if let Some(metadata) = tasks.get(section) {
        if !task_needs_run(section, metadata, &cache) {
            let skip_msg = format!("[CACHED] Task '{}' is up to date - skipping", section);
            return CString::new(skip_msg).unwrap().into_raw();
        }
    }

    if let Some(commands) = sections.get(section) {
        let result = commands.join("\n");
        CString::new(result).unwrap().into_raw()
    } else {
        let error = format!("[ERROR] Section '{}' not found.", section);
        CString::new(error).unwrap().into_raw()
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn update_cache(section: *const c_char) {
    let section = unsafe {
        CStr::from_ptr(section).to_str().unwrap_or("")
    };

    let file_name = "Rakefile";
    if !Path::new(file_name).exists() {
        return;
    }

    let content = match fs::read_to_string(file_name) {
        Ok(c) => c,
        Err(_) => return,
    };

    let re_header = Regex::new(r"\[(.*?)\]").unwrap();
    let re_inputs = Regex::new(r"inputs:\s*(.+)").unwrap();

    let mut found_section = false;
    let mut current_inputs: Vec<String> = Vec::new();

    // Find the section and its inputs
    for line in content.lines() {
        let trimmed = line.trim();

        if let Some(cap) = re_header.captures(trimmed) {
            if found_section {
                // We passed the section, stop
                break;
            }
            if cap[1].to_string() == section {
                found_section = true;
            }
        } else if found_section && !trimmed.starts_with('[') {
            if let Some(cap) = re_inputs.captures(trimmed) {
                current_inputs = cap[1]
                    .split(',')
                    .map(|s| s.trim().to_string())
                    .collect();
            }
            // Keep checking until we hit a command or next section
            if trimmed.starts_with(|c: char| c.is_ascii_digit()) {
                // Hit a command, inputs parsing is done
                break;
            }
        }
    }

    if found_section {
        let hash = hash_files(&current_inputs);
        let mut cache_content = String::new();

        // Load existing cache
        if let Ok(existing) = fs::read_to_string(".rake_cache") {
            for line in existing.lines() {
                if !line.starts_with(&format!("{}=", section)) {
                    cache_content.push_str(line);
                    cache_content.push('\n');
                }
            }
        }

        // Add/update this task
        cache_content.push_str(&format!("{}={}\n", section, hash));

        // Write cache
        let _ = fs::write(".rake_cache", cache_content);
    }
}
