use rsu::*;

#[test]
fn cmd_cd() {
    assert_eq!(check_cmd("cd"), true);
}
#[test]
fn cmd_abcd() {
    assert_eq!(check_cmd("abcd"), false);
}

#[test]
fn path_lib() {
    assert_eq!(check_path("src/lib.rs"), true);
}

#[test]
fn path_src2() {
    assert_eq!(check_path("./src2"), false);
}
