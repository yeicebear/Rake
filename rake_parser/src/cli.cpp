#include <filesystem>
#include <iostream>
#include <string>
#include <cstdlib>
#include <cstring>

namespace std {
    namespace fs = filesystem;
}

extern "C" char* get_commands(const char* section);
extern "C" void update_cache(const char* section);

namespace rake {
    int parseCLI(int argc, char *argv[]) {
        for (int i = 0; i < argc; ++i) {
            std::string raw_argument = argv[i];
            std::cout << raw_argument << '\n';
        }
        return 0;
    }
}

extern "C" int rake_parseCLI(int argc, char **argv) {
    return rake::parseCLI(argc, argv);
}

int main(int argc, char *argv[]) {
    if (argc < 2 || std::string(argv[1]).substr(0, 2) != "--") {
        std::cerr << "Usage: " << argv[0] << " --<section>" << std::endl;
        return 1;
    }

    std::string section = std::string(argv[1]).substr(2);
    char* commands_str = get_commands(section.c_str());

    if (!commands_str) {
        std::cerr << "[ERROR] Failed to get commands" << std::endl;
        return 1;
    }

    std::string commands(commands_str);
    std::free(commands_str);

    // Check for error messages
    if (commands.substr(0, 7) == "[ERROR]") {
        std::cerr << commands << std::endl;
        return 1;
    }

    // Check for cached tasks (skip execution)
    if (commands.substr(0, 8) == "[CACHED]") {
        std::cout << commands << std::endl;
        return 0;
    }

    // Execute commands
    size_t pos = 0;
    std::string delimiter = "\n";
    bool any_failed = false;

    while ((pos = commands.find(delimiter)) != std::string::npos) {
        std::string cmd = commands.substr(0, pos);
        if (!cmd.empty()) {
            std::cout << "[EXEC] " << cmd << std::endl;
            int ret = std::system(cmd.c_str());
            if (ret != 0) {
                std::cerr << "[ERROR] Command failed with code " << ret << ": " << cmd << std::endl;
                any_failed = true;
                break;
            }
        }
        commands.erase(0, pos + delimiter.length());
    }

    // Execute last command if any
    if (!any_failed && !commands.empty()) {
        std::cout << "[EXEC] " << commands << std::endl;
        int ret = std::system(commands.c_str());
        if (ret != 0) {
            std::cerr << "[ERROR] Command failed with code " << ret << ": " << commands << std::endl;
            return ret;
        }
    }

    if (any_failed) {
        return 1;
    }

    // Update cache after successful execution
    update_cache(section.c_str());
    std::cout << "[SUCCESS] Task '" << section << "' completed and cached" << std::endl;

    return 0;
}
