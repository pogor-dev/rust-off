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

    let mut infer_def = |inference_result: Arc<InferenceResult>, body: Arc<Body>, body_source_map: Arc<BodySourceMap>| {};

    let module = db.module_for_file(file_id);
    let def_map = module.def_map(&db);

    let mut defs: Vec<(DefWithBodyId, Crate)> = Vec::new();
    visit_module(&db, &def_map, module.local_id, &mut |it| {
        let def = match it {
            ModuleDefId::FunctionId(it) => it.into(),
            ModuleDefId::EnumVariantId(it) => it.into(),
            ModuleDefId::ConstId(it) => it.into(),
            ModuleDefId::StaticId(it) => it.into(),
            _ => return,
        };
        defs.push((def, module.krate()))
    });
    defs.sort_by_key(|(def, _)| match def {
        DefWithBodyId::FunctionId(it) => {
            let loc = it.lookup(&db);
            loc.source(&db).value.syntax().text_range().start()
        }
        DefWithBodyId::ConstId(it) => {
            let loc = it.lookup(&db);
            loc.source(&db).value.syntax().text_range().start()
        }
        DefWithBodyId::StaticId(it) => {
            let loc = it.lookup(&db);
            loc.source(&db).value.syntax().text_range().start()
        }
        DefWithBodyId::VariantId(it) => {
            let loc = it.lookup(&db);
            loc.source(&db).value.syntax().text_range().start()
        }
        DefWithBodyId::InTypeConstId(it) => it.source(&db).syntax().text_range().start(),
    });
    for (def, krate) in defs {
        let (body, source_map) = db.body_with_source_map(def);
        let infer = db.infer(def);
        infer_def(infer, body, source_map, krate);
    }

    buf.truncate(buf.trim_end().len());
    buf
}
