// This file was autogenerated from Opc.Ua.Types.bsd.xml
// DO NOT EDIT THIS FILE

use std::io::{Read, Write};

#[allow(unused_imports)]
use types::*;

/// A request to browse the the references from a node.
#[derive(Debug, Clone, PartialEq)]
pub struct BrowseDescription {
    pub node_id: NodeId,
    pub browse_direction: BrowseDirection,
    pub reference_type_id: NodeId,
    pub include_subtypes: Boolean,
    pub node_class_mask: UInt32,
    pub result_mask: UInt32,
}

impl MessageInfo for BrowseDescription {
    fn object_id(&self) -> ObjectId {
        ObjectId::BrowseDescription_Encoding_DefaultBinary
    }
}

impl BinaryEncoder<BrowseDescription> for BrowseDescription {
    fn byte_len(&self) -> usize {
        let mut size = 0;
        size += self.node_id.byte_len();
        size += self.browse_direction.byte_len();
        size += self.reference_type_id.byte_len();
        size += self.include_subtypes.byte_len();
        size += self.node_class_mask.byte_len();
        size += self.result_mask.byte_len();
        size
    }

    #[allow(unused_variables)]
    fn encode<S: Write>(&self, stream: &mut S) -> EncodingResult<usize> {
        let mut size = 0;
        size += self.node_id.encode(stream)?;
        size += self.browse_direction.encode(stream)?;
        size += self.reference_type_id.encode(stream)?;
        size += self.include_subtypes.encode(stream)?;
        size += self.node_class_mask.encode(stream)?;
        size += self.result_mask.encode(stream)?;
        Ok(size)
    }

    #[allow(unused_variables)]
    fn decode<S: Read>(stream: &mut S) -> EncodingResult<Self> {
        let node_id = NodeId::decode(stream)?;
        let browse_direction = BrowseDirection::decode(stream)?;
        let reference_type_id = NodeId::decode(stream)?;
        let include_subtypes = Boolean::decode(stream)?;
        let node_class_mask = UInt32::decode(stream)?;
        let result_mask = UInt32::decode(stream)?;
        Ok(BrowseDescription {
            node_id: node_id,
            browse_direction: browse_direction,
            reference_type_id: reference_type_id,
            include_subtypes: include_subtypes,
            node_class_mask: node_class_mask,
            result_mask: result_mask,
        })
    }
}
