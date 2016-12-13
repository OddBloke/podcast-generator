extern crate tempdir;

use std::fs;
use std::io;

fn main() {
    println!("Hello, world!");
}

struct PodcastItem {}

fn get_target_items(target_directory: &std::path::Path) -> io::Result<Vec<PodcastItem>> {
    let mut items = Vec::new();
    let entries = try!(fs::read_dir(target_directory));
    for entry in entries {
        let entry = try!(entry);
        if try!(fs::metadata(entry.path())).is_dir() {
            continue;
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
        let tmpdir = setup_tmpdir_with_items(vec!["first"]);

        let items = get_target_items(tmpdir.path()).unwrap();
        assert_eq!(1, items.len())
    }
}
