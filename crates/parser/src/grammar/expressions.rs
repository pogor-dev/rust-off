use super::*;

const EXPR_FIRST: TokenSet = LHS_FIRST;

pub(super) fn expr(p: &mut Parser<'_>) -> Option<CompletedMarker> {
    expr_bp(p, None, 1)
}

// Parses expression with binding power of at least bp.
fn expr_bp(p: &mut Parser<'_>, m: Option<Marker>, _bp: u8) -> Option<CompletedMarker> {
    let m = m.unwrap_or_else(|| {
        let m = p.start();
        m
    });

    if !p.at_ts(EXPR_FIRST) {
        p.err_recover("expected expression", atom::EXPR_RECOVERY_SET);
        m.abandon(p);
        return None;
    }

    let lhs = match lhs(p) {
        Some(lhs) => {
            let lhs = lhs.extend_to(p, m);
            lhs
        }
        None => {
            m.abandon(p);
            return None;
        }
    };

    Some(lhs)
}

const LHS_FIRST: TokenSet = atom::ATOM_EXPR_FIRST;

fn lhs(p: &mut Parser<'_>) -> Option<CompletedMarker> {
    let _kind = match p.current() {
        _ => {
            let lhs = atom::atom_expr(p)?;
            return Some(lhs);
        }
    };
}
