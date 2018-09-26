// This file was autogenerated from Opc.Ua.Types.bsd.xml by tools/schema/gen_types.js
// DO NOT EDIT THIS FILE

use std::io::{Read, Write};

use encoding::*;
#[allow(unused_imports)]
use basic_types::*;
use node_id::NodeId;
use byte_string::ByteString;

#[derive(Debug, Clone, PartialEq)]
pub struct DeleteEventDetails {
    pub node_id: NodeId,
    pub event_ids: Option<Vec<ByteString>>,
}

impl BinaryEncoder<DeleteEventDetails> for DeleteEventDetails {
    fn byte_len(&self) -> usize {
        let mut size = 0;
        size += self.node_id.byte_len();
        size += byte_len_array(&self.event_ids);
        size
    }

    #[allow(unused_variables)]
    fn encode<S: Write>(&self, stream: &mut S) -> EncodingResult<usize> {
        let mut size = 0;
        size += self.node_id.encode(stream)?;
        size += write_array(stream, &self.event_ids)?;
        Ok(size)
    }

    #[allow(unused_variables)]
    fn decode<S: Read>(stream: &mut S, decoding_limits: &DecodingLimits) -> EncodingResult<Self> {
        let node_id = NodeId::decode(stream, decoding_limits)?;
        let event_ids: Option<Vec<ByteString>> = read_array(stream, decoding_limits)?;
        Ok(DeleteEventDetails {
            node_id,
            event_ids,
        })
    }
}
