//! docgen — generates docs/components.md Shared-Components section from ui/widgets/*.slint.
//! Usage: cargo run -p docgen

use std::{io::Write, path::Path};

mod parser;
mod renderer;

/// Slint property access qualifier.
#[derive(PartialEq)]
pub(crate) enum PropMode { In, InOut, Out }

impl PropMode {
    fn detect(t: &str) -> Option<Self> {
        if      t.starts_with("in-out property") { Some(Self::InOut) }
        else if t.starts_with("in property")     { Some(Self::In)    }
        else if t.starts_with("out property")    { Some(Self::Out)   }
        else                                     { None              }
    }
    fn label(&self) -> &'static str {
        match self { Self::In => "in", Self::InOut => "in-out", Self::Out => "out" }
    }
}

/// Parsed Slint property declaration.
pub(crate) struct Prop { pub(crate) mode: PropMode, pub(crate) ty: String, pub(crate) name: String, pub(crate) default: String, pub(crate) desc: String }
/// Parsed Slint callback declaration.
pub(crate) struct Cb   { pub(crate) name: String, pub(crate) args: String, pub(crate) desc: String }
/// Parsed Slint struct field.
pub(crate) struct Field { pub(crate) name: String, pub(crate) ty: String }

/// Parsed Slint component or struct item.
pub(crate) enum Item {
    Struct { name: String, #[allow(dead_code)] file: String, fields: Vec<Field> },
    Comp   { name: String, file: String, props: Vec<Prop>, cbs: Vec<Cb> },
}

/// Length of "//" comment prefix.
pub(crate) const COMMENT_PREFIX_LEN: usize = "//".len();
/// File extension for Slint source files.
pub(crate) const SLINT_EXT: &str = "slint";

type Err = Box<dyn std::error::Error>;

fn run() -> Result<(), Err> {
    let all = renderer::scan(Path::new("ui/shared"))?;
    let (c, s) = renderer::generate(&all, Path::new("docs/components.md"))?;
    writeln!(std::io::stdout(), "docgen: {c} components + {s} structs → docs/components.md")?;
    Ok(())
}

fn main() -> Result<(), Err> { run() }
