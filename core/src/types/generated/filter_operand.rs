// This file was autogenerated from Opc.Ua.Types.bsd.xml
// DO NOT EDIT THIS FILE

use std::io::{Read, Write};

#[allow(unused_imports)]
use types::*;

#[derive(Debug, Clone, PartialEq)]
pub struct FilterOperand {
}

impl MessageInfo for FilterOperand {
    fn object_id(&self) -> ObjectId {
        ObjectId::FilterOperand_Encoding_DefaultBinary
    }
}

impl BinaryEncoder<FilterOperand> for FilterOperand {
    fn byte_len(&self) -> usize {
        0
    }

    #[allow(unused_variables)]
    fn encode<S: Write>(&self, stream: &mut S) -> EncodingResult<usize> {
        Ok(0)
    }

    #[allow(unused_variables)]
    fn decode<S: Read>(stream: &mut S) -> EncodingResult<Self> {
        Ok(FilterOperand {
        })
    }
}
