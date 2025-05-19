#include "../include/tiny-rmqtt.h"
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
    const char *config = "{ \"mqtt_address\": \"0.0.0.0:1883\", \"http_address\": \"0.0.0.0:8080\", \"basic_path\": \"\" }";
    ErrorCode code = start_server(config);

    rust_function();
    signal(SIGINT, handle_signal);   // Ctrl+C
    signal(SIGTERM, handle_signal);  // kill command

    while (!stop) {
        Sleep(2000);// Sleep until a signal is received
    }

    return code;
}
