use super::*;

pub(crate) const LITERAL_FIRST: TokenSet = TokenSet::new(&[T![true], T![false], T![null], INT_NUMBER, REAL_NUMBER, LITERAL_STRING, HEX_STRING, NAME]);
pub(super) const EXPR_RECOVERY_SET: TokenSet = TokenSet::new(&[T![>>], T![']']]);

pub(crate) fn literal(p: &mut Parser<'_>) -> Option<CompletedMarker> {
    if !p.at_ts(LITERAL_FIRST) {
        return None;
    }
    let m = p.start();
    p.bump_any();
    Some(m.complete(p, LITERAL))
}

pub(super) fn atom_expr(p: &mut Parser<'_>) -> Option<CompletedMarker> {
    if let Some(m) = literal(p) {
        return Some(m);
    }

    return None;
}
