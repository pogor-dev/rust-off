//! This module greps parser's code for specially formatted comments and turns
//! them into tests.
#![allow(clippy::disallowed_types, clippy::print_stdout)]

use std::{collections::HashMap, fs, path::Path};

use anyhow::Result;
use itertools::Itertools as _;

use crate::{
    codegen::{ensure_file_contents, reformat},
    flags::CodegenType,
    project_root,
};

pub(crate) fn generate(codegen_type: CodegenType, test_data_path_str: &str, check: bool) {
    let crate_root_path = project_root().join("crates/parser");
    let test_data_path = crate_root_path.join(test_data_path_str);

    let tests_ok = list(&test_data_path.join("ok")).unwrap();
    let tests_err = list(&test_data_path.join("err")).unwrap();

    let ok_tests = tests_ok.values().sorted_by(|a, b| a.name.cmp(&b.name)).map(|test| {
        let test_name = quote::format_ident!("{}", test.name);
        let test_file = format!("{test_data_path_str}/ok/{test_name}.pdf");
        let fun_name = match codegen_type {
            CodegenType::ParserLexerStrTests => quote::format_ident!("lex_and_expect_no_errors"),
            _ => quote::format_ident!("parse_and_expect_no_errors"),
        };
        let (test_func, args) = (fun_name, quote::quote! {#test_file});

        quote::quote! {
            #[test]
            fn #test_name() {
                #test_func(#args);
            }
        }
    });

    let err_tests = tests_err.values().sorted_by(|a, b| a.name.cmp(&b.name)).map(|test| {
        let test_name = quote::format_ident!("{}", test.name);
        let test_file = format!("{test_data_path_str}/err/{test_name}.pdf");
        let fun_name = match codegen_type {
            CodegenType::ParserLexerStrTests => quote::format_ident!("lex_and_expect_errors"),
            _ => quote::format_ident!("parse_and_expect_errors"),
        };
        let (test_func, args) = (fun_name, quote::quote! {#test_file});

        quote::quote! {
            #[test]
            fn #test_name() {
                #test_func(#args);
            }
        }
    });

    let output = quote::quote! {
        mod ok {
            #![allow(unused_imports)]
            use crate::tests::*;
            #(#ok_tests)*
        }
        mod err {
            #![allow(unused_imports)]
            use crate::tests::*;
            #(#err_tests)*
        }
    };

    let pretty = reformat(output.to_string());
    ensure_file_contents(codegen_type, test_data_path.join("generated/runner.rs").as_ref(), &pretty, check);
}

#[derive(Debug)]
struct TestCase {
    name: String,
}

fn list(dir: &Path) -> Result<HashMap<String, TestCase>> {
    let mut res = HashMap::new();
    let read_dir = fs::read_dir(&dir).unwrap_or_else(|err| panic!("can't `read_dir` {}: {err}", dir.display()));

    for file in read_dir {
        let file = file.unwrap();
        let path = file.path();

        if path.extension().unwrap_or_default() == "pdf" {
            let hashkey = path.file_stem().map(|x| x.to_string_lossy().to_string()).unwrap();
            let name = hashkey.clone();
            let test = TestCase { name };
            if let Some(old) = res.insert(hashkey, test) {
                println!("Duplicate test: {:?}", old);
            }
        }
    }

    Ok(res)
}

#[test]
fn test() {
    generate(CodegenType::ParserLexerStrTests, "test_data/lexer", true);
    generate(CodegenType::ParserTests, "test_data/parser", true);
}
