// This file was autogenerated from Opc.Ua.Types.bsd.xml by tools/schema/gen_types.js
// DO NOT EDIT THIS FILE

use std::io::{Read, Write};

use encoding::*;
#[allow(unused_imports)]
use basic_types::*;
use basic_types::LocalizedText;

/// The attributes for a view node.
#[derive(Debug, Clone, PartialEq)]
pub struct ViewAttributes {
    pub specified_attributes: UInt32,
    pub display_name: LocalizedText,
    pub description: LocalizedText,
    pub write_mask: UInt32,
    pub user_write_mask: UInt32,
    pub contains_no_loops: Boolean,
    pub event_notifier: Byte,
}

impl BinaryEncoder<ViewAttributes> for ViewAttributes {
    fn byte_len(&self) -> usize {
        let mut size = 0;
        size += self.specified_attributes.byte_len();
        size += self.display_name.byte_len();
        size += self.description.byte_len();
        size += self.write_mask.byte_len();
        size += self.user_write_mask.byte_len();
        size += self.contains_no_loops.byte_len();
        size += self.event_notifier.byte_len();
        size
    }

    #[allow(unused_variables)]
    fn encode<S: Write>(&self, stream: &mut S) -> EncodingResult<usize> {
        let mut size = 0;
        size += self.specified_attributes.encode(stream)?;
        size += self.display_name.encode(stream)?;
        size += self.description.encode(stream)?;
        size += self.write_mask.encode(stream)?;
        size += self.user_write_mask.encode(stream)?;
        size += self.contains_no_loops.encode(stream)?;
        size += self.event_notifier.encode(stream)?;
        Ok(size)
    }

    #[allow(unused_variables)]
    fn decode<S: Read>(stream: &mut S, decoding_limits: &DecodingLimits) -> EncodingResult<Self> {
        let specified_attributes = UInt32::decode(stream, decoding_limits)?;
        let display_name = LocalizedText::decode(stream, decoding_limits)?;
        let description = LocalizedText::decode(stream, decoding_limits)?;
        let write_mask = UInt32::decode(stream, decoding_limits)?;
        let user_write_mask = UInt32::decode(stream, decoding_limits)?;
        let contains_no_loops = Boolean::decode(stream, decoding_limits)?;
        let event_notifier = Byte::decode(stream, decoding_limits)?;
        Ok(ViewAttributes {
            specified_attributes,
            display_name,
            description,
            write_mask,
            user_write_mask,
            contains_no_loops,
            event_notifier,
        })
    }
}
