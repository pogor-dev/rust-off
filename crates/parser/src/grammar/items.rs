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

    if let Some(m) = cross_reference_table(p) {
        return Some(m);
    }

    expressions::expr(p)
}

fn cross_reference_table(p: &mut Parser<'_>) -> Option<CompletedMarker> {
    // The cross-reference table consists of one or more cross-reference sections.
    // Each cross-reference section starts with the xref keyword. See ISO 32000-1:2008, 7.5.4.
    if !p.at(T![xref]) {
        return None;
    }

    let m = p.start();

    while !(p.at(EOF)) {
        if cross_reference_section(p).is_none() {
            break;
        }
    }

    Some(m.complete(p, X_REF_TABLE))
}

fn cross_reference_section(p: &mut Parser<'_>) -> Option<CompletedMarker> {
    if !p.at(T![xref]) {
        return None;
    }

    let m = p.start();
    p.bump(T![xref]);

    while !(p.at(EOF)) {
        if cross_reference_sub_section(p).is_none() {
            break;
        }
    }

    Some(m.complete(p, X_REF_SECTION))
}

fn cross_reference_sub_section(p: &mut Parser<'_>) -> Option<CompletedMarker> {
    if !p.nth_at(0, INT_NUMBER) && !p.nth_at(1, INT_NUMBER) {
        return None;
    }

    let m = p.start();
    atom::atom_expr(p); // object number
    atom::atom_expr(p); // number of consecutive entries

    while !(p.at(EOF)) {
        if cross_reference_entry(p).is_none() {
            break;
        }
    }

    Some(m.complete(p, X_REF_SUBSECTION))
}

const X_REF_ENTRTY_TYPES: TokenSet = TokenSet::new(&[T![f], T![n]]);

fn cross_reference_entry(p: &mut Parser<'_>) -> Option<CompletedMarker> {
    if !p.nth_at(0, INT_NUMBER) && !p.nth_at(1, INT_NUMBER) && !p.nth_at_ts(2, X_REF_ENTRTY_TYPES) {
        return None;
    }

    let m = p.start();
    atom::atom_expr(p); // object number
    atom::atom_expr(p); // generation number
    atom::atom_expr(p); // 'f' or 'n' keyword
                        // assert_eq!(p.current(), NEWLINE);
    p.eat(NEWLINE); // end-of-line marker is part of the entry
    Some(m.complete(p, X_REF_ENTRY))
}

fn indirect_object(p: &mut Parser<'_>) -> Option<CompletedMarker> {
    let m: Marker = p.start();

    if indirect_reference_definition(p).is_none() {
        m.abandon(p);
        return None;
    };

    indirect_object_body(p);

    assert!(p.at(T![endobj]));
    atom::atom_expr(p); // endobj
    return Some(m.complete(p, INDIRECT_OBJECT_EXPR));
}

fn indirect_object_body(p: &mut Parser<'_>) {
    // When the object body is empty, we can just return, as there is no need to parse anything.
    if p.at(T![endobj]) {
        return;
    }

    let object_body = match expressions::expr(p) {
        Some(object_body) => object_body,
        None => return,
    };

    if object_body.kind() == DICTIONARY_EXPR && p.at(T![stream]) {
        stream_expr(p);
    }
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
    assert!(p.at(T![stream]));
    let m = p.start();

    p.bump(T![stream]);
    while !p.at(EOF) && !p.at(T![endstream]) {
        p.bump_any();
    }

    p.expect(T![endstream]);
    m.complete(p, STREAM_EXPR)
}
