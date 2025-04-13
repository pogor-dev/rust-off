use super::check_infer;
use expect_test::expect;

#[test]
fn infer_basics() {
    check_infer(
        r#"
fn test(a: u32, b: isize, c: !, d: &str) {
    a;
    b;
    c;
    d;
    1usize;
    1isize;
    "test";
    1.0f32;
}
"#,
        expect![[r#"
            8..9 'a': u32
            16..17 'b': isize
            26..27 'c': !
            32..33 'd': &'? str
            41..120 '{     ...f32; }': ()
            47..48 'a': u32
            54..55 'b': isize
            61..62 'c': !
            68..69 'd': &'? str
            75..81 '1usize': usize
            87..93 '1isize': isize
            99..105 '"test"': &'static str
            111..117 '1.0f32': f32
        "#]],
    );
}
