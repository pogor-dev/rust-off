use super::*;

pub(crate) const LITERAL_FIRST: TokenSet = TokenSet::new(&[T![true], T![false], T![null], INT_NUMBER, REAL_NUMBER, LITERAL_STRING, HEX_STRING, NAME]);
pub(super) const EXPR_RECOVERY_SET: TokenSet = TokenSet::new(&[T![>>], T![']']]);
pub(super) const ATOM_EXPR_FIRST: TokenSet = LITERAL_FIRST.union(TokenSet::new(&[T!['['], T![<<]]));

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
    let la = p.nth(1);
    let done = match p.current() {
        T!['['] => array_expr(p),
        T![<<] => dictionary_expr(p),
        k if k.is_keyword(edition::Edition::LATEST) => {
            p.bump_any();
            return None;
        }
        INT_NUMBER if la == INT_NUMBER && p.nth(2) == T![R] => indirect_reference(p),
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
        if expressions::expr(p).is_none() {
            break;
        }
    }

    p.expect(T![']']);
    m.complete(p, ARRAY_EXPR)
}

fn dictionary_expr(p: &mut Parser<'_>) -> CompletedMarker {
    assert!(p.at(T![<<]));
    let m = p.start();

    p.bump(T![<<]);
    while !p.at(EOF) && !p.at(T![>>]) {
        // if !p.at_ts(LITERAL_FIRST) {
        //     p.err_and_bump("expected key");
        //     break;
        // }
        // literal(p);
        if expressions::expr(p).is_none() {
            break;
        }
    }

    p.expect(T![>>]);
    m.complete(p, DICTIONARY_EXPR)
}

fn indirect_reference(p: &mut Parser<'_>) -> CompletedMarker {
    assert!(p.at(INT_NUMBER));
    let m = p.start();

    p.bump(INT_NUMBER);
    p.bump(INT_NUMBER);
    p.bump(T![R]);

    m.complete(p, INDIRECT_REFERENCE_EXPR)
}
