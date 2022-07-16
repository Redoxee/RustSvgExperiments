use std::fs;
use std::vec;
use std::collections::HashMap;


use std::str::from_utf8;
use glam::Vec2;
use quick_xml::Reader;
use quick_xml::events::Event;
use quick_xml::events::attributes::{Attribute, AttrError};

use crate::utils::Instruction;

pub struct Sigil {
    path : Vec<Instruction>,
    width : f32, 
}

pub struct Font {
    sigils: HashMap<String, Sigil>,
} 

impl Font {

    pub fn load(font_name: &str) -> Font {
        
        let accepted_characters = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789 ?.-#".to_owned();

        let font_file_raw = fs::read_to_string(font_name)
            .expect("Something went wrong reading the font file");
        let mut reader = Reader::from_str(&font_file_raw);

        let mut font = Font{
            sigils: HashMap::new(),
        };

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
                                        let mut instructions = Vec::new();

                                        let splitted : Vec<&str> = path.split(" ").collect();
                                        
                                        for split in splitted.chunks(3) {
                                            let operation = split[0];
                                            let x = split[1].parse::<f32>().unwrap();
                                            let y = split[2].parse::<f32>().unwrap();
                                            let pos = Vec2::new(x, y);
                                            instructions.push(match operation {
                                                "M" => {Instruction::MoveTo(pos)},
                                                "L" => {Instruction::LineTo(pos)},
                                                _=> panic!("Unknown Instruction {}.", operation)
                                            });
                                        }

                                        font.sigils.insert(unicode,
                                            Sigil {
                                                path : instructions,
                                                width: width.parse::<f32>().unwrap(),
                                                });
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

        return font;
    }

    pub fn print_in_instructions(&self, data : &str, position : &Vec2, scale : &f32, instructions : &mut Vec<Instruction>) {

    }
}