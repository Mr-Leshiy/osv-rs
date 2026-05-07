//! Shared C FFI declarations used by multiple modules in this crate.

use std::ffi::c_void;

#[allow(clippy::missing_docs_in_private_items)]
unsafe extern "C" {
    /// Frees a heap-allocated pointer returned by the C library.
    pub(crate) fn free(ptr: *mut c_void);
}
