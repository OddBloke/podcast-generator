use std::io;

use xml::common::XmlVersion;
use xml::writer::{EmitterConfig, XmlEvent};

pub struct PodcastItem {
    pub name: String,
}

pub struct Podcast {}

pub fn create_podcast_xml<W: io::Write>(podcast: Podcast, writer: &mut W) {
    let mut writer = EmitterConfig::new().perform_indent(true).create_writer(writer);
    let start_doc_event = XmlEvent::StartDocument{
        encoding: Some("utf-8"),
        version: XmlVersion::Version10,
        standalone: Some(true),
    };
    writer.write(start_doc_event);
}

#[cfg(test)]
mod test {
    use std::io;
    use std::io::Seek;
    use std::io::SeekFrom;

    use xml::reader::{EventReader, XmlEvent};

    use podcast::Podcast;
    use podcast::create_podcast_xml;

    #[test]
    fn test_empty_podcast_produces_empty_xml() {
        let stdout_vector = vec![0; 15];
        let mut cursor = io::Cursor::new(stdout_vector);

        create_podcast_xml(Podcast{}, &mut cursor);

        cursor.seek(SeekFrom::Start(0)).unwrap();
        let parser = EventReader::new(cursor);
        let mut starts_found = 0;
        for e in parser {
            match e {
                Ok(XmlEvent::StartDocument {..}) => starts_found += 1,
                _ => (),
            }
        }
        assert_eq!(1, starts_found)
    }
}
