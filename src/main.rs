extern crate docopt;
extern crate tempdir;

use std::env;
use std::ffi::OsStr;
use std::fs;
use std::io;
use std::path::Path;

use docopt::Docopt;

const USAGE: &'static str = "
Usage: podcast-generator SOURCE
";

fn main() {
    let our_stdout = io::stdout();
    real_main(env::args(), &mut our_stdout.lock());
}


fn real_main<I, S, W: io::Write>(argv: I, writer: &mut W)
    where I: IntoIterator<Item=S>, S: AsRef<str> {
    let args = Docopt::new(USAGE).and_then(|d| d.argv(argv).parse()).unwrap_or_else(|e| e.exit());
    let podcast_items = get_target_items(Path::new(args.get_str("SOURCE")));
    for podcast_item in podcast_items.unwrap().iter() {
        writer.write_fmt(format_args!("{}\n", podcast_item.name));
    }
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
mod test_helpers {
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
}

#[cfg(test)]
mod test_main {
    use std::io;
    use std::io::Read;
    use std::io::Seek;
    use std::io::SeekFrom;

    use test_helpers::setup_tmpdir_with_items;

    use real_main;

    fn check_expected_names_are_output(files_to_write: Vec<&str>, expected_names: Vec<&str>) {
        let tmpdir = setup_tmpdir_with_items(&files_to_write);

        let args = vec!["command_name", tmpdir.path().to_str().unwrap()];
        let stdout_vector = vec![0; 15];
        let mut cursor = io::Cursor::new(stdout_vector);

        real_main(args.iter(), &mut cursor);

        let mut written_contents = vec![0; 15];
        cursor.seek(SeekFrom::Start(0)).unwrap();
        cursor.read_to_end(&mut written_contents).unwrap();
        let written_string = String::from_utf8(written_contents).unwrap();
        for name in expected_names {
            assert!(written_string.contains(format!("{}\n", name).as_str()))
        }
    }

    #[test]
    fn test_empty_existing_directory() {
        let tmpdir = setup_tmpdir_with_items(&Vec::new());

        let args = vec!["command_name", tmpdir.path().to_str().unwrap()];
        let stdout_vector = vec![0; 15];
        let mut cursor = io::Cursor::new(stdout_vector);

        real_main(args.iter(), &mut cursor);

        for &character in cursor.get_ref() {
            assert_eq!(0, character);
        }
    }

    #[test]
    fn test_single_file() {
        let files = vec!["first.mp3"];
        let names = files.clone();
        check_expected_names_are_output(files, names);
    }

    #[test]
    fn test_multiple_files() {
        let files = vec!["first.mp3", "second.mp3"];
        let names = files.clone();
        check_expected_names_are_output(files, names);
    }

    #[test]
    fn test_non_mp3_files() {
        let mut files = vec!["first.mp3", "second.mp3"];
        let names = files.clone();
        files.push("not-an-mp3");
        files.push("but-similar.mp3a");
        check_expected_names_are_output(files, names);
    }
}

#[cfg(test)]
mod test {
    use std::collections::HashSet;
    use std::io;
    use std::path::Path;

    use get_target_items;

    use test_helpers::setup_tmpdir_with_items;

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
