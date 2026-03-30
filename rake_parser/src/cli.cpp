#include <filesystem>
#include <iostream>
#include <string>

namespace std {
    namespace fs = filesystem;
}

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
