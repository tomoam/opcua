// This file was autogenerated from Opc.Ua.Types.bsd.xml
// DO NOT EDIT THIS FILE

use std::io::{Read, Write};

#[allow(unused_imports)]
use types::*;

#[derive(Debug, Clone, PartialEq)]
pub struct ComplexNumberType {
    pub real: Float,
    pub imaginary: Float,
}

impl MessageInfo for ComplexNumberType {
    fn object_id(&self) -> ObjectId {
        ObjectId::ComplexNumberType_Encoding_DefaultBinary
    }
}

impl BinaryEncoder<ComplexNumberType> for ComplexNumberType {
    fn byte_len(&self) -> usize {
        let mut size = 0;
        size += self.real.byte_len();
        size += self.imaginary.byte_len();
        size
    }

    #[allow(unused_variables)]
    fn encode<S: Write>(&self, stream: &mut S) -> EncodingResult<usize> {
        let mut size = 0;
        size += self.real.encode(stream)?;
        size += self.imaginary.encode(stream)?;
        Ok(size)
    }

    #[allow(unused_variables)]
    fn decode<S: Read>(stream: &mut S) -> EncodingResult<Self> {
        let real = Float::decode(stream)?;
        let imaginary = Float::decode(stream)?;
        Ok(ComplexNumberType {
            real: real,
            imaginary: imaginary,
        })
    }
}
