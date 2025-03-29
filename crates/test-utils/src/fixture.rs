//! Defines `Fixture` -- a convenient way to describe the initial state of
//! rust-analyzer database from a single string.
//!
//! Fixtures are strings containing rust source code with optional metadata.
//! A fixture without metadata is parsed into a single source file.
//! Use this to test functionality local to one file.
//!
//! Simple Example:
//!
//! ```ignore
//! r#"
//! fn main() {
//!     println!("Hello World")
//! }
//! "#
//! ```
//!
//! Metadata can be added to a fixture after a `//-` comment.
//! The basic form is specifying filenames,
//! which is also how to define multiple files in a single test fixture
//!
//! Example using two files in the same crate:
//!
//! ```ignore
//! "
//! //- /main.rs
//! mod foo;
//! fn main() {
//!     foo::bar();
//! }
//!
//! //- /foo.rs
//! pub fn bar() {}
//! "
//! ```
//!
//! Example using two crates with one file each, with one crate depending on the other:
//!
//! ```ignore
//! r#"
//! //- /main.rs crate:a deps:b
//! fn main() {
//!     b::foo();
//! }
//! //- /lib.rs crate:b
//! pub fn b() {
//!     println!("Hello World")
//! }
//! "#
//! ```
//!
//! Metadata allows specifying all settings and variables
//! that are available in a real rust project. See [`Fixture`]
//! for the syntax.
//!
//! Example using some available metadata:
//!
//! ```ignore
//! "
//! //- /lib.rs crate:foo deps:bar,baz cfg:foo=a,bar=b env:OUTDIR=path/to,OTHER=foo
//! fn insert_source_code_here() {}
//! "
//! ```

use stdx::trim_indent;

#[derive(Debug, Eq, PartialEq)]
pub struct Fixture {
    /// Specifies the path for this file. It must start with "/".
    pub path: String,
    /// Specifies the edition of this crate. This must be used with `crate` meta. If
    /// this is not specified, ([`base_db::input::Edition::CURRENT`]) will be used.
    /// This must be used with `crate` meta.
    ///
    /// Syntax: `edition:Pdf20`
    pub edition: Option<String>,
    /// Actual file contents. All meta comments are stripped.
    pub text: String,
}

pub struct FixtureWithProjectMeta {
    pub fixture: Vec<Fixture>,
    pub version: Option<String>,
}

impl FixtureWithProjectMeta {
    /// Parses text which looks like this:
    ///
    ///  ```text
    ///  //- some meta
    ///  line 1
    ///  line 2
    ///  //- other meta
    ///  ```
    ///
    /// Fixture can also start with a proc_macros and minicore declaration (in that order):
    ///
    /// ```text
    /// //- toolchain: nightly
    /// //- proc_macros: identity
    /// //- minicore: sized
    /// ```
    ///
    /// That will set toolchain to nightly and include predefined proc macros and a subset of
    /// `libcore` into the fixture, see `minicore.rs` for what's available. Note that toolchain
    /// defaults to stable.
    pub fn parse(#[rust_analyzer::rust_fixture] ra_fixture: &str) -> Self {
        let fixture = trim_indent(ra_fixture);
        let fixture = fixture.as_str();
        let mut res: Vec<Fixture> = Vec::new();
        let mut version = None;

        const PDF_HEADER: &str = "%PDF-";
        if fixture.starts_with(PDF_HEADER) {
            let (meta, _) = fixture[PDF_HEADER.len()..].split_once('\n').unwrap();
            version = Some(meta.to_owned());
        }

        let default = if fixture.contains("//-") { None } else { Some("//- /main.pdf") };

        for (ix, line) in default.into_iter().chain(fixture.split_inclusive('\n')).enumerate() {
            if let Some(line) = line.strip_prefix("//-") {
                let meta = Self::parse_meta_line(line);
                res.push(meta);
            }
        }

        Self { fixture: res, version }
    }
}
