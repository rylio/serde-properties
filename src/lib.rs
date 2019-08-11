extern crate serde;

mod de;
mod error;
mod ser;

pub const DEFAULT_ESCAPE: char = '\\';
pub const DEFAULT_SEPARATOR: char = '=';

pub use de::from_buf_read;
pub use de::from_bytes;
pub use de::from_str;

pub use ser::to_writer;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
