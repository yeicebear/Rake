use std::fs;
use regex::Regex;
use std::path::Path;
use std::ffi::{CStr, c_char, CString};
use std::collections::HashMap;
#[unsafe(no_mangle)]
pub extern "C" fn get_commands(section: *const c_char) -> *mut c_char {
    let section = unsafe {
        CStr::from_ptr(section).to_str().unwrap_or("")
    };
    let file_name = "Rakefile";
    if !Path::new(file_name).exists() {
        let error = "[ERR] Rakefile not found";
        return CString::new(error).unwrap().into_raw();
    }
    let content = match fs::read_to_string(file_name) {
        Ok(c) => c,
        Err(_) => {
            let error = "[ERR] Could not read Rakefile";
            return CString::new(error).unwrap().into_raw();
        }
    };
    let re_header = Regex::new(r"\[(.*?)\]").unwrap();
    let re_command = Regex::new(r"\d+\)\s+(.*)").unwrap();
    let mut sections: HashMap<String, Vec<String>> = HashMap::new();
    let mut current_header = String::new();
    for line in content.lines() {
        if let Some(cap) = re_header.captures(line) {
            current_header = cap[1].to_string();
        } else if let Some(cap) = re_command.captures(line) {
            let cmd_raw = cap[1].to_string();
            sections.entry(current_header.clone()).or_insert(Vec::new()).push(cmd_raw);
        }
    }

    if let Some(commands) = sections.get(section) {
        let result = commands.join("\n");
        CString::new(result).unwrap().into_raw()
    } else {
        let error = format!("[ERROR] Section {} not found.", section);
        CString::new(error).unwrap().into_raw()
    }
}