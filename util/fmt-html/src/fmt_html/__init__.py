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
    rows = []
    for r in relations:
        rows.append(
            f"<tr class=\"rel-row\" data-start=\"{r.span.start_line}\" data-end=\"{r.span.end_line}\" data-start-col=\"{r.span.start_col}\" data-end-col=\"{r.span.end_col}\" data-id=\"{r.relation}\">"
            f"<td>{r.relation}</td>"
            f"<td>{r.scope}</td>"
            f"<td class=\"center\">{r.span.start_line}:{r.span.start_col}</td>"
            f"<td class=\"center\">{r.span.end_line}:{r.span.end_col}</td>"
            f"</tr>"
        )
    table_html = (
        "<table class=\"relations\">"
        "<thead><tr><th>relation</th><th>scope</th><th>start</th><th>end</th></tr></thead>"
        f"<tbody>{''.join(rows)}</tbody>"
        "</table>"
    )
    html = f"""
<!doctype html>
<html lang=\"en\">
<head>
    <meta charset=\"utf-8\" />
    <meta name=\"viewport\" content=\"width=device-width, initial-scale=1\" />
    <title>{title}</title>
    <style>
  
        {style_css}
      
        ::selection {{
            background: #cceeff;
        }}
      
        #code-pane {{
            padding: 0;
        }}
        
        .container {{
            display: flex;
            height: calc(100vh - 44px);
            overflow: hidden;
        }}

        .hash {{
            color: #505050;
            font-size: 12px;
            padding: 0 0 0 24px;
        }}
                        
        .highlight-table {{
            border-collapse: collapse;
            width: 100%;
        }}
        
        .highlight-table .linenos {{
            background: #f8f8f8;
            color: #909090;
        }}
        
        .highlight-table pre {{
            margin: 0;
        }}
        
        .highlight-table td {{
            vertical-align: top;
        }}
        
        .highlight-table td.code,
        .highlight-table .code,
        .highlight-table pre {{
            text-align: left !important;
        }}
        
        .left {{
            border-right: 1px solid #d0d0d0;
            flex: 0 0 30%;
            font-size: 12px;
            max-width: 30%;
        }}
        
        .pane {{
            overflow: auto;
        }}
        
        .pane .code-wrap {{
            padding: 8px;
        }}
        
        .pane-header {{
            background: #e0e0e0;
            border-bottom: 1px solid #e5e5e5;
            font-size: 16px;
            font-weight: 600;
            padding: 12px 6px 12px 6px;
            position: sticky;
            text-align: left;
            text-decoration: underline;
            top: 0;
            z-index: 2;
        }}
        
        .rel-row:hover {{
            background: #fff7b3;
            cursor: pointer;
        }}
        
        .relations {{
            border-collapse: collapse;
            font-size: 12px;
            table-layout: fixed;
            width: 100%;
        }}
        
        .relations th,
        .relations td {{
            border-bottom: 1px solid #e0e0e0;
            overflow: hidden;
            padding: 4px 6px;
            text-overflow: ellipsis;
            white-space: nowrap;
        }}
        
        .relations thead th {{
            background: #f0f0f0;
            position: sticky;
            top: 44px;
            z-index: 1;
        }}
        
        .relations thead th:nth-child(1),
        .relations thead th:nth-child(2) {{
            text-align: left;
        }}
        
        .relations thead th:nth-child(3),
        .relations thead th:nth-child(4),
        .center {{
            text-align: center;
        }}
        
        .right {{
            flex: 1 1 70%;
        }}
                
        .title {{
            font-size: 16px;
            font-weight: 600;
            text-decoration: underline;
        }}
        
        .top-bar {{
            align-items: baseline;
            background: #ffffff;
            border-bottom: 1px solid #e5e5e5;
            display: flex;
            padding: 0 6px 8px 6px;
            position: sticky;
            top: 0;
            z-index: 5;
        }}
        
        a {{
            color: #b0b0b0;
            font-size: 12px;
            text-decoration: none;
        }}
        
        a:hover,
        a:focus,
        a:active {{
            color: #404040;
        }}
        
        a:visited {{
            color: #a0a0a0;
        }}
        
        body {{
            font-family: monospace;
        }}
        
        html,
        body {{
            height: 100%;
            margin: 3px 6px 3px 6px;
            padding: 3px 6px 3px 6px;
        }}
        
    </style>
</head>
<body>
    <div class=\"top-bar\">
        <span class=\"title\">{filename}</span>
        <span class=\"hash\">SHA256: {sha256_hex}</span>
    </div>
    <div class=\"container\">
    <div class=\"pane left\" id=\"left-pane\">
        <div class=\"pane-header\">Relations</div>
            {table_html}
        </div>
        <div class=\"pane right\" id=\"code-pane\">
        <div class=\"pane-header\">Source</div>
        <div class=\"code-wrap\">
            {code_html}
        </div>
    </div>
    <script>
        (function() {{

            function lineElement(n) {{
                const span = document.getElementById(`LC-${{n}}`);
                if (span) return span;
                const a = document.getElementById(`L-${{n}}`);
                if (!a) return null;
                let el = a;
                while (el && el.tagName !== 'TR') el = el.parentElement;
                return el;
            }}

            function clearSelection() {{
                const sel = window.getSelection && window.getSelection();
                if (sel && sel.removeAllRanges) sel.removeAllRanges();
            }}

            function textPositionInLine(spanEl, col) {{
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
                }} catch (_) {{ }}
                return null;
            }}

            function endOfLine(spanEl) {{
                const walker = document.createTreeWalker(spanEl, NodeFilter.SHOW_TEXT, null);
                let lastText = null;
                while (walker.nextNode()) lastText = walker.currentNode;
                if (lastText) return {{ node: lastText, offset: lastText.textContent.length }};
                return {{ node: spanEl, offset: spanEl.childNodes.length }};
            }}

            function selectRange(sLine, sCol, eLine, eCol) {{

                clearSelection();

                let startLine = Math.min(sLine, eLine);
                let endLine = Math.max(sLine, eLine);
                let startCol = (startLine === sLine) ? sCol : eCol;
                let endCol = (endLine === eLine) ? eCol : sCol;

                const startSpan = lineElement(startLine);
                const endSpan = lineElement(endLine);
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

                startSpan.scrollIntoView({{
                    behavior: 'smooth',
                    block: 'center',
                    inline: 'center'
                }});

            }}

            function scrollToAnchor() {{
                if (!location.hash) return;
                const match = location.hash.slice(1).match(/^L-(\\d+)$/);
                if (!match) return;
                const line = parseInt(match[1], 10);
                const span = lineElement(line);
                if (!span) return null;
                span.scrollIntoView({{
                    behavior: 'smooth',
                    block: 'center',
                    inline: 'center'
                }});    
            }}

            document.addEventListener('DOMContentLoaded', function() {{
            
                document.querySelectorAll('.rel-row').forEach(row => {{
                    row.addEventListener('click', () => {{
                        const start = parseInt(row.dataset.start, 10);
                        const end = parseInt(row.dataset.end, 10);
                        const startCol = parseInt(row.dataset.startCol, 10);
                        const endCol = parseInt(row.dataset.endCol, 10);
                        selectRange(start, startCol, end, endCol);
                    }});
                }});
    
                scrollToAnchor();
                
            }});

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

    title = f"{rust_path.name} relations"
    html = build_html(code_html, style_css, relations, title, rust_path.name, sha256_hex)
    print(html)
