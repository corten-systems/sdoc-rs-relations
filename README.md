# sdoc-rs-relations

[![Crates.io](https://img.shields.io/crates/v/sdoc-rs-relations)](https://crates.io/crates/sdoc-rs-relations)
[![Crates.io](https://img.shields.io/crates/d/sdoc-rs-relations)](https://crates.io/crates/sdoc-rs-relations)
[![License: AGPL v3](https://img.shields.io/badge/License-AGPL_v3-orange.svg)](https://www.gnu.org/licenses/agpl-3.0)
![Release Workflow](https://img.shields.io/github/actions/workflow/status/adfernandes/sdoc-rs-relations/release.yml)

Parse Rust source trees for [StrictDoc](https://strictdoc.readthedocs.io/) `@relation` markers to yield JSON source code spans.

> **Side note:** _If you're looking for the `sdoc-rs-relation-html` Python package to demonstrate what the `html` output could look like, see the [`demo/sdoc-rs-relation-html`](demo/sdoc-rs-relation-html) subdirectory!_

This crate is intended to support traceability between `StrictDoc` requirements and Rust source code. Specifically, it helps support linking `SDoc`-style `@relation` markers in Rust source code back to requirements, as currently (experimentally) supported for [`C`, `C++`, and Python](https://strictdoc.readthedocs.io/en/stable/stable/docs/strictdoc_01_user_guide.html#SECTION-UG-Parsing-SDoc-source-nodes).

## Installation

Download a binary from the [Releases](https://github.com/corten-systems/sdoc-rs-relations/releases) page (**TODO**), or install from source using `cargo`:

```bash
cargo install sdoc-rs-relations # TODO Not published to crates.io yet! 
```

## Usage

```bash
$ sdoc-rs-relations --help

Parse Rust source trees for StrictDoc `@relation` markers to yield JSON source code spans.

Usage: sdoc-rs-relations [OPTIONS] [PATHS]...

Arguments:
  [PATHS]...  List of files and directories to search for Rust files

Options:
  -p, --prefix <PREFIX>  Prefix path to remove from each filename entry [default: .]
  -o, --output <FILE>    Output file (use '-' or omit for output to stdout) [default: -]
  -h, --help             Print help
  -V, --version          Print version

```

## Output

Given a sample Rust file called `relations.rs` that looks like this:
```rust
// ...

/// Type alias with @relation(XH5mhH0)
/// Words: temporal flux capacitor
pub type CustomResult<T> = Result<T, Box<dyn std::error::Error>>;

/// Struct with @relation(OiEBsG0)
/// Description: polymorphic data structure
pub struct Container {
    /// Field doc with @relation(8nRE5KG)
    /// Random: ethereal quantum state
    pub name: String,

    /// Another field with @relation(oByPxhWz)
    /// Text: cascading resonance field
    value: i32,
}

// ...
```
the output `JSON` looks like this:
```json
[
  {
    "file": "relations.rs",
    "hash": {
      "sha256": "a08599f900610ab706eeab0f3f272719548364bd95dbd3cef2e10584eae0111f"
    },
    "relations": [
      {
        "relation": "S5x7ZEWE",
        "scope": "File",
        "span": {
          "start": {
            "line": 1,
            "column": 0
          },
          "end": {
            "line": 275,
            "column": 1
          }
        }
      },
      {
        "relation": "R3m2aYp",
        "scope": "ItemConst",
        "span": {
          "start": {
            "line": 4,
            "column": 0
          },
          "end": {
            "line": 6,
            "column": 33
          }
        }
      },
      ...
```

Full input and output examples can be found in the [`demo/sdoc-rs-relation-html/examples`](demo/sdoc-rs-relation-html/examples) directory. 

As per Rust convention, **lines are counted from one, while columns are counted from zero**.

This JSON output is intended to be consumed by other tools that need to map source code back to requirements. An example of such is the [`sdoc-rs-relation-html`](demo/sdoc-rs-relation-html) tool which can be used to generate an interactive HTML of what `@relation` maps to what block of code.

For sample output, see:
 * [`relations.rs`](https://corten.systems/demo/sdoc-rs-relations/relations.html)
 * [`relations1.rs`](https://corten.systems/demo/sdoc-rs-relations/relations1.html)
 * [`relations2.rs`](https://corten.systems/demo/sdoc-rs-relations/relations2.html)
 * [`relations3.rs`](https://corten.systems/demo/sdoc-rs-relations/relations3.html)

## Design

Unlike many languages, Rust has two type of comments:
* "normal" comments that are stripped out of the source code
* "doc comments" that are **preserved** in the source code as `#[doc]` attributes.

This is quite a bit different from both Python and Java, where documentation comments are still just normal comments that do not affect code generation. Although Rust doc-comments _also_ do **not** affect code generation, they **are** attached to specific nodes in the AST. There are some details here, such as "inner" and "outer" doc comments (and attributes), but that's the gist of it.

For a few more details, see the [Rust documentation](https://doc.rust-lang.org/reference/comments.html) and the documentation for [`syn::Attribute`](https://docs.rs/syn/latest/syn/struct.Attribute.html).

## Rationale

In Rust, AST nodes that have attributes can be viewed as the "smallest" unit of code that has independent meaning. For example, `#[cfg(...)`] attributes can be used for conditional compilation.

For more detail on Rust attributes, see the [Rust Reference](https://doc.rust-lang.org/reference/attributes.html).

## Examples

Example `HTML` output is in the [`demo/sdoc-rs-relation-html/examples/html`](demo/sdoc-rs-relation-html/examples/html) directory. If you're viewing the repository in a web browser, you will need to **download** or **clone** those files to your local computer to view them. 

Those demo HTML files were generated by running the [`demo/sdoc-rs-relation-html`](demo/sdoc-rs-relation-html) utility.

## Upcoming Changes

The Python package `sdoc-rs-relation-html` will be deprecated in favor of a new Rust binary, of the same name, in this crate. The new binary will be a re-implementation of the Python CLI too, in Rust.

The reason for the rewrite is that the Python package uses [`Pygments`](https://pygments.org) to highlight the source code, using a [custom lexer](https://pygments.org/docs/lexerdevelopment/) for each language.

The new binary uses [`autumnus`](https://github.com/leandrocp/autumnus) for syntax highlighting, which uses [`tree-sitter`](https://tree-sitter.github.io/tree-sitter/) for parsing.

The ongoing rewrite work is on the [`html` branch](https://github.com/corten-systems/sdoc-rs-relations/tree/html), if you want to check it out.

