use std::{
    ffi::OsStr,
    fs::{self, File},
    io::BufReader,
    path::{Path, PathBuf},
};

use xml::reader::{EventReader, XmlEvent};

pub struct Reader;

impl Reader {
    pub fn read_file(file: &PathBuf) -> Result<String, std::io::Error> {
        match Path::new(file).extension().and_then(OsStr::to_str).unwrap() {
            "xml" | "xhtml" => Ok(Self::read_xml_file(&file)),
            _ => fs::read_to_string(&file),
        }
    }

    fn read_xml_file(filepath: &PathBuf) -> String {
        let file = File::open(filepath).unwrap();
        let event_reader = EventReader::new(BufReader::new(file));

        let mut content = String::new();

        for event in event_reader.into_iter() {
            if let Ok(e) = event {
                if let XmlEvent::Characters(text) = e {
                    content.push_str(&text);
                }
            }
        }

        content
    }
}
