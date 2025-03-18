use super::*;
const INDIRECT_KEYWORDS: TokenSet = TokenSet::new(&[T![R], T![obj]]);

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
    let la = p.nth(1);
    match p.current() {
        INT_NUMBER if la == INT_NUMBER && p.nth_at_ts(2, INDIRECT_KEYWORDS) => indirect_reference(p, m),
        _ => return Err(m),
    }
    Ok(())
}

fn indirect_reference(p: &mut Parser<'_>, m: Marker) {
    assert!(p.at(INT_NUMBER));
    atom::atom_expr(p); // object number
    assert!(p.at(INT_NUMBER));
    atom::atom_expr(p); // generation number
    assert!(p.at_ts(INDIRECT_KEYWORDS));
    let k = p.current();
    atom::atom_expr(p); // obj or R keyword
    m.complete(p, if k == T![obj] { OBJECT_ID } else { INDIRECT_REFERENCE_EXPR });
}
