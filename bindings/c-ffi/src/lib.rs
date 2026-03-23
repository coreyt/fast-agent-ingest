//! C ABI for fast-agent-ingest.
//!
//! Generated C header: run `cbindgen --config cbindgen.toml`
//! which writes `../../include/fast_agent_ingest.h`.
//!
//! Consumed by:
//!   - Go       (cgo, see bindings/go/)
//!   - C#       (P/Invoke, see bindings/csharp/)
//!   - C++      (header wrapper, see bindings/cpp/)

use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use fast_agent_ingest_core::{convert as core_convert, ConversionOptions};

/// Options passed across the C boundary.
///
/// Zero-initialise and set the fields you need; unset fields use defaults.
#[repr(C)]
pub struct FaiOptions {
    /// Non-zero to run readability-style main-content extraction.
    pub extract_main_content: u8,
    /// Non-zero to emit Markdown image syntax.
    pub include_images: u8,
    /// Non-zero to emit Markdown link syntax.
    pub include_links: u8,
}

/// Result returned across the C boundary.
///
/// Both pointers are heap-allocated UTF-8 C strings.
/// Caller MUST free them with `fai_free_string`.
/// Either pointer may be NULL if the corresponding field was not present.
#[repr(C)]
pub struct FaiResult {
    pub markdown: *mut c_char,
    pub title: *mut c_char,
    pub description: *mut c_char,
}

/// Convert HTML to Markdown.
///
/// # Safety
/// - `html` must be a valid, non-null, null-terminated UTF-8 C string.
/// - `options` may be null; if so, defaults are used.
/// - The returned `FaiResult` must be freed with `fai_free_result`.
#[no_mangle]
pub unsafe extern "C" fn fai_convert(
    html: *const c_char,
    options: *const FaiOptions,
) -> FaiResult {
    let html_str = unsafe {
        assert!(!html.is_null());
        CStr::from_ptr(html).to_str().unwrap_or("")
    };

    let opts = if options.is_null() {
        ConversionOptions::default()
    } else {
        let o = unsafe { &*options };
        ConversionOptions {
            extract_main_content: o.extract_main_content != 0,
            include_images: o.include_images != 0,
            include_links: o.include_links != 0,
            ..Default::default()
        }
    };

    let result = core_convert(html_str, &opts);

    FaiResult {
        markdown:    cstring_or_null(Some(result.markdown)),
        title:       cstring_or_null(result.title),
        description: cstring_or_null(result.description),
    }
}

/// Free a string returned by this library.
///
/// # Safety
/// `ptr` must have been returned by `fai_convert` and not yet freed.
/// Passing NULL is a no-op.
#[no_mangle]
pub unsafe extern "C" fn fai_free_string(ptr: *mut c_char) {
    if !ptr.is_null() {
        let _ = unsafe { CString::from_raw(ptr) };
    }
}

/// Free a `FaiResult` returned by `fai_convert`.
///
/// # Safety
/// `result` must have been returned by `fai_convert`.
#[no_mangle]
pub unsafe extern "C" fn fai_free_result(result: FaiResult) {
    unsafe {
        fai_free_string(result.markdown);
        fai_free_string(result.title);
        fai_free_string(result.description);
    }
}

// ── Internal helpers ─────────────────────────────────────────────────────────

fn cstring_or_null(s: Option<String>) -> *mut c_char {
    s.and_then(|v| CString::new(v).ok())
        .map(|cs| cs.into_raw())
        .unwrap_or(std::ptr::null_mut())
}
