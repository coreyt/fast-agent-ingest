package fastagentingest_test

import (
	"os"
	"path/filepath"
	"strings"
	"testing"

	. "github.com/coreyt/fast-agent-ingest/bindings/go"
)

func TestConvertSimpleArticle(t *testing.T) {
	html := `<html><body><h1>Hello</h1><p>World</p></body></html>`
	result := Convert(html, Options{
		ExtractMainContent: false,
		IncludeImages:      true,
		IncludeLinks:       true,
	})

	if !strings.Contains(result.Markdown, "# Hello") {
		t.Errorf("expected '# Hello' in markdown, got:\n%s", result.Markdown)
	}
	if !strings.Contains(result.Markdown, "World") {
		t.Errorf("expected 'World' in markdown, got:\n%s", result.Markdown)
	}
}

// TestConformance runs every fixture in tests/fixtures and compares output
// to the expected Markdown.
func TestConformance(t *testing.T) {
	fixturesDir := filepath.Join("..", "..", "tests", "fixtures")

	inputDir := filepath.Join(fixturesDir, "inputs")
	expectedDir := filepath.Join(fixturesDir, "expected")

	entries, err := os.ReadDir(inputDir)
	if err != nil {
		t.Fatalf("cannot read fixtures: %v", err)
	}

	for _, e := range entries {
		if !strings.HasSuffix(e.Name(), ".html") {
			continue
		}
		name := strings.TrimSuffix(e.Name(), ".html")
		t.Run(name, func(t *testing.T) {
			htmlBytes, err := os.ReadFile(filepath.Join(inputDir, e.Name()))
			if err != nil {
				t.Fatalf("read input: %v", err)
			}
			expectedBytes, err := os.ReadFile(filepath.Join(expectedDir, name+".md"))
			if err != nil {
				t.Fatalf("read expected: %v", err)
			}

			result := ConvertDefault(string(htmlBytes))
			got := strings.TrimSpace(result.Markdown)
			want := strings.TrimSpace(string(expectedBytes))

			if got != want {
				t.Errorf("markdown mismatch for %s\n--- want ---\n%s\n--- got ---\n%s", name, want, got)
			}
		})
	}
}
