use std::io;

use serde::{
    ser::{
        self, SerializeSeq, SerializeStruct, SerializeStructVariant, SerializeTuple,
        SerializeTupleStruct, SerializeTupleVariant,
    },
    Serialize,
};

use crate::error::Error;

pub struct Serializer<'a> {
    buf: &'a mut [u8],
    len: usize,
    delimiter: u8,
}

impl<'a> Serializer<'a> {
    /// Creates a new NMEA serializer.
    #[inline]
    pub fn new(buf: &'a mut [u8]) -> Self {
        Serializer {
            buf,
            len: 0,
            delimiter: b',',
        }
    }

    fn push(&mut self, c: u8) -> Result<(), Error> {
        if self.len < self.buf.len() {
            self.buf[self.len] = c;
            self.len += 1;
            Ok(())
        } else {
            Err(Error::BufferFull)
        }
    }

    fn push_bytes(&mut self, val: &[u8]) -> Result<(), Error> {
        if self.len + val.len() <= self.buf.len() {
            self.buf[self.len..(self.len + val.len())].copy_from_slice(val);
            Ok(())
        } else {
            Err(Error::BufferFull)
        }
    }

    fn write_delimiter(&mut self) -> Result<(), Error> {
        self.push(self.delimiter)
    }

    #[inline]
    fn write_name(&mut self, name: &str) -> Result<(), Error> {
        if name.as_bytes().first() != Some(&b'_') {
            self.push_bytes(name.as_bytes())
        } else {
            Ok(())
        }
    }
}

macro_rules! write_int {
    ($value:expr, $self:expr) => {{
        let mut buf = ::itoa::Buffer::new();
        let s = buf.format($value);
        $self.push_bytes(s.as_bytes()).map_err(From::from)
    }};
}

macro_rules! write_float {
    ($value:expr, $self:expr) => {{
        if $value.is_finite() {
            let mut buf = ::zmij::Buffer::new();
            let s = buf.format($value);
            $self.push_bytes(s.as_bytes()).map_err(From::from)
        } else {
            Ok(())
        }
    }};
}

impl<'a: 'b, 'b> ser::Serializer for &'a mut Serializer<'b> {
    fn serialize_i128(self, v: i128) -> Result<Self::Ok, Self::Error> {
        write_int!(v, self)
    }

    fn serialize_u128(self, v: u128) -> Result<Self::Ok, Self::Error> {
        write_int!(v, self)
    }

    type Ok = ();

    type Error = Error;

    type SerializeSeq = Compound<'a, 'b>;

    type SerializeTuple = Compound<'a, 'b>;

    type SerializeTupleStruct = Compound<'a, 'b>;

    type SerializeTupleVariant = Compound<'a, 'b>;

    type SerializeMap = ser::Impossible<(), Error>;

    type SerializeStruct = Compound<'a, 'b>;

    type SerializeStructVariant = Compound<'a, 'b>;

    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
        if v {
            self.push(b'1')
        } else {
            self.push(b'0')
        }
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
        write_int!(v, self)
    }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
        write_int!(v, self)
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        write_int!(v, self)
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
        write_int!(v, self)
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        write_int!(v, self)
    }

    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
        write_int!(v, self)
    }

    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
        write_int!(v, self)
    }

    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
        write_int!(v, self)
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        write_float!(v, self)
    }

    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
        write_float!(v, self)
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        let mut buf = [0u8; 4];
        let s = v.encode_utf8(&mut buf);
        self.serialize_str(s)
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        // TODO: escape string.
        self.push_bytes(v.as_bytes())
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        self.push_bytes(v)
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }

    fn serialize_some<T>(self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + ser::Serialize,
    {
        value.serialize(self)
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok, Self::Error> {
        self.serialize_unit()
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        self.write_name(variant)
    }

    fn serialize_newtype_struct<T>(
        self,
        _name: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + ser::Serialize,
    {
        value.serialize(self)
    }

    fn serialize_newtype_variant<T>(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + ser::Serialize,
    {
        self.write_name(variant)?;
        self.write_delimiter()?;
        value.serialize(self)
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        Ok(Compound {
            ser: self,
            is_first: true,
        })
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        self.serialize_seq(Some(len))
    }

    fn serialize_tuple_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        self.write_name(name)?;
        self.write_delimiter()?;
        self.serialize_tuple(len)
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        self.write_name(variant)?;
        self.write_delimiter()?;
        self.serialize_tuple(len)
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        Err(Error::NotImplemented)
    }

    fn serialize_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        self.write_name(name)?;
        self.write_delimiter()?;
        self.serialize_tuple(len)
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        self.write_name(variant)?;
        self.write_delimiter()?;
        self.serialize_tuple(len)
    }
}

pub struct Compound<'a, 'b> {
    ser: &'a mut Serializer<'b>,
    is_first: bool,
}

impl<'a: 'b, 'b> SerializeSeq for Compound<'a, 'b> {
    type Ok = ();

    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + ser::Serialize,
    {
        if self.is_first {
            self.is_first = false;
        } else {
            self.ser.write_delimiter()?;
        }
        value.serialize(&mut *self.ser)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}
impl<'a> SerializeTuple for Compound<'a> {
    type Ok = ();

    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + ser::Serialize,
    {
        SerializeSeq::serialize_element(self, value)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        SerializeSeq::end(self)
    }
}
impl<'a> SerializeTupleStruct for Compound<'a> {
    type Ok = ();

    type Error = Error;

    fn serialize_field<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + ser::Serialize,
    {
        SerializeSeq::serialize_element(self, value)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        SerializeSeq::end(self)
    }
}
impl<'a> SerializeTupleVariant for Compound<'a> {
    type Ok = ();

    type Error = Error;

    fn serialize_field<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + ser::Serialize,
    {
        SerializeSeq::serialize_element(self, value)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        SerializeSeq::end(self)
    }
}
impl<'a> SerializeStruct for Compound<'a> {
    type Ok = ();

    type Error = Error;

    fn serialize_field<T>(&mut self, _key: &'static str, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + ser::Serialize,
    {
        SerializeSeq::serialize_element(self, value)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        SerializeSeq::end(self)
    }
}
impl<'a> SerializeStructVariant for Compound<'a> {
    type Ok = ();

    type Error = Error;

    fn serialize_field<T>(&mut self, _key: &'static str, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + ser::Serialize,
    {
        SerializeSeq::serialize_element(self, value)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        SerializeSeq::end(self)
    }
}

fn to_buf<'a, T>(value: &T, buf: &'a mut [u8]) -> Result<&'a [u8], Error>
where
    T: ?Sized + Serialize,
{
    let mut ser = Serializer::new(buf);
    value.serialize(&mut ser)?;
    todo!()
}
