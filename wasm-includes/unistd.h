#ifndef _WASM_SHIM_UNISTD_H
#define _WASM_SHIM_UNISTD_H
#include <stddef.h>
/* Added readlink */
typedef long ssize_t;
ssize_t readlink(const char *path, char *buf, size_t bufsiz);
#endif
