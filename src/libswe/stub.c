#include <stdio.h>
#include <stdarg.h>

// Stubs for variadic functions that can't be implemented in Rust easily
// These satisfy link dependencies for Swiss Ephemeris when compiling to Wasm

int fprintf(FILE *stream, const char *format, ...) {
    return 0;
}

int printf(const char *format, ...) {
    return 0;
}

int sprintf(char *str, const char *format, ...) {
    if (str) *str = '\0';
    return 0;
}

int sscanf(const char *str, const char *format, ...) {
    return 0;
}
