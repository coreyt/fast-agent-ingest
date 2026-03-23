// Package fastagentingest provides Go bindings for fast-agent-ingest.
//
// # Build requirements
//
// This package uses CGO to call the Rust-compiled C ABI.
// You MUST have CGO_ENABLED=1 and a C toolchain available.
//
// Cross-compilation limitations:
//   - CGO cross-compilation requires a cross-C-compiler for the target.
//   - For CGO_ENABLED=0 users, a WASM-based alternative is planned.
//
// Build the native library first:
//
//	cd ../../ && cargo build --package fast-agent-ingest-c-ffi --release
//
// Then run Go tests:
//
//	CGO_ENABLED=1 go test ./...
package fastagentingest

/*
#cgo CFLAGS:  -I${SRCDIR}/../../include
#cgo LDFLAGS: -L${SRCDIR}/../../target/release -lfast_agent_ingest_c_ffi
#cgo linux  LDFLAGS: -Wl,-rpath,${SRCDIR}/../../target/release -ldl -lpthread -lm
#cgo darwin LDFLAGS: -Wl,-rpath,${SRCDIR}/../../target/release

#include "fast_agent_ingest.h"
#include <stdlib.h>
*/
import "C"
import (
	"runtime"
	"unsafe"
)

// Options controls HTML → Markdown conversion.
type Options struct {
	// ExtractMainContent runs readability-style main-content extraction.
	// Default: true
	ExtractMainContent bool
	// IncludeImages emits Markdown image syntax. Default: true
	IncludeImages bool
	// IncludeLinks emits Markdown link syntax. Default: true
	IncludeLinks bool
}

// DefaultOptions returns the recommended default options.
func DefaultOptions() Options {
	return Options{
		ExtractMainContent: true,
		IncludeImages:      true,
		IncludeLinks:       true,
	}
}

// Result is the output of a conversion operation.
type Result struct {
	Markdown    string
	Title       string // empty if not found
	Description string // empty if not found
}

// Convert converts an HTML string to Markdown.
func Convert(html string, opts Options) Result {
	cHTML := C.CString(html)
	defer C.free(unsafe.Pointer(cHTML))

	cOpts := C.FaiOptions{
		extract_main_content: boolToU8(opts.ExtractMainContent),
		include_images:       boolToU8(opts.IncludeImages),
		include_links:        boolToU8(opts.IncludeLinks),
	}

	raw := C.fai_convert(cHTML, &cOpts)

	r := Result{
		Markdown:    cStringToGo(raw.markdown),
		Title:       cStringToGo(raw.title),
		Description: cStringToGo(raw.description),
	}

	// Register a finaliser-free eager free: the result struct owns the strings.
	C.fai_free_result(raw)
	runtime.KeepAlive(raw)

	return r
}

// ConvertDefault converts HTML with default options.
func ConvertDefault(html string) Result {
	return Convert(html, DefaultOptions())
}

// ── helpers ───────────────────────────────────────────────────────────────────

func boolToU8(b bool) C.uint8_t {
	if b {
		return 1
	}
	return 0
}

func cStringToGo(p *C.char) string {
	if p == nil {
		return ""
	}
	return C.GoString(p)
}
