// This file was autogenerated from Opc.Ua.Types.bsd.xml
// DO NOT EDIT THIS FILE

use std::io::{Read, Write};

#[allow(unused_imports)]
use types::*;

/// A relative path constructed from reference types and browse names.
#[derive(Debug, Clone, PartialEq)]
pub struct RelativePath {
    pub elements: Option<Vec<RelativePathElement>>,
}

impl MessageInfo for RelativePath {
    fn object_id(&self) -> ObjectId {
        ObjectId::RelativePath_Encoding_DefaultBinary
    }
}

impl BinaryEncoder<RelativePath> for RelativePath {
    fn byte_len(&self) -> usize {
        let mut size = 0;
        size += byte_len_array(&self.elements);
        size
    }

    #[allow(unused_variables)]
    fn encode<S: Write>(&self, stream: &mut S) -> EncodingResult<usize> {
        let mut size = 0;
        size += write_array(stream, &self.elements)?;
        Ok(size)
    }

    #[allow(unused_variables)]
    fn decode<S: Read>(stream: &mut S) -> EncodingResult<Self> {
        let elements: Option<Vec<RelativePathElement>> = read_array(stream)?;
        Ok(RelativePath {
            elements: elements,
        })
    }
}
