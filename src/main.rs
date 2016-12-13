extern crate docopt;
extern crate tempdir;

use std::env;
use std::io;
use std::path::Path;

use docopt::Docopt;

mod collect;

#[cfg(test)]
mod test_helpers;

const USAGE: &'static str = "
Usage: podcast-generator SOURCE
";

fn main() {
    let our_stdout = io::stdout();
    real_main(env::args(), &mut our_stdout.lock());
}


fn real_main<I, S, W: io::Write>(argv: I, writer: &mut W)
    where I: IntoIterator<Item = S>,
          S: AsRef<str>
{
    let args = Docopt::new(USAGE).and_then(|d| d.argv(argv).parse()).unwrap_or_else(|e| e.exit());
    let podcast_items = collect::get_target_items(Path::new(args.get_str("SOURCE")));
    for podcast_item in podcast_items.unwrap().iter() {
        writer.write_fmt(format_args!("{}\n", podcast_item.name));
    }
}

#[cfg(test)]
mod test {
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
