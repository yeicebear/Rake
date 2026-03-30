#pragma once

#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif
enum {
    RESULT_OK = 0,
    RESULT_ERR = 1
};
union ResultData {
    void *value;
    const char *msg;
};
struct Result {
    uint8_t code;
    ResultData data;
};
#ifdef __cplusplus
}
#endif

