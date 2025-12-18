use std::ffi::{CStr, CString};

use libc::c_char;

/// Frees a string that was allocated by the Rust library.
/// This should be used to free strings returned by zen functions.
/// # Safety
/// The pointer must have been allocated by CString::into_raw() from this library.
#[unsafe(no_mangle)]
pub extern "C" fn zen_free_string(ptr: *mut c_char) {
    if !ptr.is_null() {
        unsafe {
            // Reconstruct the CString and let it drop to free the memory
            let _ = CString::from_raw(ptr);
        }
    }
}

/// Frees a byte buffer that was allocated by the Rust library.
/// # Safety
/// The pointer must have been allocated by Box::into_raw() from this library.
#[unsafe(no_mangle)]
pub extern "C" fn zen_free_bytes(ptr: *mut u8) {
    if !ptr.is_null() {
        unsafe {
            // Reconstruct the Box and let it drop to free the memory
            let _ = Box::from_raw(ptr);
        }
    }
}

pub(crate) fn safe_cstr_from_ptr<'a>(ptr: *const c_char) -> Option<&'a CStr> {
    if ptr.is_null() {
        None
    } else {
        // SAFETY: The caller must ensure the pointer is not null and points to a valid, null-terminated C string.
        // This unsafe block is necessary because CStr::from_ptr inherently requires an unsafe operation.
        Some(unsafe { CStr::from_ptr(ptr) })
    }
}

pub(crate) fn safe_str_from_ptr<'a>(ptr: *const c_char) -> Option<&'a str> {
    if ptr.is_null() {
        None
    } else {
        // SAFETY: The caller must ensure the pointer is not null and points to a valid, null-terminated C string.
        // This unsafe block is necessary because CStr::from_ptr inherently requires an unsafe operation.
        Some(unsafe { CStr::from_ptr(ptr) }.to_str().ok()?)
    }
}
