use std::io::{Bytes, Read};
use crate::calc::engine::{Cell, Delimiter};

pub struct CSVReader<T> {
    reader: Bytes<T>,
    row: usize,
    column: usize
}

impl<T> CSVReader<T> {
    pub fn new(reader: Bytes<T>) -> Self {
        CSVReader{ reader, row: 0, column: 0 }
    }
}

impl<T: Read> Iterator for CSVReader<T> {
    type Item = Cell;

    fn next(&mut self) -> Option<Self::Item> {
        let mut content = Vec::new();
        let mut escaped = false;
        loop {
            match self.reader.next() {
                Some(res) => {
                    match res {
                        Ok(byte) => {
                            match byte {
                                b'"' => {
                                    escaped = !escaped;
                                }
                                b'\n' => {
                                    let cell = Cell::new(content, self.row, self.column, Delimiter::NewLine);
                                    self.row += 1;
                                    self.column = 0;
                                    return Some(cell);
                                },
                                b',' => {
                                    if escaped {
                                        content.push(byte);
                                    }else{
                                        let cell = Cell::new(content, self.row, self.column, Delimiter::Comma);
                                        self.column += 1;
                                        return Some(cell);
                                    }
                                },
                                _ => content.push(byte),
                            }
                        },
                        Err(_) => return None
                    }
                }
                None => {
                    if !content.is_empty() {
                        return Some(Cell::new(content, self.row, self.column, Delimiter::EOF))
                    }
                    return None
                }
            }
        }
    }
}