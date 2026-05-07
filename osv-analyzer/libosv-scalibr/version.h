/*
 * version.h — opaque handle to a parsed ecosystem version.
 *
 * Lifecycle:
 *   1. version_new()  — parse a version string; returns an error message
 *                        (empty string on success) and writes the handle to
 *                        an out-parameter.  Free the returned string with free().
 *   2. version_cmp()  — compare two handles; returns an error message
 *                        (empty string on success) and writes -1/0/+1 to an
 *                        out-parameter.  Free the returned string with free().
 *   3. version_free()  — release the handle; must be called exactly once
 *                        to avoid a Go-side memory leak.
 */
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