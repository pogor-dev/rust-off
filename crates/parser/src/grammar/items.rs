use super::*;
const INDIRECT_KEYWORDS: TokenSet = TokenSet::new(&[T![R], T![obj]]);

pub(super) fn pdf_body(p: &mut Parser<'_>) {
    while !(p.at(EOF)) {
        pdf_item(p);
    }
}

pub(super) fn pdf_item(p: &mut Parser<'_>) -> Option<CompletedMarker> {
    let la = p.nth(1);
    match p.current() {
        INT_NUMBER if la == INT_NUMBER && p.nth_at_ts(2, INDIRECT_KEYWORDS) => indirect_obj_or_reference(p),
        _ => atom::atom_expr(p),
    }
}

fn indirect_obj_or_reference(p: &mut Parser<'_>) -> Option<CompletedMarker> {
    assert!(p.at(INT_NUMBER));
    let m = p.start();
    atom::atom_expr(p); // object number
    assert!(p.at(INT_NUMBER));
    atom::atom_expr(p); // generation number
    assert!(p.at_ts(INDIRECT_KEYWORDS));
    let k = p.current();
    atom::atom_expr(p); // obj or R keyword
    Some(m.complete(p, if k == T![obj] { OBJECT_ID } else { INDIRECT_REFERENCE_EXPR }))
}
