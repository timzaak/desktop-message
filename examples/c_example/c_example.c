#include "deskmsg_c.h"
#include <signal.h>
#include <windows.h>
#include <stdio.h>

volatile sig_atomic_t stop = 0;

// Signal handler function
void handle_signal(int sig) {
    if (sig == SIGINT || sig == SIGTERM) {
        printf("\nReceived signal %d, stopping...\n", sig);
        stop = 1;
    }
}

int main() {
    const char *config = "{ \"mqtt_address\": \"0.0.0.0:1883\", \"http_address\": \"0.0.0.0:0\", \"basic_path\": \"\", \"http_auth_token\":\"abc\" }";

    char config_buffer[2048] = {0};
    deskmsg_ErrorCode code = deskmsg_start_server(config);

    deskmsg_get_config(config_buffer);

    deskmsg_ErrorCode result = deskmsg_get_config(config_buffer); // 调用函数

    if (result == Ok) {
        printf("get config success:\n%s\n", config_buffer);
    } else {
        printf("get config failure: %d\n", result);
    }

    signal(SIGINT, handle_signal);   // Ctrl+C
    signal(SIGTERM, handle_signal);  // kill command

    //while (!stop) {
    //    Sleep(2000);// Sleep until a signal is received
    //}

    return code;
}
