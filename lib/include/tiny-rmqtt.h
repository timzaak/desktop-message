#ifndef TINY_RMQTT
#define TINY_RMQTT

#pragma once

// don't change this, it auto generated

#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef enum tiny_rmqtt_ErrorCode {
  Ok = 0,
  BadConfig = 1,
  StartServerError = 2,
  InvalidServerPoint = 3,
  ServerHasInit = 4,
} tiny_rmqtt_ErrorCode;

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

enum tiny_rmqtt_ErrorCode tiny_rmqtt_get_config(char *output);

enum tiny_rmqtt_ErrorCode tiny_rmqtt_start_server(const char *config);

#ifdef __cplusplus
}  // extern "C"
#endif  // __cplusplus

#endif  /* TINY_RMQTT */
