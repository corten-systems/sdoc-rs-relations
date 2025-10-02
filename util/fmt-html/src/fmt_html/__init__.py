from __future__ import annotations

import argparse
import json
from dataclasses import dataclass
from pathlib import Path
from typing import Any, List

from pygments import highlight
from pygments.formatters import HtmlFormatter
from pygments.lexers import RustLexer


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
    lexer = RustLexer()
    formatter = HtmlFormatter(
        linenos="table",  # table with a separate line number column
        lineanchors="L",  # id="L-<line>" on each line number
        anchorlinenos=True,
        noclasses=False,  # emit CSS classes; we'll inject CSS styles
    )
    highlighted = highlight(code, lexer, formatter)
    styles = formatter.get_style_defs('.highlight')
    return highlighted, styles


def build_html(code_html: str, style_css: str, relations: List[Relation], title: str) -> str:
    # Build table rows
    rows = []
    for r in relations:
        rows.append(
            f"<tr class=\"rel-row\" data-start=\"{r.span.start_line}\" data-end=\"{r.span.end_line}\" data-id=\"{r.relation}\">"
            f"<td class=\"mono\">{r.relation}</td>"
            f"<td class=\"mono\">{r.scope}</td>"
            f"<td class=\"mono right\">{r.span.start_line}:{r.span.start_col}</td>"
            f"<td class=\"mono right\">{r.span.end_line}:{r.span.end_col}</td>"
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
    /* Pygments styles */
    {style_css}

    html, body {{ height: 100%; margin: 0; padding: 0; }}
    body {{ font-family: system-ui, -apple-system, Segoe UI, Roboto, Helvetica, Arial, \"Apple Color Emoji\", \"Segoe UI Emoji\"; }}
    .container {{ display: flex; height: 100vh; overflow: hidden; }}
    .pane {{ overflow: auto; }}
    .left {{ border-right: 1px solid #ddd; flex: 0 0 33.333%; max-width: 33.333%; font-size: 12px; }}
    .right {{ flex: 1 1 66.666%; }}
    .relations {{ width: 100%; border-collapse: collapse; table-layout: fixed; font-size: 12px; }}
    .relations th, .relations td {{ border-bottom: 1px solid #eee; padding: 4px 6px; white-space: nowrap; overflow: hidden; text-overflow: ellipsis; }}
    .relations thead th {{ position: sticky; top: 0; background: #f8f8f8; z-index: 1; }}
    .mono {{ font-family: ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, \"Liberation Mono\", \"Courier New\", monospace; }}
    .right {{ text-align: right; }}
    .rel-row:hover {{ background: #f0f7ff; cursor: pointer; }}

    /* Highlight selected code rows */
    .hl {{ background: #fff3bf !important; }}

    /* Make the pygments table span full width */
    .highlighttable {{ width: 100%; border-collapse: collapse; }}
    .highlighttable td {{ vertical-align: top; }}
    .highlighttable .linenos {{ user-select: none; background: #f8f8f8; color: #999; }}
    .highlighttable pre {{ margin: 0; }}
    /* Ensure code text is left-aligned and not justified */
    .highlighttable td.code, .highlighttable .code, .highlighttable pre {{ text-align: left !important; }}

    /* Make each table row highlightable by adding class to its tr */
    .highlighttable tr.hl > td {{ background: #fff3bf; }}

    /* Code pane padding */
    #code-pane {{ padding: 8px; }}
  </style>
</head>
<body>
  <div class=\"container\">
    <div class=\"pane left\" id=\"left-pane\">
      {table_html}
    </div>
    <div class=\"pane right\" id=\"code-pane\">
      {code_html}
    </div>
  </div>
  <script>
    (function() {{
      const codePane = document.getElementById('code-pane');
      function trForLine(n) {{
        const a = document.getElementById(`L-${{n}}`);
        if (!a) return null;
        // anchor is usually inside the left linenos cell's <pre>, then up to td -> tr
        let el = a;
        while (el && el.tagName !== 'TR') el = el.parentElement;
        return el;
      }}
      function clearHighlights() {{
        document.querySelectorAll('.highlighttable tr.hl').forEach(tr => tr.classList.remove('hl'));
      }}
      function highlightRange(start, end) {{
        clearHighlights();
        for (let n = start; n <= end; n++) {{
          const tr = trForLine(n);
          if (tr) tr.classList.add('hl');
        }}
        const first = trForLine(start);
        if (first) first.scrollIntoView({{block: 'center'}});
      }}
      document.querySelectorAll('.rel-row').forEach(row => {{
        row.addEventListener('click', () => {{
          const start = parseInt(row.dataset.start, 10);
          const end = parseInt(row.dataset.end, 10);
          highlightRange(start, end);
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
      if (rng) highlightRange(rng[0], rng[1]);
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

    title = f"{rust_path.name} — relations"
    html = build_html(code_html, style_css, relations, title)
    print(html)
