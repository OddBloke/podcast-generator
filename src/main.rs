extern crate tempdir;

use std::ffi::OsStr;
use std::fs;
use std::io;

fn main() {
    println!("Hello, world!");
}

struct PodcastItem {}

fn get_target_items(target_directory: &std::path::Path) -> io::Result<Vec<PodcastItem>> {
    let mut items = Vec::new();
    let entries = fs::read_dir(target_directory).unwrap();
    for entry in entries {
        let entry = entry.unwrap();
        if fs::metadata(entry.path()).unwrap().is_dir() {
            continue;
        }
        match entry.path().extension() {
            Some(x) if x == OsStr::new("mp3") => (),
            _ => continue,
        }
        items.push(PodcastItem {});
    }
    Ok(items)
}

#[cfg(test)]
mod test {
    use std::fs::File;

    use tempdir::TempDir;

    use get_target_items;

    fn setup_tmpdir_with_items(items: Vec<&str>) -> TempDir {
        let tmpdir = TempDir::new("test").expect("create temp dir");
        for filename in &items {
            let full_path = tmpdir.path().join(filename);
            File::create(full_path).expect("create file");
        }
        tmpdir
    }

    #[test]
    fn list_empty_directory() {
        let tmpdir = setup_tmpdir_with_items(Vec::new());

        let items = get_target_items(tmpdir.path()).unwrap();
        assert_eq!(0, items.len())
    }

    #[test]
    fn list_single_item() {
        let tmpdir = setup_tmpdir_with_items(vec!["first.mp3"]);

        let items = get_target_items(tmpdir.path()).unwrap();
        assert_eq!(1, items.len())
    }

    #[test]
    fn non_mp3_files_ignored() {
        let tmpdir = setup_tmpdir_with_items(vec!["first.mp3", "second.not"]);

        let items = get_target_items(tmpdir.path()).unwrap();
        assert_eq!(1, items.len())
    }

    #[test]
    fn non_extension_files_ignored() {
        let tmpdir = setup_tmpdir_with_items(vec!["first.mp3", "second"]);

        let items = get_target_items(tmpdir.path()).unwrap();
        assert_eq!(1, items.len())
    }
}
