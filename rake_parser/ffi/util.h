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
<<<<<<< HEAD

// C++ Implements these
Result rake_ResultOk(void *value);
Result rake_ResultError(const char *msg);
// END

=======
>>>>>>> fed1b76fd5a3de454d4e0d558a69c5d81512d43e
#ifdef __cplusplus
}
#endif

