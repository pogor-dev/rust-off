use super::*;

pub(crate) const LITERAL_FIRST: TokenSet = TokenSet::new(&[T![true], T![false], T![null], INT_NUMBER, REAL_NUMBER, LITERAL_STRING, HEX_STRING, NAME]);
pub(super) const EXPR_RECOVERY_SET: TokenSet = TokenSet::new(&[T![>>], T![']']]);
pub(super) const ATOM_EXPR_FIRST: TokenSet = LITERAL_FIRST.union(TokenSet::new(&[T!['[']]));

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
    let done = match p.current() {
        T!['['] => array_expr(p),
        _ => {
            p.err_and_bump("expected expression");
            return None;
        }
    };
    return Some(done);
}

fn array_expr(p: &mut Parser<'_>) -> CompletedMarker {
    assert!(p.at(T!['[']));
    let m = p.start();

    p.bump(T!['[']);
    while !p.at(EOF) && !p.at(T![']']) {
        if p.at(T!['[']) {
            array_expr(p);
        }

        if expressions::expr(p).is_none() {
            break;
        }
    }

    p.expect(T![']']);
    m.complete(p, ARRAY_EXPR)
}
