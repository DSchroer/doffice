use std::cmp::Ordering;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone)]
pub struct Cell {
    pub position: CellRef,
    pub delimiter: Delimiter,
    pub content: String,
}

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, Ord)]
pub struct CellRef {
    row: usize,
    column: usize,
}

impl CellRef {
    pub fn new(row: usize, column: usize) -> Self {
        CellRef { row, column }
    }

    pub fn parse(text: &str) -> Self {
        let mut row = 0;
        let mut column = 0;

        for char in text.bytes() {
            if char >= b'A' && char <= b'Z' {
                column *= 10;
                column += char - b'A';
            }

            if char >= b'0' && char <= b'9' {
                row *= 10;
                row += char - b'0' - 1;
            }
        }

        CellRef{ row: row as usize, column: column as usize }
    }

    pub fn range(start: &CellRef, end: &CellRef) -> Vec<CellRef> {
        let mut cells = Vec::new();
        for row in start.row..end.row+1 {
            for column in start.column..end.column+1 {
                cells.push(CellRef::new(row, column))
            }
        }
        cells
    }
}

impl PartialOrd<Self> for CellRef {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(if self.row < other.row {
            Ordering::Less
        } else if self.row > other.row {
            Ordering::Greater
        } else {
            self.column.cmp(&other.column)
        })
    }
}

#[derive(Debug, Clone)]
pub enum Delimiter {
    Comma,
    NewLine,
    EOF,
}

impl Cell {
    pub fn new(content: Vec<u8>, row: usize, column: usize, delimiter: Delimiter) -> Self {
        Cell {
            position: CellRef { row, column },
            delimiter,
            content: String::from_utf8(content).expect("UTF-8 format error"),
        }
    }
}

impl Display for Cell {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.content)?;
        match self.delimiter {
            Delimiter::Comma => write!(f, ","),
            Delimiter::NewLine => write!(f, "\n"),
            Delimiter::EOF => write!(f, ""),
        }
    }
}

