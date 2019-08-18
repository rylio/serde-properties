extern crate serde;

mod de;
mod error;
mod ser;

pub const DEFAULT_ESCAPE: char = '\\';
pub const DEFAULT_SEPARATOR: char = '=';

pub use de::{from_buf_read, from_bytes, from_str, from_reader};
pub use ser::to_writer;

pub use error::{Error, ParseError};

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
