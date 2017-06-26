// This file was autogenerated from Opc.Ua.Types.bsd.xml
// DO NOT EDIT THIS FILE

use std::io::{Read, Write};

#[allow(unused_imports)]
use types::*;

#[derive(Debug, Clone, PartialEq)]
pub struct HistoryEvent {
    pub events: Option<Vec<HistoryEventFieldList>>,
}

impl MessageInfo for HistoryEvent {
    fn object_id(&self) -> ObjectId {
        ObjectId::HistoryEvent_Encoding_DefaultBinary
    }
}

impl BinaryEncoder<HistoryEvent> for HistoryEvent {
    fn byte_len(&self) -> usize {
        let mut size = 0;
        size += byte_len_array(&self.events);
        size
    }

    #[allow(unused_variables)]
    fn encode<S: Write>(&self, stream: &mut S) -> EncodingResult<usize> {
        let mut size = 0;
        size += write_array(stream, &self.events)?;
        Ok(size)
    }

    #[allow(unused_variables)]
    fn decode<S: Read>(stream: &mut S) -> EncodingResult<Self> {
        let events: Option<Vec<HistoryEventFieldList>> = read_array(stream)?;
        Ok(HistoryEvent {
            events: events,
        })
    }
}
