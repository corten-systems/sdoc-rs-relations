use anyhow::{bail, Result};

use autumnus::{formatter::Formatter, languages::Language, HtmlLinkedBuilder};

use quick_xml::events::{BytesText, Event};
use quick_xml::reader::Reader;
use quick_xml::writer::Writer;

use std::io::Cursor;

/* TODO Add line numbers!

var range = document.createRange();
var sel = window.getSelection();

var span_start = document.querySelector('pre.athl.code span.location[data-line="3"][data-column="4"]');
var span_end =   document.querySelector('pre.athl.code span.location[data-line="3"][data-column="12"]');

sel.removeAllRanges();
range.setStart(span_start, 0);
range.setEnd(span_end, 0);
sel.addRange(range);

 */

pub fn html_from(title: &str, input: &str) -> Result<String> {
    let formatter = HtmlLinkedBuilder::new()
        .source(input)
        .lang(Language::Rust)
        .pre_class(Some("code"))
        .build()?;

    let mut output = Vec::new();
    formatter.format(&mut output)?;
    let code = String::from_utf8(output)?;
    let body = add_line_column_annotations(&code)?;

    let style = include_str!("html/style.css");
    let theme = include_str!("html/theme/github_light_colorblind.css");
    let script = include_str!("html/script.js");
    let html = format!(include_str!("html/file.html"), title, theme, style, script, body);

    Ok(html)
}

#[test]
fn test_html_from() -> Result<()> {
    let title = "test_html_from";
    let input = include_str!("html.rs");
    let output = html_from(title, input)?;
    println!("{}", output);
    Ok(())
}

/// Processes an HTML string to insert line and column tracking spans into its text content.
///
/// This function parses the input as HTML and iterates through its nodes. It preserves
/// all HTML tags, comments, and processing instructions as they are, but it modifies
/// the text content. Within the text nodes, it inserts `span` tags to mark the line
/// and column of every character.
///
/// # Arguments
///
/// * `input` - A string slice containing the HTML to be processed.
///
/// # Returns
///
/// A `String` containing the processed HTML with spans inserted only into text content.
/// If the input is not well-formed HTML, it will panic.
///
/// # Details
///
/// - It uses the `quick-xml` crate to parse the HTML.
/// - It only annotates content within `Event::Text`. All other events (tags, etc.) are untouched.
/// - A `<span id="L${line}C0" />` tag is inserted at the beginning of any text on a new line.
/// - A `<span id="L${line}C${column}" />` tag is inserted after every character in a text node.
/// - Line and column counts are maintained correctly across the entire document.
///
fn add_line_column_annotations(input: &str) -> Result<String> {
    if input.contains('\t') {
        bail!("tab characters are not supported");
    }

    let input = input.to_string().replace("\r\n", "\n");
    let mut reader = Reader::from_str(&input);
    reader.config_mut().trim_text(false);

    let mut line = 1;
    let mut column = 0;
    let mut writer = Writer::new(Cursor::new(Vec::new()));

    loop {
        match reader.read_event() {
            Ok(Event::Eof) => break,

            Ok(Event::Text(text)) => {
                let text = text.html_content()?;
                let mut result = String::new();
                if line == 1 && column == 0 {
                    result.push_str(&format!("<span class=\"location\" data-line=\"{}\" data-column=\"{}\"></span>", line, column));
                }
                for ch in text.chars() {
                    if ch == '\n' {
                        line += 1;
                        column = 0;
                    }
                    result.push(ch);
                    result.push_str(&format!("<span class=\"location\" data-line=\"{}\" data-column=\"{}\"></span>", line, column));
                    column += 1;
                }
                let event = BytesText::from_escaped(result);
                writer.write_event(Event::Text(event))?;
            }

            Ok(Event::Start(event)) => {
                match event.name().as_ref() {
                    b"code" => {} // code-inside-pre is not valid html5
                    b"div" => {}  // div-inside-pre is not valid html5
                    _ => writer.write_event(Event::Start(event))?,
                }
            }

            Ok(Event::End(event)) => {
                match event.name().as_ref() {
                    b"code" => {} // code-inside-pre is not valid html5
                    b"div" => {}  // div-inside-pre is not valid html5
                    _ => writer.write_event(Event::End(event))?,
                }
            }

            Ok(event) => {
                writer.write_event(event)?;
            }

            Err(error) => {
                bail!("parsing failed at offset {}: {:?}", reader.buffer_position(), error)
            }
        }
    }

    Ok(writer.into_inner().into_inner().into_iter().map(|b| b as char).collect::<String>())
}
