use crate::error::Error;
use serde::ser;
use serde::Serialize;
use std::io::Write;
use std::str::from_utf8;

pub struct Serializer<W: Write> {
    output: W,
    separator: char,
    escape: char,
}

pub struct SeqSerializer<'a, W: Write> {
    serializer: &'a mut Serializer<W>,
    first: bool,
}

pub fn to_writer<T, W: Write>(output: W, value: &T) -> Result<(), Error>
where
    T: Serialize,
{
    let mut serializer = Serializer::new(output);
    value.serialize(&mut serializer)
}

impl<W: Write> Serializer<W> {
    pub fn new(output: W) -> Serializer<W> {
        Serializer {
            output: output,
            separator: crate::DEFAULT_SEPARATOR,
            escape: crate::DEFAULT_ESCAPE,
        }
    }
    fn write_value<T: AsRef<str>>(&mut self, value: T) -> Result<(), Error> {
        self.output.write_all(value.as_ref().as_bytes())?;
        Ok(())
    }
}

impl<'a, W: Write> ser::Serializer for &'a mut Serializer<W> {
    type Ok = ();

    type Error = Error;

    type SerializeSeq = SeqSerializer<'a, W>;
    type SerializeTuple = SeqSerializer<'a, W>;
    type SerializeTupleStruct = Self;
    type SerializeTupleVariant = Self;
    type SerializeMap = Self;
    type SerializeStruct = Self;
    type SerializeStructVariant = Self;

    fn serialize_bool(self, v: bool) -> Result<(), Error> {
        self.write_value(v.to_string())
    }

    fn serialize_i8(self, v: i8) -> Result<(), Error> {
        self.write_value(v.to_string())
    }

    fn serialize_i16(self, v: i16) -> Result<(), Error> {
        self.write_value(v.to_string())
    }

    fn serialize_i32(self, v: i32) -> Result<(), Error> {
        self.write_value(v.to_string())
    }

    fn serialize_i64(self, v: i64) -> Result<(), Error> {
        self.write_value(v.to_string())
    }

    fn serialize_u8(self, v: u8) -> Result<(), Error> {
        self.write_value(v.to_string())
    }

    fn serialize_u16(self, v: u16) -> Result<(), Error> {
        self.write_value(v.to_string())
    }

    fn serialize_u32(self, v: u32) -> Result<(), Error> {
        self.write_value(v.to_string())
    }

    fn serialize_u64(self, v: u64) -> Result<(), Error> {
        self.write_value(v.to_string())
    }

    fn serialize_f32(self, v: f32) -> Result<(), Error> {
        self.write_value(v.to_string())
    }

    fn serialize_f64(self, v: f64) -> Result<(), Error> {
        self.write_value(v.to_string())
    }

    fn serialize_char(self, v: char) -> Result<(), Error> {
        self.write_value(v.to_string())
    }

    fn serialize_str(self, v: &str) -> Result<(), Error> {
        let s = v.replace(self.escape, &format!("{}{}", self.escape, self.escape));
        let s = s.replace(
            self.separator,
            &format!("{}{}", self.escape, self.separator),
        );
        self.write_value(s)
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<(), Error> {
        self.write_value(from_utf8(v)?)
    }

    fn serialize_none(self) -> Result<(), Error> {
        self.serialize_unit()
    }

    fn serialize_some<T>(self, value: &T) -> Result<(), Error>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(self)
    }

    fn serialize_unit(self) -> Result<(), Error> {
        Ok(())
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<(), Error> {
        unimplemented!()
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
    ) -> Result<(), Error> {
        self.write_value(variant)
    }

    fn serialize_newtype_struct<T>(self, _name: &'static str, _value: &T) -> Result<(), Error>
    where
        T: ?Sized + Serialize,
    {
        unimplemented!()
    }

    fn serialize_newtype_variant<T>(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _value: &T,
    ) -> Result<(), Error>
    where
        T: ?Sized + Serialize,
    {
        unimplemented!()
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq, Error> {
        Ok(SeqSerializer {
            serializer: self,
            first: true,
        })
    }

    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple, Error> {
        Ok(SeqSerializer {
            serializer: self,
            first: true,
        })
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct, Error> {
        unimplemented!()
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant, Error> {
        unimplemented!()
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap, Error> {
        Ok(self)
    }

    fn serialize_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStruct, Error> {
        Ok(self)
    }

    fn serialize_struct_variant(
        self,
        name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStructVariant, Error> {
        self.serialize_struct(name, len)
    }
}

impl<'a, W: Write> ser::SerializeSeq for SeqSerializer<'a, W> {
    type Ok = ();

    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<(), Error>
    where
        T: ?Sized + Serialize,
    {
        if !self.first {
            self.serializer.write_value(",")?;
        }
        value.serialize(&mut *self.serializer)
    }

    fn end(self) -> Result<(), Error> {
        Ok(())
    }
}

impl<'a, W: Write> ser::SerializeTuple for SeqSerializer<'a, W> {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<(), Error>
    where
        T: ?Sized + Serialize,
    {
        if !self.first {
            self.serializer.write_value(",")?;
        } else {
            self.first = false;
        }
        value.serialize(&mut *self.serializer)
    }

    fn end(self) -> Result<(), Error> {
        Ok(())
    }
}

impl<'a, W: Write> ser::SerializeTupleStruct for &'a mut Serializer<W> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, _value: &T) -> Result<(), Error>
    where
        T: ?Sized + Serialize,
    {
        unimplemented!()
    }

    fn end(self) -> Result<(), Error> {
        unimplemented!()
    }
}

impl<'a, W: Write> ser::SerializeTupleVariant for &'a mut Serializer<W> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, _value: &T) -> Result<(), Error>
    where
        T: ?Sized + Serialize,
    {
        unimplemented!()
    }

    fn end(self) -> Result<(), Error> {
        unimplemented!()
    }
}

impl<'a, W: Write> ser::SerializeMap for &'a mut Serializer<W> {
    type Ok = ();
    type Error = Error;

    fn serialize_key<T>(&mut self, key: &T) -> Result<(), Error>
    where
        T: ?Sized + Serialize,
    {
        key.serialize(&mut **self)?;
        self.output
            .write_all(self.separator.to_string().as_bytes())
            .map_err(|e| Error::IO(e))
    }

    fn serialize_value<T>(&mut self, value: &T) -> Result<(), Error>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(&mut **self)?;
        self.output.write_all(&[b'\n']).map_err(|e| Error::IO(e))
    }

    fn end(self) -> Result<(), Error> {
        Ok(())
    }
}

impl<'a, W: Write> ser::SerializeStruct for &'a mut Serializer<W> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<(), Error>
    where
        T: ?Sized + Serialize,
    {
        self.write_value(key)?;
        self.output
            .write_all(self.separator.to_string().as_bytes())?;
        value.serialize(&mut **self)?;
        self.output.write_all(&[b'\n']).map_err(|e| Error::IO(e))
    }

    fn end(self) -> Result<(), Error> {
        Ok(())
    }
}

impl<'a, W: Write> ser::SerializeStructVariant for &'a mut Serializer<W> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, _key: &'static str, _value: &T) -> Result<(), Error>
    where
        T: ?Sized + Serialize,
    {
        unimplemented!()
    }

    fn end(self) -> Result<(), Error> {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;
    #[test]
    fn test() {
        #[derive(Serialize, PartialEq, Debug)]
        struct Test {
            int: u32,
        }
        let t: Test = Test{int: 10};
        let mut buf = Cursor::new(Vec::<u8>::new());
        to_writer(&mut buf, &t).unwrap();
        assert_eq!(from_utf8(buf.get_ref()).unwrap(), "int=10\n");
    }
}
