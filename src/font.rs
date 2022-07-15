use std::fs;

use std::str::from_utf8;
use quick_xml::Reader;
use quick_xml::events::Event;
use quick_xml::events::attributes::{Attribute, AttrError};

pub fn load_font(font_name: &str) {

    let accepted_characters = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789 ?.-#".to_owned();

    let font_file_raw = fs::read_to_string(font_name)
        .expect("Something went wrong reading the font file");
    let mut reader = Reader::from_str(&font_file_raw);

    loop {
        match reader.read_event_unbuffered() {
            Ok(Event::Empty(ref e)) => {
                match e.name() {
                    b"glyph" => {
                        let mut unicode = Option::None;
                        let mut width= Option::None;
                        let mut path= Option::None;

                        for attr in e.attributes() {
                            let attr = attr.unwrap();

                            let attr_value = from_utf8(&attr.value).unwrap().to_owned();
                            match attr.key {
                                b"unicode" =>{
                                    unicode = Some(attr_value);
                                },
                                b"horiz-adv-x" => {
                                    width = Some(attr_value);
                                },
                                b"d" => {
                                    path = Some(attr_value);
                                },
                                _ =>(),
                            }
                        }

                        match (unicode, width, path) {
                            (Some(unicode), Some(width), Some(path)) => {
                                if accepted_characters.contains(&unicode)
                                {
                                    println!("{} - {} - {}", unicode, width, path);
                                }
                            },
                            _ => (),
                        }
                    },
                    _ => (),
                }
            },
        Ok(Event::Eof) => break, // exits the loop when reaching end of file
        Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
        _ => (),
        }
    }
}