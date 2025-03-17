use super::*;

pub(super) fn pdf_body(p: &mut Parser<'_>) {
    while !(p.at(EOF)) {
        atom::atom_expr(p);
    }
}
