use crate::error::{Error, ParseError};
use serde::de::{self, DeserializeSeed, MapAccess, SeqAccess, Visitor};
use serde::Deserialize;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

pub struct Deserializer<B: BufRead> {
    input: B,
    current_key: Option<String>,
    current_value: Option<String>,
    escape: char,
    separator: char,
}

pub fn from_str<'a, T: Deserialize<'a>>(s: &'a str) -> Result<T, Error> {
    from_bytes(s.as_bytes())
}

pub fn from_bytes<'a, T: Deserialize<'a>>(b: &'a [u8]) -> Result<T, Error> {
    from_buf_read(BufReader::new(b))
}

pub fn from_buf_read<'a, T: Deserialize<'a>, B: BufRead + 'a>(b: B) -> Result<T, Error> {
    let mut deserializer = Deserializer::new(b);
    T::deserialize(&mut deserializer)
}

impl<'de, B: BufRead> Deserializer<B> {
    pub fn new(input: B) -> Self {
        Deserializer {
            input: input,
            current_key: None,
            current_value: None,
            escape: crate::DEFAULT_ESCAPE,
            separator: crate::DEFAULT_SEPARATOR,
        }
    }

    fn parse_line<'b>(&mut self, l: &'b str) -> Result<(&'b str, &'b str), ParseError> {
        let mut key: Option<usize> = None;
        let mut value: Option<usize> = None;
        let mut escaped = false;
        for c in l.chars() {
            if !escaped && c == self.escape {
                escaped = true;
            } else {
                if !escaped && c == self.separator {
                    if key.is_none() {
                        return Err(ParseError::NoKey);
                    }
                    value = Some(key.unwrap() + 1);
                    break;
                } else {
                    if let Some(key) = key.as_mut() {
                        *key += c.len_utf8();
                    } else {
                        key = Some(c.len_utf8());
                    }
                }
            }
            if escaped {
                escaped = false;
            }
        }
        if value.is_none() {
            return Err(ParseError::NoValue);
        }
        unsafe {
            Ok((
                l.get_unchecked(..key.unwrap()).trim(),
                l.get_unchecked(value.unwrap()..).trim(),
            ))
        }
    }

    fn parse<U: FromStr>(value: &str) -> Option<U> {
        value.parse().map(|v| Some(v)).unwrap_or(None)
    }

    fn deserialize<T: FromStr>(v: Option<&str>) -> Result<T, Error> {
        Deserializer::<B>::parse(v.as_ref().ok_or(ParseError::NoValue)?)
            .ok_or(Error::Parse(ParseError::InvalidValue))
    }
}

impl<'de, B: BufRead> de::Deserializer<'de> for &mut Deserializer<B> {
    type Error = Error;
    fn deserialize_any<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        if self.current_key.is_some() {
            self.deserialize_str(visitor)
        } else if let Some(value) = self.current_value.as_ref() {
            if let Some(v) = Deserializer::<B>::parse(value) {
                visitor.visit_bool(v)
            } else if let Some(v) = Deserializer::<B>::parse(value) {
                visitor.visit_u64(v)
            } else if let Some(v) = Deserializer::<B>::parse(value) {
                visitor.visit_i64(v)
            } else if value.is_empty() {
                self.deserialize_unit(visitor)
            } else {
                self.deserialize_str(visitor)
            }
        } else {
            Err(Error::Parse(ParseError::NoValue))
        }
    }

    fn deserialize_bool<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        visitor.visit_bool(Deserializer::<B>::deserialize(
            self.current_value.as_ref().map(|x| &**x),
        )?)
    }

    fn deserialize_i8<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        visitor.visit_i8(Deserializer::<B>::deserialize(
            self.current_value.as_ref().map(|x| &**x),
        )?)
    }

    fn deserialize_i16<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        visitor.visit_i16(Deserializer::<B>::deserialize(
            self.current_value.as_ref().map(|x| &**x),
        )?)
    }

    fn deserialize_i32<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        visitor.visit_i32(Deserializer::<B>::deserialize(
            self.current_value.as_ref().map(|x| &**x),
        )?)
    }

    fn deserialize_i64<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        visitor.visit_i64(Deserializer::<B>::deserialize(
            self.current_value.as_ref().map(|x| &**x),
        )?)
    }

    fn deserialize_u8<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        visitor.visit_u8(Deserializer::<B>::deserialize(
            self.current_value.as_ref().map(|x| &**x),
        )?)
    }

    fn deserialize_u16<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        visitor.visit_u16(Deserializer::<B>::deserialize(
            self.current_value.as_ref().map(|x| &**x),
        )?)
    }

    fn deserialize_u32<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        visitor.visit_u32(Deserializer::<B>::deserialize(
            self.current_value.as_ref().map(|x| &**x),
        )?)
    }

    fn deserialize_u64<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        visitor.visit_u64(Deserializer::<B>::deserialize(
            self.current_value.as_ref().map(|x| &**x),
        )?)
    }

    fn deserialize_f32<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        visitor.visit_f32(Deserializer::<B>::deserialize(
            self.current_value.as_ref().map(|x| &**x),
        )?)
    }

    fn deserialize_f64<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        visitor.visit_f64(Deserializer::<B>::deserialize(
            self.current_value.as_ref().map(|x| &**x),
        )?)
    }

    fn deserialize_char<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        visitor.visit_char(Deserializer::<B>::deserialize(
            self.current_value.as_ref().map(|x| &**x),
        )?)
    }

    fn deserialize_str<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        if let Some(key) = self.current_key.as_ref() {
            visitor.visit_str(key)
        } else if let Some(value) = self.current_value.as_ref() {
            visitor.visit_str(value)
        } else {
            Err(Error::Custom("No key or value".to_string()))
        }
    }

    fn deserialize_string<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        self.deserialize_str(visitor)
    }

    fn deserialize_bytes<V: Visitor<'de>>(self, _visitor: V) -> Result<V::Value, Self::Error> {
        unimplemented!()
    }

    fn deserialize_byte_buf<V: Visitor<'de>>(self, _visitor: V) -> Result<V::Value, Self::Error> {
        unimplemented!()
    }

    fn deserialize_option<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        let v = self.current_value.as_ref().ok_or(ParseError::NoValue)?;
        if v.is_empty() {
            visitor.visit_none()
        } else {
            visitor.visit_some(self)
        }
    }

    fn deserialize_unit<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        let v = self.current_value.as_ref().ok_or(ParseError::NoValue)?;
        if !v.is_empty() {
            Err(Error::Parse(ParseError::InvalidValue))
        } else {
            visitor.visit_unit()
        }
    }

    fn deserialize_unit_struct<V: Visitor<'de>>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        self.deserialize_unit(visitor)
    }

    fn deserialize_newtype_struct<V: Visitor<'de>>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        visitor.visit_newtype_struct(self)
    }

    fn deserialize_seq<V: Visitor<'de>>(mut self, visitor: V) -> Result<V::Value, Self::Error> {
        
        let values = self.current_value.as_ref().ok_or(ParseError::NoValue)?.split(",").map(|s|s.to_string()).collect::<Vec<String>>();
        visitor.visit_seq(SeqDeserializer::new(&mut self, values))
    }

    fn deserialize_tuple<V: Visitor<'de>>(
        self,
        _len: usize,
        _visitor: V,
    ) -> Result<V::Value, Self::Error> {
        unimplemented!()
    }

    fn deserialize_tuple_struct<V: Visitor<'de>>(
        self,
        _name: &'static str,
        _len: usize,
        _visitor: V,
    ) -> Result<V::Value, Self::Error> {
        unimplemented!()
    }

    fn deserialize_map<V: Visitor<'de>>(mut self, visitor: V) -> Result<V::Value, Self::Error> {
        if self.current_value.is_some() || self.current_key.is_some() {
            return Err(Error::Custom("Nested maps or structs not supported".to_string()));
        }
        visitor.visit_map(&mut self)
    }

    fn deserialize_struct<V: Visitor<'de>>(
        self,
        _name: &'static str,
        _fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        self.deserialize_map(visitor)
    }

    fn deserialize_enum<V: Visitor<'de>>(
        self,
        _name: &'static str,
        _variants: &'static [&'static str],
        _visitor: V,
    ) -> Result<V::Value, Self::Error> {
        unimplemented!()
    }

    fn deserialize_identifier<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        self.deserialize_str(visitor)
    }

    fn deserialize_ignored_any<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        self.deserialize_any(visitor)
    }
}

impl<'de, B: BufRead> MapAccess<'de> for &mut Deserializer<B> {
    type Error = Error;

    fn next_key_seed<K: DeserializeSeed<'de>>(
        &mut self,
        seed: K,
    ) -> Result<Option<K::Value>, Self::Error> {
        let mut buf = Box::new(String::new());
        let n = self.input.read_line(&mut buf)?;
        if n == 0 {
            return Ok(None);
        }
        let (k, v) = self.parse_line(&buf)?;
        self.current_key = Some(k.to_string());
        self.current_value = Some(v.to_string());
        let ret = seed.deserialize(&mut **self).map(Some);
        self.current_key = None;
        ret
    }

    fn next_value_seed<V: DeserializeSeed<'de>>(
        &mut self,
        seed: V,
    ) -> Result<V::Value, Self::Error> {
        let ret = seed.deserialize(&mut **self);
        self.current_value = None;
        ret
    }
}

struct SeqDeserializer<'a, B: BufRead> {
    deserializer: &'a mut Deserializer<B>,
    index: usize,
    values: Vec<String>
}

impl<'a, 'de, B: BufRead> SeqDeserializer<'a, B> {
    fn new(deserializer: &'a mut Deserializer<B>, values: Vec<String>) -> Self {
        SeqDeserializer{
            deserializer: deserializer,
            index: 0,
            values: values
        }
    }
}

impl<'de, 'a, B: BufRead> SeqAccess<'de> for SeqDeserializer<'a, B> {
    type Error = Error;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Error>
    where
        T: DeserializeSeed<'de>,
    {
        if self.index >= self.values.len() {
            Ok(None)
        } else {
            self.deserializer.current_value = Some(self.values[self.index].to_string());
            self.index += 1;
            seed.deserialize(&mut *self.deserializer).map(Some)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        #[derive(Deserialize, PartialEq, Debug)]
        struct Test {
            int: u32,
        }
        let t: Test = from_str(r#"int=1"#).unwrap();
        assert_eq!(t.int, 1);
    }
}
