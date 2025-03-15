use super::*;

pub(super) fn pdf_contents(p: &mut Parser<'_>) {
    while !(p.at(EOF)) {
        pdf_body(p);
    }
}

pub(super) fn pdf_body(p: &mut Parser<'_>) {
    atom::literal(p);
}
