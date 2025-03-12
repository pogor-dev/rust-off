use super::*;

pub(super) fn mod_contents(p: &mut Parser<'_>) {
    while !(p.at(EOF)) {
        p.bump_any();
    }
}
