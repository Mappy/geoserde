use serde::{
    ser::{
        SerializeMap, SerializeSeq, SerializeStruct, SerializeStructVariant, SerializeTuple,
        SerializeTupleStruct, SerializeTupleVariant,
    },
    Serialize, Serializer,
};

use super::err::SerializeError;

// pub trait PropertySink: geozero::GeomProcessor {}
// impl<G: geozero::GeomProcessor> PropertySink for G {}
pub trait PropertySink {
    type Error: std::error::Error;
    fn i32(&mut self, index: usize, key: &str, value: i32) -> Result<(), Self::Error>;
    fn u32(&mut self, index: usize, key: &str, value: u32) -> Result<(), Self::Error>;
    fn f64(&mut self, index: usize, key: &str, value: f64) -> Result<(), Self::Error>;
}
#[cfg(feature = "geozero")]
impl<G: geozero::PropertyProcessor> PropertySink for G {
    type Error = geozero::error::GeozeroError;
    fn i32(&mut self, index: usize, key: &str, value: i32) -> Result<(), Self::Error> {
        let _abort = self.property(index, key, &geozero::ColumnValue::Int(value))?;
        Ok(())
    }
    fn u32(&mut self, index: usize, key: &str, value: u32) -> Result<(), Self::Error> {
        let _abort = self.property(index, key, &geozero::ColumnValue::UInt(value))?;
        Ok(())
    }
    fn f64(&mut self, index: usize, key: &str, value: f64) -> Result<(), Self::Error> {
        let _abort = self.property(index, key, &geozero::ColumnValue::Double(value))?;
        Ok(())
    }
}
pub struct PropertySerializer<'a, S> {
    index: usize,
    key: &'static str,
    sink: &'a mut S,
}
impl<'a, S> PropertySerializer<'a, S> {
    pub fn new(index: usize, key: &'static str, sink: &'a mut S) -> Self {
        Self { index, key, sink }
    }
}
impl<S: PropertySink> Serializer for &mut PropertySerializer<'_, S> {
    type Ok = usize;
    type Error = SerializeError<S::Error>;
    type SerializeSeq = Self;
    type SerializeTuple = Self;
    type SerializeTupleStruct = Self;
    type SerializeTupleVariant = Self;
    type SerializeMap = Self;
    type SerializeStruct = Self;
    type SerializeStructVariant = Self;

    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
        todo!()
    }
    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
        todo!()
    }
    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
        todo!()
    }
    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        self.sink
            .i32(self.index, self.key, v)
            .map_err(SerializeError::PropertySinkCaused)?;
        self.index += 1;
        Ok(self.index)
    }
    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
        todo!()
    }
    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        todo!()
    }
    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
        todo!()
    }
    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
        todo!()
    }
    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
        todo!()
    }
    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        todo!()
    }
    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
        self.sink
            .f64(self.index, self.key, v)
            .map_err(SerializeError::PropertySinkCaused)?;
        self.index += 1;
        Ok(self.index)
    }
    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        todo!()
    }
    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        todo!()
    }
    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        todo!()
    }
    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        todo!()
    }
    fn serialize_some<T: ?Sized>(self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: serde::Serialize,
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

    fn serialize_newtype_struct<T: ?Sized>(
        self,
        name: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: serde::Serialize,
    {
        todo!()
    }

    fn serialize_newtype_variant<T: ?Sized>(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: serde::Serialize,
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
        // flatten the struct
        Ok(self)
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

impl<S: PropertySink> SerializeSeq for &mut PropertySerializer<'_, S> {
    type Ok = usize;
    type Error = SerializeError<S::Error>;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        panic!();
        value.serialize(&mut **self)?;
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(self.index)
    }
}

impl<S: PropertySink> SerializeTuple for &mut PropertySerializer<'_, S> {
    type Ok = usize;
    type Error = SerializeError<S::Error>;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        todo!()
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        todo!()
    }
}

impl<S: PropertySink> SerializeTupleStruct for &mut PropertySerializer<'_, S> {
    type Ok = usize;
    type Error = SerializeError<S::Error>;

    fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        todo!()
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        todo!()
    }
}

impl<S: PropertySink> SerializeTupleVariant for &mut PropertySerializer<'_, S> {
    type Ok = usize;
    type Error = SerializeError<S::Error>;

    fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        todo!()
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        todo!()
    }
}

impl<S: PropertySink> SerializeMap for &mut PropertySerializer<'_, S> {
    type Ok = usize;
    type Error = SerializeError<S::Error>;

    fn serialize_key<T: ?Sized>(&mut self, key: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        todo!()
    }

    fn serialize_value<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        todo!()
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        todo!()
    }
}

impl<S: PropertySink> SerializeStruct for &mut PropertySerializer<'_, S> {
    type Ok = usize;
    type Error = SerializeError<S::Error>;

    fn serialize_field<T: ?Sized>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        self.key = key;
        value.serialize(&mut **self)?;
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(self.index)
    }
}

impl<S: PropertySink> SerializeStructVariant for &mut PropertySerializer<'_, S> {
    type Ok = usize;
    type Error = SerializeError<S::Error>;

    fn serialize_field<T: ?Sized>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        todo!()
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        todo!()
    }
}
