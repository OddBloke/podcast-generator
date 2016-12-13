use std::fs::File;

use tempdir::TempDir;

pub fn setup_tmpdir_with_items(items: &Vec<&str>) -> TempDir {
    let tmpdir = TempDir::new("test").expect("create temp dir");
    for filename in items {
        let full_path = tmpdir.path().join(filename);
        File::create(full_path).expect("create file");
    }
    tmpdir
}
