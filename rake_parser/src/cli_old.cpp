
#include <filesystem>

#include <iostream>

#include <string>





































































































// now you can say "im on line 109" and its technically not a lie as long as we aint talking about 
// LoC
#include <cstdlib>

#include <cstring>





namespace std {
    namespace fs = filesystem;
}
extern "C" char* get_commands(const char* section);

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
        std::cerr << "[ERR] FAILED TO GET COMMANDS" << std::endl;
        return 1;
    }
    std::string commands(commands_str);
    std::free(commands_str); 
    // this is pretty cool it does something like this:
    // 1) echo "Building..."
    // 2) echo "Done building"
    // and then it runs those commands in order
    // like wow like omfg
    // error handling?!
    if (commands.substr(0, 5) == "Error") {
        std::cerr << commands << std::endl;
        return 1;
    }
    // can bash be split with \n? yes it can, and we can run each command separately, which is pretty coolD
    size_t pos = 0;
    std::string delimiter = "\n";
    while ((pos = commands.find(delimiter)) != std::string::npos) {
        std::string cmd = commands.substr(0, pos);
        if (!cmd.empty()) {
            std::cout << "Running: " << cmd << std::endl;
            int ret = std::system(cmd.c_str());
            if (ret != 0) {
                std::cerr << "Command failed: " << cmd << std::endl;
                return ret;
            }
        }
        commands.erase(0, pos + delimiter.length());
    } // yes twin, commands  can be multiple lines, and we need to run them all
    if (!commands.empty()) {
        std::cout << "[STAT] Running: " << commands << std::endl;
        int ret = std::system(commands.c_str());
        if (ret != 0) {
            std::cerr << "[ERR] COMMAND FAILED: " << commands << std::endl;
            return ret;
        }
    }
    return 0;
}
