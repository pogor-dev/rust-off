use super::*;

pub(super) fn pdf_body(p: &mut Parser<'_>) {
    while !(p.at(EOF)) {
        pdf_item(p);
    }
}

pub(super) fn pdf_item(p: &mut Parser<'_>) {
    let m = p.start();

    let m = match opt_item(p, m) {
        Ok(()) => {
            return;
        }
        Err(m) => m,
    };

    m.abandon(p);
    atom::atom_expr(p);
}

/// Try to parse an item, completing `m` in case of success.
pub(super) fn opt_item(p: &mut Parser<'_>, m: Marker) -> Result<(), Marker> {
    if p.at(INT_NUMBER) && matches!((p.nth(1), p.nth(2)), (INT_NUMBER, T![R])) {
        atom::atom_expr(p);
        atom::atom_expr(p);
        atom::atom_expr(p);
        m.complete(p, INDIRECT_REFERENCE_EXPR);
    }
    Ok(())
}
