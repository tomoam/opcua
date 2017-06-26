// This file was autogenerated from Opc.Ua.Types.bsd.xml
// DO NOT EDIT THIS FILE

use std::io::{Read, Write};

#[allow(unused_imports)]
use types::*;

#[derive(Debug, Clone, PartialEq)]
pub struct ReadValueId {
    pub node_id: NodeId,
    pub attribute_id: UInt32,
    pub index_range: UAString,
    pub data_encoding: QualifiedName,
}

impl MessageInfo for ReadValueId {
    fn object_id(&self) -> ObjectId {
        ObjectId::ReadValueId_Encoding_DefaultBinary
    }
}

impl BinaryEncoder<ReadValueId> for ReadValueId {
    fn byte_len(&self) -> usize {
        let mut size = 0;
        size += self.node_id.byte_len();
        size += self.attribute_id.byte_len();
        size += self.index_range.byte_len();
        size += self.data_encoding.byte_len();
        size
    }

    #[allow(unused_variables)]
    fn encode<S: Write>(&self, stream: &mut S) -> EncodingResult<usize> {
        let mut size = 0;
        size += self.node_id.encode(stream)?;
        size += self.attribute_id.encode(stream)?;
        size += self.index_range.encode(stream)?;
        size += self.data_encoding.encode(stream)?;
        Ok(size)
    }

    #[allow(unused_variables)]
    fn decode<S: Read>(stream: &mut S) -> EncodingResult<Self> {
        let node_id = NodeId::decode(stream)?;
        let attribute_id = UInt32::decode(stream)?;
        let index_range = UAString::decode(stream)?;
        let data_encoding = QualifiedName::decode(stream)?;
        Ok(ReadValueId {
            node_id: node_id,
            attribute_id: attribute_id,
            index_range: index_range,
            data_encoding: data_encoding,
        })
    }
}
