use std::io;

use serde::ser::{self};

use crate::error::Error;

pub struct Serializer<W> {
    writer: W,
}

impl<W> Serializer<W>
where
    W: io::Write,
{
    /// Creates a new NMEA serializer.
    #[inline]
    pub fn new(writer: W) -> Self {
        Serializer { writer }
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

impl<W> ser::Serializer for Serializer<W>
where
    W: io::Write,
{
    fn serialize_i128(mut self, v: i128) -> Result<Self::Ok, Self::Error> {
        write_int!(v, self.writer)
    }

    fn serialize_u128(mut self, v: u128) -> Result<Self::Ok, Self::Error> {
        write_int!(v, self.writer)
    }

    fn collect_seq<I>(self, iter: I) -> Result<Self::Ok, Self::Error>
    where
        I: IntoIterator,
        <I as IntoIterator>::Item: ser::Serialize,
    {
        let mut iter = std::iter.into_iter();
        let mut serializer = tri!(self.serialize_seq(iterator_len_hint(&iter)));
        tri!(iter.try_for_each(|item| serializer.serialize_element(&item)));
        serializer.end()
    }

    fn collect_map<K, V, I>(self, iter: I) -> Result<Self::Ok, Self::Error>
    where
        K: ser::Serialize,
        V: ser::Serialize,
        I: IntoIterator<Item = (K, V)>,
    {
        let mut iter = std::iter.into_iter();
        let mut serializer = tri!(self.serialize_map(iterator_len_hint(&iter)));
        tri!(iter.try_for_each(|(key, value)| serializer.serialize_entry(&key, &value)));
        serializer.end()
    }

    fn collect_str<T>(self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + std::fmt::Display,
    {
        self.serialize_str(&value.to_string())
    }

    fn is_human_readable(&self) -> bool {
        true
    }

    type Ok = ();

    type Error = Error;

    type SerializeSeq;

    type SerializeTuple;

    type SerializeTupleStruct;

    type SerializeTupleVariant;

    type SerializeMap;

    type SerializeStruct;

    type SerializeStructVariant;

    fn serialize_bool(mut self, v: bool) -> Result<Self::Ok, Self::Error> {
        if v {
            self.writer.write_all(b"1")
        } else {
            self.writer.write_all(b"0")
        }
        .map_err(From::from)
    }

    fn serialize_i8(mut self, v: i8) -> Result<Self::Ok, Self::Error> {
        write_int!(v, self.writer)
    }

    fn serialize_i16(mut self, v: i16) -> Result<Self::Ok, Self::Error> {
        write_int!(v, self.writer)
    }

    fn serialize_i32(mut self, v: i32) -> Result<Self::Ok, Self::Error> {
        write_int!(v, self.writer)
    }

    fn serialize_i64(mut self, v: i64) -> Result<Self::Ok, Self::Error> {
        write_int!(v, self.writer)
    }

    fn serialize_u8(mut self, v: u8) -> Result<Self::Ok, Self::Error> {
        write_int!(v, self.writer)
    }

    fn serialize_u16(mut self, v: u16) -> Result<Self::Ok, Self::Error> {
        write_int!(v, self.writer)
    }

    fn serialize_u32(mut self, v: u32) -> Result<Self::Ok, Self::Error> {
        write_int!(v, self.writer)
    }

    fn serialize_u64(mut self, v: u64) -> Result<Self::Ok, Self::Error> {
        write_int!(v, self.writer)
    }

    fn serialize_f32(mut self, v: f32) -> Result<Self::Ok, Self::Error> {
        write_float!(v, self.writer)
    }

    fn serialize_f64(mut self, v: f64) -> Result<Self::Ok, Self::Error> {
        write_float!(v, self.writer)
    }

    fn serialize_char(mut self, v: char) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_str(mut self, v: &str) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_some<T>(self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + ser::Serialize,
    {
        todo!()
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_unit_struct(self, name: &'static str) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_unit_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_newtype_struct<T>(
        self,
        name: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + ser::Serialize,
    {
        todo!()
    }

    fn serialize_newtype_variant<T>(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + ser::Serialize,
    {
        todo!()
    }

    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        todo!()
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        todo!()
    }

    fn serialize_tuple_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        todo!()
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
