#!/usr/bin/env python3
"""
fetch_to_markdown.py — fetch one or more URLs and print clean Markdown.

Usage:
    python tools/fetch_to_markdown.py <url> [<url> ...]
    python tools/fetch_to_markdown.py --help

Examples:
    python tools/fetch_to_markdown.py https://en.wikipedia.org/wiki/Markdown
    python tools/fetch_to_markdown.py https://example.com https://httpbin.org/html
    python tools/fetch_to_markdown.py https://example.com --out /tmp/out.md
    python tools/fetch_to_markdown.py https://example.com --no-extract   # skip content extraction
"""

import argparse
import sys
import time
from dataclasses import dataclass
from typing import Optional

import requests
import fast_agent_ingest


@dataclass
class FetchResult:
    url: str
    html_bytes: int
    markdown: str
    markdown_bytes: int
    title: Optional[str]
    description: Optional[str]
    fetch_ms: float
    convert_ms: float

    @property
    def reduction_pct(self) -> float:
        if self.html_bytes == 0:
            return 0.0
        return (1 - self.markdown_bytes / self.html_bytes) * 100


def fetch_and_convert(
    url: str,
    opts: fast_agent_ingest.ConversionOptions,
    *,
    verify_ssl: bool = True,
) -> FetchResult:
    # ── Fetch ────────────────────────────────────────────────────────────────
    t0 = time.perf_counter()
    resp = requests.get(
        url,
        timeout=15,
        verify=verify_ssl,
        headers={"User-Agent": "fast-agent-ingest/0.1 (test harness)"},
    )
    resp.raise_for_status()
    fetch_ms = (time.perf_counter() - t0) * 1000

    html = resp.text
    html_bytes = len(resp.content)

    # ── Convert ──────────────────────────────────────────────────────────────
    t1 = time.perf_counter()
    result = fast_agent_ingest.convert(html, opts)
    convert_ms = (time.perf_counter() - t1) * 1000

    return FetchResult(
        url=url,
        html_bytes=html_bytes,
        markdown=result.markdown,
        markdown_bytes=len(result.markdown.encode()),
        title=result.title,
        description=result.description,
        fetch_ms=fetch_ms,
        convert_ms=convert_ms,
    )


def print_result(r: FetchResult, *, verbose: bool, out_file=None) -> None:
    sep = "─" * 72
    header = (
        f"\n{sep}\n"
        f"URL:         {r.url}\n"
        f"Title:       {r.title or '(none)'}\n"
    )
    if r.description:
        header += f"Description: {r.description[:120]}\n"
    header += (
        f"HTML:        {r.html_bytes:,} bytes\n"
        f"Markdown:    {r.markdown_bytes:,} bytes  "
        f"({r.reduction_pct:.0f}% smaller)\n"
        f"Fetch:       {r.fetch_ms:.0f} ms   "
        f"Convert: {r.convert_ms:.1f} ms\n"
        f"{sep}\n"
    )

    print(header, file=sys.stderr)

    if out_file:
        out_file.write(r.markdown)
    else:
        print(r.markdown)


def main() -> None:
    parser = argparse.ArgumentParser(
        description="Fetch URLs and print clean Markdown via fast-agent-ingest."
    )
    parser.add_argument("urls", nargs="+", metavar="URL")
    parser.add_argument(
        "--no-extract",
        action="store_true",
        help="Disable readability-style main-content extraction",
    )
    parser.add_argument(
        "--no-images",
        action="store_true",
        help="Strip image references from Markdown output",
    )
    parser.add_argument(
        "--no-links",
        action="store_true",
        help="Strip hyperlinks from Markdown output",
    )
    parser.add_argument(
        "--no-verify",
        action="store_true",
        help="Disable SSL certificate verification (useful on restricted networks)",
    )
    parser.add_argument(
        "--out", "-o",
        metavar="FILE",
        help="Write Markdown to FILE instead of stdout "
             "(for multiple URLs, appends with separators)",
    )
    args = parser.parse_args()

    opts = fast_agent_ingest.ConversionOptions(
        extract_main_content=not args.no_extract,
        include_images=not args.no_images,
        include_links=not args.no_links,
    )

    out_file = open(args.out, "w", encoding="utf-8") if args.out else None

    try:
        errors = 0
        for url in args.urls:
            try:
                result = fetch_and_convert(url, opts, verify_ssl=not args.no_verify)
                print_result(result, verbose=True, out_file=out_file)
                if out_file and len(args.urls) > 1:
                    out_file.write(f"\n\n---\n\n")
            except requests.HTTPError as e:
                print(f"HTTP error for {url}: {e}", file=sys.stderr)
                errors += 1
            except requests.RequestException as e:
                print(f"Fetch error for {url}: {e}", file=sys.stderr)
                errors += 1
    finally:
        if out_file:
            out_file.close()

    sys.exit(1 if errors else 0)


if __name__ == "__main__":
    main()
