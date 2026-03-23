"""Conformance tests: run every HTML fixture and compare to expected Markdown.

Run:
    cd bindings/python && maturin develop --release
    cd tests/conformance/python && python -m pytest -v
"""
from pathlib import Path
import pytest
import fast_agent_ingest

FIXTURES = Path(__file__).parent.parent.parent / "fixtures"
INPUTS   = FIXTURES / "inputs"
EXPECTED = FIXTURES / "expected"


def fixture_names():
    return [p.stem for p in sorted(INPUTS.glob("*.html"))]


@pytest.mark.parametrize("name", fixture_names())
def test_conformance(name: str) -> None:
    html     = (INPUTS   / f"{name}.html").read_text(encoding="utf-8")
    expected = (EXPECTED / f"{name}.md").read_text(encoding="utf-8").strip()

    result = fast_agent_ingest.convert(html)
    assert result.markdown.strip() == expected, (
        f"Markdown mismatch for {name}\n"
        f"--- expected ---\n{expected[:500]}\n"
        f"--- got ---\n{result.markdown[:500]}"
    )
