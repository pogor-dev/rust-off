use super::*;

pub(super) fn pdf_body(p: &mut Parser<'_>) {
    while !(p.at(EOF)) {
        pdf_item(p);
    }
}

pub(super) fn pdf_item(p: &mut Parser<'_>) -> Option<CompletedMarker> {
    if let Some(m) = indirect_object(p) {
        return Some(m);
    }

    expressions::expr(p)
}

fn indirect_object(p: &mut Parser<'_>) -> Option<CompletedMarker> {
    let m: Marker = p.start();

    if indirect_reference_definition(p).is_none() {
        m.abandon(p);
        return None;
    };

    // indirect object body
    let object_body = match expressions::expr(p) {
        Some(object_body) => object_body,
        None => {
            m.abandon(p);
            return None;
        }
    };

    if object_body.kind() == DICTIONARY_EXPR && p.at(RAW_STREAM) {
        stream_expr(p);
    }

    assert!(p.at(T![endobj]));
    atom::atom_expr(p); // endobj
    return Some(m.complete(p, INDIRECT_OBJECT_EXPR));
}

fn indirect_reference_definition(p: &mut Parser<'_>) -> Option<CompletedMarker> {
    match p.nth_at(0, INT_NUMBER) && p.nth_at(1, INT_NUMBER) && p.nth_at(2, T![obj]) {
        true => {
            let m = p.start();
            atom::atom_expr(p); // object number
            atom::atom_expr(p); // generation number
            atom::atom_expr(p); // obj keyword
            Some(m.complete(p, INDIRECT_OBJECT_ID))
        }
        false => None,
    }
}

fn stream_expr(p: &mut Parser<'_>) -> CompletedMarker {
    // assert!(p.at(T![stream]));
    let m = p.start();

    // p.bump(T![stream]);
    // TODO: change the lexer to differentiate between stream and endstream
    while !p.at(EOF) && !p.at(T![endobj]) {
        p.bump_any();
    }

    // p.expect(T![endstream]);
    m.complete(p, STREAM_EXPR)
}
