#ifndef manifest_h
#define manifest_h

#include <stddef.h>
#include <stdint.h>

/* Opaque handle to a Go-managed list of extracted packages. */
typedef uintptr_t manifest;

/*
 * manifest_parse parses `data` (length `data_len` bytes) using the extractor
 * for `ecosystem` (e.g. "npm", "PyPI", "crates.io").
 *
 * On success: writes the handle to *out and returns "".
 * On failure: sets *out to 0 and returns the error message.
 *
 * The returned char* is heap-allocated and must be free()d by the caller.
 * On success the caller must eventually call manifest_free().
 */
char* manifest_parse(char* ecosystem,
                     uint8_t* data, size_t data_len,
                     manifest* out);

/*
 * manifest_packages_len writes the number of packages in the list to *out.
 * The returned char* is heap-allocated and must be free()d by the caller.
 */
char* manifest_packages_len(manifest m, size_t* out);

/*
 * manifest_package_name writes the name of the idx-th package to *out.
 * Both the returned char* and *out are heap-allocated; caller must free() each.
 */
char* manifest_package_name(manifest m, size_t idx, char** out);

/*
 * manifest_package_version writes the version of the idx-th package to *out.
 * Both the returned char* and *out are heap-allocated; caller must free() each.
 */
char* manifest_package_version(manifest m, size_t idx, char** out);

/*
 * manifest_package_ecosystem writes the OSV ecosystem of the idx-th package to *out
 * (e.g. "npm", "PyPI", "crates.io").
 * Both the returned char* and *out are heap-allocated; caller must free() each.
 */
char* manifest_package_ecosystem(manifest m, size_t idx, char** out);

/*
 * manifest_free releases the Go-side handle.  Must be called exactly once
 * for every handle returned by manifest_parse.
 */
void manifest_free(manifest m);

#endif /* manifest_h */
