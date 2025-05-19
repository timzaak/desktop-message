#ifndef TINY_RMQTT
#define TINY_RMQTT

// don't change this, it auto generated

#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef enum ErrorCode {
  Ok = 0,
  BadConfig = 1,
  StartServerError = 2,
  InvalidServerPoint = 3,
  ServerHasInit = 4,
} ErrorCode;

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

void rust_function(void);

enum ErrorCode get_config(char *output);

enum ErrorCode start_server(const char *config);

#ifdef __cplusplus
}  // extern "C"
#endif  // __cplusplus

#endif  /* TINY_RMQTT */
