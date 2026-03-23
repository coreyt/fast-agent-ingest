#include <cassert>
#include <cstring>
#include <filesystem>
#include <fstream>
#include <iostream>
#include <sstream>
#include <string>

#include "fast_agent_ingest.hpp"

namespace fs = std::filesystem;

static std::string read_file(const fs::path& p) {
    std::ifstream f(p);
    if (!f) throw std::runtime_error("Cannot open: " + p.string());
    std::ostringstream ss;
    ss << f.rdbuf();
    return ss.str();
}

static std::string trim(std::string s) {
    auto start = s.find_first_not_of(" \t\r\n");
    auto end   = s.find_last_not_of(" \t\r\n");
    if (start == std::string::npos) return "";
    return s.substr(start, end - start + 1);
}

int main() {
    fai::Converter conv;
    int failures = 0;

    // ── Smoke test ────────────────────────────────────────────────────────────
    auto r = conv.convert("<html><body><h1>Hello</h1><p>World</p></body></html>");
    assert(r.markdown.find("# Hello") != std::string::npos && "smoke test: heading");
    assert(r.markdown.find("World")   != std::string::npos && "smoke test: paragraph");
    std::cout << "[PASS] smoke test\n";

    // ── Fixture conformance ───────────────────────────────────────────────────
    fs::path fixtures("tests/fixtures");
    if (!fs::exists(fixtures)) {
        std::cerr << "Fixtures directory not found — skipping conformance tests\n";
        return 0;
    }

    for (auto& entry : fs::directory_iterator(fixtures / "inputs")) {
        if (entry.path().extension() != ".html") continue;

        auto name     = entry.path().stem().string();
        auto expected = trim(read_file(fixtures / "expected" / (name + ".md")));
        auto html     = read_file(entry.path());
        auto result   = conv.convert(html);
        auto got      = trim(result.markdown);

        if (got == expected) {
            std::cout << "[PASS] " << name << "\n";
        } else {
            std::cerr << "[FAIL] " << name << "\n"
                      << "  expected: " << expected.substr(0, 120) << "...\n"
                      << "  got:      " << got.substr(0, 120) << "...\n";
            ++failures;
        }
    }

    return failures == 0 ? 0 : 1;
}
