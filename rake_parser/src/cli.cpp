#include <cstring>
#include <filesystem>
#include <string>
#include <vector>
#include "../ffi/cli.h"
#include "../ffi/util.h"

namespace std {
    namespace fs = filesystem;
}

namespace rake {
    char **to_c_array(const std::vector<std::string> &vec) {
        char **out = new char*[vec.size()];

        for (size_t i = 0; i < vec.size(); ++i) {
            const std::string &s = vec[i];
            out[i] = new char[s.size() + 1];
            std::memcpy(out[i], s.c_str(), s.size() + 1);
        }

        return out;
    }

    static Result parseCLI(int argc, char *argv[]) {
        std::vector<std::string> tasks;
        for (int i = 0; i < argc; ++i) {
            std::string raw_argument = argv[i];
            std::string argument;
            if (raw_argument)

            if (argument == "asd") {

            }
            else {
                tasks.push_back(argument);
            }
        }
        return rake_ResultOk(to_c_array(tasks));
    }
}

extern "C" {
    Result rake_parseCLI(int argc, char **argv) {
        return rake::parseCLI(argc, argv);
    }

    Result rake_ResultOk(void *value) {
        Result res;
        res.code = RESULT_OK;
        res.data.value = value;
        return res;
    }

    Result rake_ResultError(const char *msg) {
        Result res;
        res.code = RESULT_ERR;
        res.data.msg = msg;
        return res;
    }
}
