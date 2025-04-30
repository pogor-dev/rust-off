use super::*;

#[test]
fn virtual_path_extensions() {
    assert_eq!(VirtualPath("/".to_owned()).name_and_extension(), None);
    assert_eq!(VirtualPath("/directory".to_owned()).name_and_extension(), Some(("directory", None)));
    assert_eq!(VirtualPath("/directory/".to_owned()).name_and_extension(), Some(("directory", None)));
    assert_eq!(VirtualPath("/directory/file".to_owned()).name_and_extension(), Some(("file", None)));
    assert_eq!(VirtualPath("/directory/.file".to_owned()).name_and_extension(), Some((".file", None)));
    assert_eq!(VirtualPath("/directory/.file.rs".to_owned()).name_and_extension(), Some((".file", Some("rs"))));
    assert_eq!(VirtualPath("/directory/file.rs".to_owned()).name_and_extension(), Some(("file", Some("rs"))));
}

#[test]
fn encode_virtual_path() {
    let path = VfsPath::new_virtual_path("/foo/bar.txt".to_owned());
    let mut buf = Vec::new();
    path.encode(&mut buf);
    assert_eq!(buf, b"\x01/foo/bar.txt");
}

#[cfg(unix)]
#[test]
fn encode_real_path_unix() {
    let abs = AbsPathBuf::assert("/tmp/test.txt".into());
    let path = VfsPath::from(abs);
    let mut buf = Vec::new();
    path.encode(&mut buf);
    assert_eq!(buf, b"\x00/tmp/test.txt");
}

#[cfg(windows)]
#[test]
fn encode_real_path_windows() {
    use std::path::PathBuf;
    let abs = AbsPathBuf::assert("C:/tmp/test.txt".into());
    let path = VfsPath::from(abs);
    let mut buf = Vec::new();
    path.encode(&mut buf);
    // The expected bytes are UTF-16LE encoding of the path string
    let expected: Vec<u8> = vec![
        0, 67, 0, 58, 0, 92, 0, 116, 0, 109, 0, 112, 0, 92, 0, 116, 0, 101, 0, 115, 0, 116, 0, 46, 0, 116, 0, 120, 0, 116, 0,
    ];
    assert_eq!(buf, expected);
}
