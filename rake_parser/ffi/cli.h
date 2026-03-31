#pragma once

#include "util.h"
#include <stddef.h>
#ifdef __cplusplus
extern "C" {
#endif

Result rake_parseCLI(int argc, char **argv);

struct CliParseResult {
    char **tasks;
    size_t tasks_len;
};

#ifdef __cplusplus
}
#endif
