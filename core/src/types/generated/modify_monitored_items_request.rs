// This file was autogenerated from Opc.Ua.Types.bsd.xml
// DO NOT EDIT THIS FILE

use std::io::{Read, Write};

#[allow(unused_imports)]
use types::*;

#[derive(Debug, Clone, PartialEq)]
pub struct ModifyMonitoredItemsRequest {
    pub request_header: RequestHeader,
    pub subscription_id: UInt32,
    pub timestamps_to_return: TimestampsToReturn,
    pub items_to_modify: Option<Vec<MonitoredItemModifyRequest>>,
}

impl MessageInfo for ModifyMonitoredItemsRequest {
    fn object_id(&self) -> ObjectId {
        ObjectId::ModifyMonitoredItemsRequest_Encoding_DefaultBinary
    }
}

impl BinaryEncoder<ModifyMonitoredItemsRequest> for ModifyMonitoredItemsRequest {
    fn byte_len(&self) -> usize {
        let mut size = 0;
        size += self.request_header.byte_len();
        size += self.subscription_id.byte_len();
        size += self.timestamps_to_return.byte_len();
        size += byte_len_array(&self.items_to_modify);
        size
    }

    #[allow(unused_variables)]
    fn encode<S: Write>(&self, stream: &mut S) -> EncodingResult<usize> {
        let mut size = 0;
        size += self.request_header.encode(stream)?;
        size += self.subscription_id.encode(stream)?;
        size += self.timestamps_to_return.encode(stream)?;
        size += write_array(stream, &self.items_to_modify)?;
        Ok(size)
    }

    #[allow(unused_variables)]
    fn decode<S: Read>(stream: &mut S) -> EncodingResult<Self> {
        let request_header = RequestHeader::decode(stream)?;
        let subscription_id = UInt32::decode(stream)?;
        let timestamps_to_return = TimestampsToReturn::decode(stream)?;
        let items_to_modify: Option<Vec<MonitoredItemModifyRequest>> = read_array(stream)?;
        Ok(ModifyMonitoredItemsRequest {
            request_header: request_header,
            subscription_id: subscription_id,
            timestamps_to_return: timestamps_to_return,
            items_to_modify: items_to_modify,
        })
    }
}
