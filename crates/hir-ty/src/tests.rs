use std::sync::Arc;

use crate::{test_db::TestDB, InferenceResult};
use expect_test::Expect;
use hir_def::expr_store::{Body, BodySourceMap};
use std::env;
use std::sync::LazyLock;
use test_fixture::WithFixture;
use tracing_subscriber::{layer::SubscriberExt, Registry};
use tracing_tree::HierarchicalLayer;

mod simple;

// These tests compare the inference results for all expressions in a file
// against snapshots of the expected results using expect. Use
// `env UPDATE_EXPECT=1 cargo test -p hir_ty` to update the snapshots.

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

fn check_infer_with_mismatches(#[rust_analyzer::rust_fixture] ra_fixture: &str, expect: Expect) {
    let mut actual = infer_with_mismatches(ra_fixture, true);
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

    buf.truncate(buf.trim_end().len());
    buf
}
