#ifndef TINY_PROTOCOL
#define TINY_PROTOCOL

#pragma once

// don't change this, it auto generated

typedef enum tiny_protocol_ErrorCode {
  Ok = 0,
  BadConfig = 1,
  StartServerError = 2,
  InvalidServerPoint = 3,
  ServerHasInit = 4,
} tiny_protocol_ErrorCode;

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

enum tiny_protocol_ErrorCode tiny_protocol_get_config(char *output);

enum tiny_protocol_ErrorCode tiny_protocol_start_server(const char *config);

#ifdef __cplusplus
}  // extern "C"
#endif  // __cplusplus

#endif  /* TINY_PROTOCOL */
