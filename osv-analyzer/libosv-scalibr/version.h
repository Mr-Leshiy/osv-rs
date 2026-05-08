#ifndef version_h
#define version_h

#include <stdint.h>

/* Opaque handle to a Go-managed Version object. */
typedef uintptr_t version;

/*
 * version_new parses `str` as a version in `ecosystem` (e.g. "npm", "PyPI").
 * On success: writes the handle to *out and returns an empty string ("").
 * On failure: sets *out to 0 and returns the error message.
 * The returned string is heap-allocated and must be free()d by the caller.
 */
char* version_new(char* str, char* ecosystem, version* out);

/*
 * version_cmp compares two versions from the same ecosystem.
 * On success: writes -1, 0, or +1 to *result and returns an empty string.
 * On failure: returns the error message (e.g. versions are from different ecosystems).
 * The returned string is heap-allocated and must be free()d by the caller.
 */
char* version_cmp(version a, version b, int* result);

/*
 * version_free releases the Go-side handle.  Must be called exactly once
 * for every handle returned by version_new.
 */
void version_free(version ref);

#endif