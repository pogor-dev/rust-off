//! See [`Name`].

use std::fmt;

use intern::Symbol;

/// `Name` is a wrapper around string, which is used in hir for both references
/// and declarations. In theory, names should also carry hygiene info, but we are
/// not there yet!
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Name {
    symbol: Symbol,
    // If you are making this carry actual hygiene, beware that the special handling for variables and labels
    // in bodies can go.
    ctx: (),
}

impl fmt::Debug for Name {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Name").field("symbol", &self.symbol.as_str()).field("ctx", &self.ctx).finish()
    }
}

impl Ord for Name {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.symbol.as_str().cmp(other.symbol.as_str())
    }
}

impl PartialOrd for Name {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

// No need to strip `r#`, all comparisons are done against well-known symbols.
impl PartialEq<Symbol> for Name {
    fn eq(&self, sym: &Symbol) -> bool {
        self.symbol == *sym
    }
}

impl PartialEq<&Symbol> for Name {
    fn eq(&self, &sym: &&Symbol) -> bool {
        self.symbol == *sym
    }
}

impl PartialEq<Name> for Symbol {
    fn eq(&self, name: &Name) -> bool {
        *self == name.symbol
    }
}

impl PartialEq<Name> for &Symbol {
    fn eq(&self, name: &Name) -> bool {
        **self == name.symbol
    }
}
