use std::slice;
use stream::wire_format;

#[derive(Debug)]
pub enum UnknownValue {
    Fixed32(u32),
    Fixed64(u64),
    Varint(u64),
    LengthDelimited(Vec<u8>),
}

impl UnknownValue {
    pub fn wire_type(&self) -> wire_format::WireType {
        self.get_ref().wire_type()
    }

    pub fn get_ref<'s>(&'s self) -> UnknownValueRef<'s> {
        match *self {
            UnknownValue::Fixed32(fixed32) => UnknownValueRef::Fixed32(fixed32),
            UnknownValue::Fixed64(fixed64) => UnknownValueRef::Fixed64(fixed64),
            UnknownValue::Varint(varint) => UnknownValueRef::Varint(varint),
            UnknownValue::LengthDelimited(ref bytes) => UnknownValueRef::LengthDelimited(&bytes),
        }
    }
}

pub enum UnknownValueRef<'o> {
    Fixed32(u32),
    Fixed64(u64),
    Varint(u64),
    LengthDelimited(&'o [u8]),
}

impl<'o> UnknownValueRef<'o> {
    pub fn wire_type(&self) -> wire_format::WireType {
        match *self {
            UnknownValueRef::Fixed32(_) => wire_format::WireTypeFixed32,
            UnknownValueRef::Fixed64(_) => wire_format::WireTypeFixed64,
            UnknownValueRef::Varint(_) => wire_format::WireTypeVarint,
            UnknownValueRef::LengthDelimited(_) => wire_format::WireTypeLengthDelimited,
        }
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct UnknownValues {
    pub fixed32: Vec<u32>,
    pub fixed64: Vec<u64>,
    pub varint: Vec<u64>,
    pub length_delimited: Vec<Vec<u8>>,
}

impl UnknownValues {
    pub fn add_value(&mut self, value: UnknownValue) {
        match value {
            UnknownValue::Fixed64(fixed64) => self.fixed64.push(fixed64),
            UnknownValue::Fixed32(fixed32) => self.fixed32.push(fixed32),
            UnknownValue::Varint(varint) => self.varint.push(varint),
            UnknownValue::LengthDelimited(length_delimited) => {
                self.length_delimited.push(length_delimited)
            }
        };
    }

    pub fn iter<'s>(&'s self) -> UnknownValuesIter<'s> {
        UnknownValuesIter {
            fixed32: self.fixed32.iter(),
            fixed64: self.fixed64.iter(),
            varint: self.varint.iter(),
            length_delimited: self.length_delimited.iter(),
        }
    }
}

impl<'a> IntoIterator for &'a UnknownValues {
    type Item = UnknownValueRef<'a>;
    type IntoIter = UnknownValuesIter<'a>;

    fn into_iter(self) -> UnknownValuesIter<'a> {
        self.iter()
    }
}

pub struct UnknownValuesIter<'o> {
    fixed32: slice::Iter<'o, u32>,
    fixed64: slice::Iter<'o, u64>,
    varint: slice::Iter<'o, u64>,
    length_delimited: slice::Iter<'o, Vec<u8>>,
}

impl<'o> Iterator for UnknownValuesIter<'o> {
    type Item = UnknownValueRef<'o>;

    fn next(&mut self) -> Option<UnknownValueRef<'o>> {
        let fixed32 = self.fixed32.next();
        if fixed32.is_some() {
            return Some(UnknownValueRef::Fixed32(*fixed32.unwrap()));
        }
        let fixed64 = self.fixed64.next();
        if fixed64.is_some() {
            return Some(UnknownValueRef::Fixed64(*fixed64.unwrap()));
        }
        let varint = self.varint.next();
        if varint.is_some() {
            return Some(UnknownValueRef::Varint(*varint.unwrap()));
        }
        let length_delimited = self.length_delimited.next();
        if length_delimited.is_some() {
            return Some(UnknownValueRef::LengthDelimited(&length_delimited.unwrap()));
        }
        None
    }
}
