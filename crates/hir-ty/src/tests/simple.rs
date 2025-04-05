use expect_test::expect;

use super::check_infer;

#[test]
fn infer_let() {
    check_infer(
        r#"
fn test() {
    let a = 1isize;
    let b: usize = 1;
    let c = b;
    let d: u32;
    let e;
    let f: i32 = e;
}
"#,
        expect![[r#"
            10..117 '{     ...= e; }': ()
            20..21 'a': isize
            24..30 '1isize': isize
            40..41 'b': usize
            51..52 '1': usize
            62..63 'c': usize
            66..67 'b': usize
            77..78 'd': u32
            93..94 'e': i32
            104..105 'f': i32
            113..114 'e': i32
        "#]],
    );
}
