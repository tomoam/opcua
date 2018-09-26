// This file was autogenerated from Opc.Ua.Types.bsd.xml by tools/schema/gen_types.js
// DO NOT EDIT THIS FILE

use std::io::{Read, Write};

use encoding::*;
#[allow(unused_imports)]
use basic_types::*;
use service_types::impls::MessageInfo;
use node_ids::ObjectId;
use node_id::NodeId;
use string::UAString;
use basic_types::QualifiedName;

#[derive(Debug, Clone, PartialEq, Serialize)]
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
    fn decode<S: Read>(stream: &mut S, decoding_limits: &DecodingLimits) -> EncodingResult<Self> {
        let node_id = NodeId::decode(stream, decoding_limits)?;
        let attribute_id = UInt32::decode(stream, decoding_limits)?;
        let index_range = UAString::decode(stream, decoding_limits)?;
        let data_encoding = QualifiedName::decode(stream, decoding_limits)?;
        Ok(ReadValueId {
            node_id,
            attribute_id,
            index_range,
            data_encoding,
        })
    }
}
