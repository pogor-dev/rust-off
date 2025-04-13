use std::{env, sync::LazyLock};

use expect_test::Expect;
use test_fixture::WithFixture;
use tracing_subscriber::{Registry, layer::SubscriberExt};
use tracing_tree::HierarchicalLayer;

use crate::test_db::TestDB;

mod simple;

fn setup_tracing() -> Option<tracing::subscriber::DefaultGuard> {
    static ENABLE: LazyLock<bool> = LazyLock::new(|| env::var("CHALK_DEBUG").is_ok());
    if !*ENABLE {
        return None;
    }

    let filter: tracing_subscriber::filter::Targets = env::var("CHALK_DEBUG").ok().and_then(|it| it.parse().ok()).unwrap_or_default();
    let layer = HierarchicalLayer::default()
        .with_indent_lines(true)
        .with_ansi(false)
        .with_indent_amount(2)
        .with_writer(std::io::stderr);
    let subscriber = Registry::default().with(filter).with(layer);
    Some(tracing::subscriber::set_default(subscriber))
}

fn check_infer(#[rust_analyzer::rust_fixture] ra_fixture: &str, expect: Expect) {
    let mut actual = infer(ra_fixture);
    actual.push('\n');
    expect.assert_eq(&actual);
}

fn infer(#[rust_analyzer::rust_fixture] ra_fixture: &str) -> String {
    infer_with_mismatches(ra_fixture, false)
}

fn infer_with_mismatches(content: &str, include_mismatches: bool) -> String {
    let _tracing = setup_tracing();
    let (db, file_id) = TestDB::with_single_file(content);
    let mut buf = String::new();

    let mut defs: Vec<DefWithBodyId> = Vec::new();
    visit_module(&db, &def_map, module.local_id, &mut |it| {
        let def = match it {
            ModuleDefId::IndirectObjectId(it) => it.into(),
            _ => return,
        };
        defs.push((def, module.krate()))
    });

    buf.truncate(buf.trim_end().len());
    buf
}
