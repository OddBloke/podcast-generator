extern crate tempdir;

use std::ffi::OsStr;
use std::fs;
use std::io;

fn main() {
    println!("Hello, world!");
}

struct PodcastItem {
    name: String,
}

fn get_target_items(target_directory: &std::path::Path) -> io::Result<Vec<PodcastItem>> {
    let mut items = Vec::new();
    let entries = try!(fs::read_dir(target_directory));
    for entry in entries {
        let entry = entry.unwrap();
        let path = entry.path();
        if fs::metadata(&path).unwrap().is_dir() {
            continue;
        }
        match path.extension() {
            Some(x) if x == OsStr::new("mp3") => (),
            _ => continue,
        }
        items.push(PodcastItem { name: String::from(path.file_name().unwrap().to_str().unwrap()) });
    }
    Ok(items)
}

#[cfg(test)]
mod test {
    use std::collections::HashSet;
    use std::fs::File;
    use std::io;
    use std::path::Path;

    use tempdir::TempDir;

    use get_target_items;

    fn setup_tmpdir_with_items(items: &Vec<&str>) -> TempDir {
        let tmpdir = TempDir::new("test").expect("create temp dir");
        for filename in items {
            let full_path = tmpdir.path().join(filename);
            File::create(full_path).expect("create file");
        }
        tmpdir
    }

    #[test]
    fn list_nonexistent_directory_returns_error() {
        match get_target_items(Path::new("/nope")) {
            Result::Ok(_) => panic!("Should fail against missing directory"),
            Result::Err(err) => assert_eq!(io::ErrorKind::NotFound, err.kind()),
        }
    }

    #[test]
    fn list_empty_directory() {
        let tmpdir = setup_tmpdir_with_items(&Vec::new());

        let items = get_target_items(tmpdir.path()).unwrap();
        assert_eq!(0, items.len())
    }

    #[test]
    fn list_single_item() {
        let tmpdir = setup_tmpdir_with_items(&vec!["first.mp3"]);

        let items = get_target_items(tmpdir.path()).unwrap();
        assert_eq!(1, items.len())
    }

    #[test]
    fn non_mp3_files_ignored() {
        let tmpdir = setup_tmpdir_with_items(&vec!["first.mp3", "second.not"]);

        let items = get_target_items(tmpdir.path()).unwrap();
        assert_eq!(1, items.len())
    }

    #[test]
    fn non_extension_files_ignored() {
        let tmpdir = setup_tmpdir_with_items(&vec!["first.mp3", "second"]);

        let items = get_target_items(tmpdir.path()).unwrap();
        assert_eq!(1, items.len())
    }

    #[test]
    fn filenames_returned_as_names() {
        let expected_names = vec!["first.mp3", "second.mp3"];
        let tmpdir = setup_tmpdir_with_items(&expected_names);

        let items = get_target_items(tmpdir.path()).unwrap();
        assert_eq!(2, items.len());

        let mut actual_names_set = HashSet::new();
        actual_names_set.extend(items.iter().map(|item| item.name.clone()));
        let mut expected_names_set = HashSet::new();
        for name in expected_names.iter() {
            expected_names_set.insert(String::from(name.clone()));
        }

        assert_eq!(expected_names_set, actual_names_set)
    }
}
