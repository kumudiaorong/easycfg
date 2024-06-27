use std::{
    fs::{self, remove_dir_all},
    panic,
};

#[test]
fn symlink() {
    let testdir = "unit_exec_symlink_test";
    let hook = std::panic::take_hook();
    panic::set_hook(Box::new(move |info| {
        remove_dir_all(testdir).unwrap();
        hook(info);
    }));
    fs::create_dir(testdir).unwrap();
    let src = testdir.to_string() + "/test.txt";
    fs::write(&src, "Hello, World!").unwrap();
    let src = fs::canonicalize(src).unwrap();
    let mut exec = super::Executer::new(crate::config::Distri::Unknown);
    let dst = testdir.to_string() + "/test_link.txt";
    exec.symlink(&src, &dst).unwrap();
    assert_eq!(fs::read_to_string(&dst).unwrap(), "Hello, World!");
    let dst = testdir.to_string() + "/parent/test_link.txt";
    exec.symlink(&src, &dst).unwrap();
    assert_eq!(fs::read_to_string(&dst).unwrap(), "Hello, World!");
    fs::remove_dir_all(testdir).unwrap();
}
