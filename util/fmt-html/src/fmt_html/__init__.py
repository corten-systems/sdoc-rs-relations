from __future__ import annotations

import argparse
import hashlib
import json

from dataclasses import dataclass
from pathlib import Path
from typing import List

from pygments import formatters, highlight, lexers


@dataclass
class Span:
    start_line: int
    start_col: int
    end_line: int
    end_col: int


@dataclass
class Relation:
    relation: str
    scope: str
    span: Span


def load_relations(json_path: Path) -> List[Relation]:
    data = json.loads(json_path.read_text())
    # File format is an array; take all objects' relations
    relations: List[Relation] = []
    for obj in data:
        for rel in obj.get("relations", []):
            span = rel.get("span", {})
            s_obj = span.get("start", {})
            e_obj = span.get("end", {})
            s_line = s_obj.get("line")
            e_line = e_obj.get("line")
            if s_line is None or e_line is None:
                continue
            s_col = s_obj.get("column", 0)
            e_col = e_obj.get("column", 0)
            relations.append(
                Relation(
                    relation=str(rel.get("relation", "")),
                    scope=str(rel.get("scope", "")),
                    span=Span(int(s_line), int(s_col), int(e_line), int(e_col)),
                )
            )
    # Stable ordering by start line then end line then relation id
    relations.sort(key=lambda r: (r.span.start_line, r.span.end_line, r.relation))
    return relations


def render_code_html(code: str) -> tuple[str, str]:
    lexer = lexers.RustLexer()
    formatter = formatters.HtmlFormatter(
        linenos="table",  # table with a separate line number column
        lineanchors="L",  # id="L-<line>" on each line number (left column)
        linespans="LC",   # wrap each code line in <span id="LC-<line>">...</span>
        noclasses=False,  # emit CSS classes; we'll inject CSS styles
        anchorlinenos=True,
    )
    highlighted = highlight(code, lexer, formatter)
    styles = formatter.get_style_defs('.highlight')
    return highlighted, styles


def build_html(code_html: str, style_css: str, relations: List[Relation], title: str, filename: str, sha256_hex: str) -> str:
    # Build table rows
    rows = []
    for r in relations:
        rows.append(
            f"<tr class=\"rel-row\" data-start=\"{r.span.start_line}\" data-end=\"{r.span.end_line}\" data-start-col=\"{r.span.start_col}\" data-end-col=\"{r.span.end_col}\" data-id=\"{r.relation}\">"
            f"<td class=\"mono\">{r.relation}</td>"
            f"<td class=\"mono\">{r.scope}</td>"
            f"<td class=\"mono center\">{r.span.start_line}:{r.span.start_col}</td>"
            f"<td class=\"mono center\">{r.span.end_line}:{r.span.end_col}</td>"
            f"</tr>"
        )
    table_html = (
        "<table class=\"relations\">"
        "<thead><tr><th>relation</th><th>scope</th><th>start</th><th>end</th></tr></thead>"
        f"<tbody>{''.join(rows)}</tbody>"
        "</table>"
    )

    # Build full HTML with CSS and JS
    html = f"""
<!doctype html>
<html lang=\"en\">
<head>
  <meta charset=\"utf-8\" />
  <meta name=\"viewport\" content=\"width=device-width, initial-scale=1\" />
  <title>{title}</title>
  <style>
    {style_css}
    html, body {{ height: 100%; margin: 6px; padding: 6px; }}
    body {{ font-family: system-ui, -apple-system, Segoe UI, Roboto, Helvetica, Arial, \"Apple Color Emoji\", \"Segoe UI Emoji\"; }}
    .top-bar {{ position: sticky; top: 0; z-index: 5; background: #ffffff; border-bottom: 1px solid #e5e5e5; padding: 8px 12px; display: flex; gap: 16px; align-items: baseline; }}
    .top-bar .title {{ font-weight: 600; font-family: ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, \"Liberation Mono\", \"Courier New\", monospace; }}
    .top-bar .hash {{ font-family: ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, \"Liberation Mono\", \"Courier New\", monospace; color: #555; font-size: 12px; }}
    .container {{ display: flex; height: calc(100vh - 44px); overflow: hidden; }}
    .pane {{ overflow: auto; }}
    .pane-header {{ position: sticky; top: 0; z-index: 2; background: #f6f8fa; border-bottom: 1px solid #e5e5e5; padding: 6px 8px; font-weight: 600; font-size: 16px; text-align: left; }}
    .pane .code-wrap {{ padding: 8px; }}
    .left {{ border-right: 1px solid #ddd; flex: 0 0 30%; max-width: 30%; font-size: 12px; }}
    .right {{ flex: 1 1 70%; }}
    .relations {{ width: 100%; border-collapse: collapse; table-layout: fixed; font-size: 12px; }}
    .relations th, .relations td {{ border-bottom: 1px solid #eee; padding: 4px 6px; white-space: nowrap; overflow: hidden; text-overflow: ellipsis; }}
    .relations thead th {{ position: sticky; top: 0; background: #f8f8f8; z-index: 1; }}
    .relations thead th:nth-child(1), .relations thead th:nth-child(2) {{ text-align: left; }}
    .relations thead th:nth-child(3), .relations thead th:nth-child(4) {{ text-align: center; }}
    .mono {{ font-family: ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, \"Liberation Mono\", \"Courier New\", monospace; }}
    .center {{ text-align: center; }}
    .rel-row:hover {{ background: #f0f7ff; cursor: pointer; }}
    .highlight-table {{ width: 100%; border-collapse: collapse; }}
    .highlight-table td {{ vertical-align: top; }}
    .highlight-table .linenos {{ user-select: none; background: #f8f8f8; color: #999; }}
    a {{ color: #aaa; text-decoration: none; font-size: 12px; font-family: ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, "Liberation Mono", "Courier New", monospace; }}
    a:visited {{ color: #aaa; text-decoration: none; }}
    a:hover {{ color: #666; text-decoration: none; }}
    a:focus, a:active {{ color: #666; text-decoration: none; }}
    .highlight-table pre {{ margin: 0; }}
    .highlight-table td.code, .highlight-table .code, .highlight-table pre {{ text-align: left !important; }}
    [id^="LC-"] .hl {{ background: #fff3bf; display: block; }}
    #code-pane {{ padding: 0; }}
  </style>
</head>
<body>
  <div class=\"top-bar\">\n    <span class=\"title\">{filename}</span>\n    <span class=\"hash\">SHA256: {sha256_hex}</span>\n  </div>
  <div class=\"container\">
    <div class=\"pane left\" id=\"left-pane\">\n      <div class=\"pane-header\">Relations</div>
      {table_html}
    </div>
    <div class=\"pane right\" id=\"code-pane\">\n      <div class=\"pane-header\">Source</div>\n      <div class=\"code-wrap\">
      {code_html}
    </div>
  </div>
  <script>
    (function() {{
      const codePane = document.getElementById('code-pane');
      function lineEl(n) {{
        // Prefer per-line code span generated via Pygments lines-pans
        const span = document.getElementById(`LC-${{n}}`);
        if (span) return span;
        // Fallback: try to find the table row (older output formats)
        const a = document.getElementById(`L-${{n}}`);
        if (!a) return null;
        let el = a;
        while (el && el.tagName !== 'TR') el = el.parentElement;
        return el;
      }}
      function clearSelection() {{
        const sel = window.getSelection && window.getSelection();
        if (sel && sel.removeAllRanges) sel.removeAllRanges();
        // Also remove any legacy CSS highlights if present
        document.querySelectorAll('.highlight-table tr.hl').forEach(tr => tr.classList.remove('hl'));
        document.querySelectorAll('[id^="LC-"] .hl').forEach(el => el.classList.remove('hl'));
      }}
      function textPositionInLine(spanEl, col) {{
        // Map a column offset (0-based) within the line's plain text to a specific text node and offset
        // If col is undefined/null, return start of line for start positions and end for end via caller.
        try {{
          let remaining = Math.max(0, Number(col) || 0);
          const walker = document.createTreeWalker(spanEl, NodeFilter.SHOW_TEXT, null);
          let lastText = null;
          while (walker.nextNode()) {{
            const node = walker.currentNode;
            const len = node.textContent.length;
            lastText = node;
            if (remaining <= len) {{
              return {{ node, offset: remaining }};
            }}
            remaining -= len;
          }}
          if (lastText) return {{ node: lastText, offset: lastText.textContent.length }};
        }} catch (_) {{}}
        return null;
      }}
      function endOfLine(spanEl) {{
        const walker = document.createTreeWalker(spanEl, NodeFilter.SHOW_TEXT, null);
        let lastText = null;
        while (walker.nextNode()) lastText = walker.currentNode;
        if (lastText) return {{ node: lastText, offset: lastText.textContent.length }};
        // fallback to element itself
        return {{ node: spanEl, offset: spanEl.childNodes.length }};
      }}
      function selectRange(sLine, sCol, eLine, eCol) {{
        clearSelection();
        let startLine = Math.min(sLine, eLine);
        let endLine = Math.max(sLine, eLine);
        let startCol = (startLine === sLine) ? sCol : eCol;
        let endCol = (endLine === eLine) ? eCol : sCol;

        const startSpan = lineEl(startLine);
        const endSpan = lineEl(endLine);
        if (!startSpan || !endSpan) return;

        let startPos = textPositionInLine(startSpan, startCol);
        if (!startPos) startPos = {{ node: startSpan, offset: 0 }};

        let endPos = textPositionInLine(endSpan, endCol);
        if (!endPos) endPos = endOfLine(endSpan);

        const range = document.createRange();
        range.setStart(startPos.node, startPos.offset);
        range.setEnd(endPos.node, endPos.offset);

        const sel = window.getSelection && window.getSelection();
        if (sel && sel.removeAllRanges) {{
          sel.removeAllRanges();
          sel.addRange(range);
        }}
        startSpan.scrollIntoView({{ block: 'center' }});
      }}
      document.querySelectorAll('.rel-row').forEach(row => {{
        row.addEventListener('click', () => {{
          const start = parseInt(row.dataset.start, 10);
          const end = parseInt(row.dataset.end, 10);
          const startCol = parseInt(row.dataset.startCol, 10);
          const endCol = parseInt(row.dataset.endCol, 10);
          selectRange(start, startCol, end, endCol);
        }});
      }});

      // Optional: support deep-linking via hash '#L-<start>,<end>'
      function parseHash() {{
        if (!location.hash) return null;
        const m = location.hash.slice(1).match(/^L-(\\d+)(?:,(\\d+))?$/);
        if (!m) return null;
        const s = parseInt(m[1], 10);
        const e = m[2] ? parseInt(m[2], 10) : s;
        return [s, e];
      }}
      const rng = parseHash();
      if (rng) selectRange(rng[0], 0, rng[1], Number.MAX_SAFE_INTEGER);
    }})();
  </script>
</body>
</html>
"""
    return html


def main() -> None:
    parser = argparse.ArgumentParser(description="Combine Rust source and relations JSON into a two-pane HTML view")
    parser.add_argument("rust_file", type=Path, help="Path to the Rust source file")
    parser.add_argument("json_file", type=Path, help="Path to the relations JSON file")
    args = parser.parse_args()

    rust_path: Path = args.rust_file
    json_path: Path = args.json_file

    code = rust_path.read_text()
    relations = load_relations(json_path)
    code_html, style_css = render_code_html(code)

    sha256_hex = hashlib.sha256(code.encode("utf-8")).hexdigest()

    title = f"{rust_path.name} — relations"
    html = build_html(code_html, style_css, relations, title, rust_path.name, sha256_hex)
    print(html)
