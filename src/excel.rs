use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

use calamine::{Excel, Range, DataType, Result};

pub fn excel_to_csv(destination: &Path, source: &Path, worksheet: &str) {
    let mut destination = BufWriter::new(File::create(destination).unwrap());
    let mut xl = Excel::open(source).unwrap();
    let range = xl.worksheet_range(worksheet).unwrap();
    write_range(&mut destination, range).unwrap();
}

fn write_range<W: Write>(dest: &mut W, range: Range) -> Result<()> {
    let n = range.get_size().1 - 1;
    for r in range.rows() {
        for (i, c) in r.iter().enumerate() {
            let _ = try!(match *c {
                DataType::Empty => Ok(()),
                DataType::String(ref s) => write!(dest, "{}", s),
                DataType::Float(ref f) => write!(dest, "{}", f),
                DataType::Int(ref i) => write!(dest, "{}", i),
                DataType::Error(ref e) => write!(dest, "{:?}", e),
                DataType::Bool(ref b) => write!(dest, "{}", b),
            });
            if i != n {
                let _ = try!(write!(dest, ";"));
            }
        }
        let _ = try!(write!(dest, "\r\n"));
    }
    Ok(())
}
