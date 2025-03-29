use std::sync::Arc;

use expect_test::Expect;

mod simple;

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

    let mut infer_def = |inference_result: Arc<InferenceResult>, body: Arc<Body>, body_source_map: Arc<BodySourceMap>, krate: Crate| {
        let display_target = DisplayTarget::from_crate(&db, krate);
        let mut types: Vec<(InFile<SyntaxNode>, &Ty)> = Vec::new();
        let mut mismatches: Vec<(InFile<SyntaxNode>, &TypeMismatch)> = Vec::new();

        if let Some(self_param) = body.self_param {
            let ty = &inference_result.type_of_binding[self_param];
            if let Some(syntax_ptr) = body_source_map.self_param_syntax() {
                let root = db.parse_or_expand(syntax_ptr.file_id);
                let node = syntax_ptr.map(|ptr| ptr.to_node(&root).syntax().clone());
                types.push((node, ty));
            }
        }

        for (pat, mut ty) in inference_result.type_of_pat.iter() {
            if let Pat::Bind { id, .. } = body.pats[pat] {
                ty = &inference_result.type_of_binding[id];
            }
            let node = match body_source_map.pat_syntax(pat) {
                Ok(sp) => {
                    let root = db.parse_or_expand(sp.file_id);
                    sp.map(|ptr| ptr.to_node(&root).syntax().clone())
                }
                Err(SyntheticSyntax) => continue,
            };
            types.push((node.clone(), ty));
            if let Some(mismatch) = inference_result.type_mismatch_for_pat(pat) {
                mismatches.push((node, mismatch));
            }
        }

        for (expr, ty) in inference_result.type_of_expr.iter() {
            let node = match body_source_map.expr_syntax(expr) {
                Ok(sp) => {
                    let root = db.parse_or_expand(sp.file_id);
                    sp.map(|ptr| ptr.to_node(&root).syntax().clone())
                }
                Err(SyntheticSyntax) => continue,
            };
            types.push((node.clone(), ty));
            if let Some(mismatch) = inference_result.type_mismatch_for_expr(expr) {
                mismatches.push((node, mismatch));
            }
        }

        // sort ranges for consistency
        types.sort_by_key(|(node, _)| {
            let range = node.value.text_range();
            (range.start(), range.end())
        });
        for (node, ty) in &types {
            let (range, text) = if let Some(self_param) = ast::SelfParam::cast(node.value.clone()) {
                (self_param.name().unwrap().syntax().text_range(), "self".to_owned())
            } else {
                (node.value.text_range(), node.value.text().to_string().replace('\n', " "))
            };
            let macro_prefix = if node.file_id != file_id { "!" } else { "" };
            format_to!(
                buf,
                "{}{:?} '{}': {}\n",
                macro_prefix,
                range,
                ellipsize(text, 15),
                ty.display_test(&db, display_target)
            );
        }
        if include_mismatches {
            mismatches.sort_by_key(|(node, _)| {
                let range = node.value.text_range();
                (range.start(), range.end())
            });
            for (src_ptr, mismatch) in &mismatches {
                let range = src_ptr.value.text_range();
                let macro_prefix = if src_ptr.file_id != file_id { "!" } else { "" };
                format_to!(
                    buf,
                    "{}{:?}: expected {}, got {}\n",
                    macro_prefix,
                    range,
                    mismatch.expected.display_test(&db, display_target),
                    mismatch.actual.display_test(&db, display_target),
                );
            }
        }
    };

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
