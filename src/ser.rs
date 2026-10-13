use std::{io, marker::PhantomData};

use serde::ser::{self, SerializeMap, SerializeSeq};

use crate::error::Error;

pub struct Serializer<'a, W> {
    writer: W,
    delimiter: u8,
    _lifetime: PhantomData<&'a W>,
}

impl<'a, W> Serializer<'a, W>
where
    W: io::Write,
{
    /// Creates a new NMEA serializer.
    #[inline]
    pub fn new(writer: W) -> Self {
        Serializer {
            writer,
            delimiter: b',',
            _lifetime: PhantomData,
        }
    }

    /// Unwrap the `Writer` from the `Serializer`
    #[inline]
    pub fn into_inner(self) -> W {
        self.writer
    }
}

macro_rules! write_int {
    ($value:expr, $writer:expr) => {{
        let mut buf = ::itoa::Buffer::new();
        let s = buf.format($value);
        $writer.write_all(s.as_bytes()).map_err(From::from)
    }};
}

macro_rules! write_float {
    ($value:expr, $writer:expr) => {{
        if $value.is_finite() {
            let mut buf = ::zmij::Buffer::new();
            let s = buf.format($value);
            $writer.write_all(s.as_bytes()).map_err(From::from)
        } else {
            Ok(())
        }
    }};
}

impl<'a, W> ser::Serializer for &mut Serializer<'a, W>
where
    W: io::Write,
{
    fn serialize_i128(self, v: i128) -> Result<Self::Ok, Self::Error> {
        write_int!(v, self.writer)
    }

    fn serialize_u128(self, v: u128) -> Result<Self::Ok, Self::Error> {
        write_int!(v, self.writer)
    }

    type Ok = ();

    type Error = Error;

    type SerializeSeq = Compound<'a, W>;

    type SerializeTuple = Compound<'a, W>;

    type SerializeTupleStruct = Compound<'a, W>;

    type SerializeTupleVariant = Compound<'a, W>;

    type SerializeMap = ser::Impossible<(), Error>;

    type SerializeStruct = Compound<'a, W>;

    type SerializeStructVariant = Compound<'a, W>;

    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
        if v {
            self.writer.write_all(b"1")
        } else {
            self.writer.write_all(b"0")
        }
        .map_err(From::from)
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
        write_int!(v, self.writer)
    }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
        write_int!(v, self.writer)
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        write_int!(v, self.writer)
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
        write_int!(v, self.writer)
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        write_int!(v, self.writer)
    }

    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
        write_int!(v, self.writer)
    }

    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
        write_int!(v, self.writer)
    }

    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
        write_int!(v, self.writer)
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        write_float!(v, self.writer)
    }

    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
        write_float!(v, self.writer)
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        let mut buf = [0u8; 4];
        let s = v.encode_utf8(&mut buf);
        self.serialize_str(s)
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        // TODO: escape string.
        self.writer.write_all(v.as_bytes())?;
        Ok(())
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        self.writer.write_all(v)?;
        Ok(())
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
        self.serialize_str(variant)
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
        self.serialize_str(variant)?;
        self.writer.write_all(&[self.delimiter])?;
        value.serialize(self)
    }

    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
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
        self.serialize_str(name)?;
        self.writer.write_all(&[self.delimiter])?;
        self.serialize_tuple(len)
    }

    fn serialize_tuple_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        todo!()
    }

    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        todo!()
    }

    fn serialize_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        todo!()
    }

    fn serialize_struct_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        todo!()
    }
}

struct Compound<'a, W> {
    ser: &'a mut Serializer<W>,
    is_first: bool,
}

impl<'a, W> SerializeSeq for Compound<'a, W>
where
    W: io::Write,
{
    type Ok = ();

    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + ser::Serialize,
    {
        if self.is_first {
            self.is_first = false;
        } else {
            self.ser.writer.write_all(&[self.ser.delimiter])?;
        }
        value.serialize(self.ser)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}
