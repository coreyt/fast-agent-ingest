default:
    @just --list

# ── Rust ──────────────────────────────────────────────────────────────────────

build:
    cargo build --workspace

build-release:
    cargo build --workspace --release

test:
    cargo test --workspace

lint:
    cargo clippy --workspace -- -D warnings
    cargo fmt --all -- --check

fmt:
    cargo fmt --all

# ── C headers (requires cbindgen) ─────────────────────────────────────────────

generate-headers:
    cd bindings/c-ffi && cbindgen --config cbindgen.toml --output ../../include/fast_agent_ingest.h

# ── Python (requires maturin) ─────────────────────────────────────────────────

build-python:
    cd bindings/python && maturin develop --release

test-python:
    cd tests/conformance/python && python -m pytest -v

# ── Node.js (requires @napi-rs/cli) ───────────────────────────────────────────

build-nodejs:
    cd bindings/nodejs && napi build --platform --release

test-nodejs:
    cd tests/conformance/nodejs && node --test

# ── Browser / WASM (requires wasm-pack) ───────────────────────────────────────

build-wasm:
    cd bindings/browser && wasm-pack build --target web --release

# ── Go ────────────────────────────────────────────────────────────────────────

test-go:
    cd bindings/go && CGO_ENABLED=1 go test ./...

# ── C# ────────────────────────────────────────────────────────────────────────

build-csharp:
    cd bindings/csharp && dotnet build -c Release

test-csharp:
    cd bindings/csharp && dotnet test -c Release

# ── C++ ───────────────────────────────────────────────────────────────────────

build-cpp:
    cmake -S bindings/cpp -B bindings/cpp/build -DCMAKE_BUILD_TYPE=Release
    cmake --build bindings/cpp/build

test-cpp:
    cd bindings/cpp/build && ctest --output-on-failure

# ── Conformance (all languages) ───────────────────────────────────────────────

test-conformance: test test-python test-nodejs test-go test-csharp test-cpp
    @echo "All conformance tests passed."

# ── Benchmarks ────────────────────────────────────────────────────────────────

bench:
    cargo bench --workspace
