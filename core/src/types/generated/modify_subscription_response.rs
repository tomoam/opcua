// This file was autogenerated from Opc.Ua.Types.bsd.xml
// DO NOT EDIT THIS FILE

use std::io::{Read, Write};

#[allow(unused_imports)]
use types::*;

#[derive(Debug, Clone, PartialEq)]
pub struct ModifySubscriptionResponse {
    pub response_header: ResponseHeader,
    pub revised_publishing_interval: Double,
    pub revised_lifetime_count: UInt32,
    pub revised_max_keep_alive_count: UInt32,
}

impl MessageInfo for ModifySubscriptionResponse {
    fn object_id(&self) -> ObjectId {
        ObjectId::ModifySubscriptionResponse_Encoding_DefaultBinary
    }
}

impl BinaryEncoder<ModifySubscriptionResponse> for ModifySubscriptionResponse {
    fn byte_len(&self) -> usize {
        let mut size = 0;
        size += self.response_header.byte_len();
        size += self.revised_publishing_interval.byte_len();
        size += self.revised_lifetime_count.byte_len();
        size += self.revised_max_keep_alive_count.byte_len();
        size
    }

    #[allow(unused_variables)]
    fn encode<S: Write>(&self, stream: &mut S) -> EncodingResult<usize> {
        let mut size = 0;
        size += self.response_header.encode(stream)?;
        size += self.revised_publishing_interval.encode(stream)?;
        size += self.revised_lifetime_count.encode(stream)?;
        size += self.revised_max_keep_alive_count.encode(stream)?;
        Ok(size)
    }

    #[allow(unused_variables)]
    fn decode<S: Read>(stream: &mut S) -> EncodingResult<Self> {
        let response_header = ResponseHeader::decode(stream)?;
        let revised_publishing_interval = Double::decode(stream)?;
        let revised_lifetime_count = UInt32::decode(stream)?;
        let revised_max_keep_alive_count = UInt32::decode(stream)?;
        Ok(ModifySubscriptionResponse {
            response_header: response_header,
            revised_publishing_interval: revised_publishing_interval,
            revised_lifetime_count: revised_lifetime_count,
            revised_max_keep_alive_count: revised_max_keep_alive_count,
        })
    }
}
