extern crate boat_lang_core;

use crate::boat_lang_core::{
    program_parser,
    program_translator,
    program_optimizer,
    interpreter,
    boat_instructions,
};
extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;
use std::{collections::HashSet, io::{BufRead, Read}};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen()]
    fn prompt() -> Option<String>;
}

struct JSReader {
    buf: String
}

impl JSReader {
    fn new() -> JSReader {
        JSReader {
            buf: String::new()
        }
    }
}

impl Read for JSReader {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let s = prompt();
        if s.is_none() {
            self.buf.clear();
            return Ok(0);
        }
        self.buf = s.unwrap();
        self.buf.push('\n');
        let n = std::cmp::min(buf.len(), self.buf.len());
        buf[..n].copy_from_slice(&self.buf.as_bytes()[..n]);
        Ok(n)
    }
}

impl BufRead for JSReader {
    fn fill_buf(&mut self) -> std::io::Result<&[u8]> {
        if self.buf.is_empty() {
            let s = prompt();
            if s.is_none() {
                return Ok(&[]);
            }
            self.buf = s.unwrap();
            self.buf.push('\n');
        }
        Ok(self.buf.as_bytes())
    }
    fn consume(&mut self, amt: usize) {
        self.buf.drain(..amt);
        // self.buf = self.buf[amt..].to_string();
    }
}

#[wasm_bindgen]
pub fn boat_lang_compile(contents: String, legacy: bool, preety: bool) -> String {
    let mut program = match program_parser::parse_program(&contents) {
        Ok(program) => program,
        Err(e) => {
            // println!("{}", e);
            return format!("{e}");
        }
    };
    program_optimizer:: optimize_reassigns(&mut program);
    let mut labeled_lines = HashSet::<u32>::new();
    let translated = program_translator::translate_program(program, &mut labeled_lines);
    if legacy {
        boat_instructions::translated_to_string(translated)
    } else {
        boat_instructions::translated_to_string2(translated, preety, &labeled_lines)
    }
}

#[wasm_bindgen]
pub fn boat_lang_interpret(contents: String, debug: bool) -> String {
    let mut program = match program_parser::parse_program(&contents) {
        Ok(program) => program,
        Err(e) => {
            // println!("{}", e);
            return format!("{e}");
        }
    };
    program_optimizer::optimize_reassigns(&mut program);
    let mut labeled_lines = HashSet::<u32>::new();
    let translated = program_translator::translate_program(program, &mut labeled_lines);
    let mut out = Vec::<u8>::new();
    interpreter::interpret(&translated, &mut out, JSReader::new(), debug);
    String::from_utf8(out).expect("output is valid utf8")
}
