// This file was autogenerated from Opc.Ua.Types.bsd.xml
// DO NOT EDIT THIS FILE

use std::io::{Read, Write};

#[allow(unused_imports)]
use types::*;

#[derive(Debug, Clone, PartialEq)]
pub struct AggregateConfiguration {
    pub use_server_capabilities_defaults: Boolean,
    pub treat_uncertain_as_bad: Boolean,
    pub percent_data_bad: Byte,
    pub percent_data_good: Byte,
    pub use_sloped_extrapolation: Boolean,
}

impl MessageInfo for AggregateConfiguration {
    fn object_id(&self) -> ObjectId {
        ObjectId::AggregateConfiguration_Encoding_DefaultBinary
    }
}

impl BinaryEncoder<AggregateConfiguration> for AggregateConfiguration {
    fn byte_len(&self) -> usize {
        let mut size = 0;
        size += self.use_server_capabilities_defaults.byte_len();
        size += self.treat_uncertain_as_bad.byte_len();
        size += self.percent_data_bad.byte_len();
        size += self.percent_data_good.byte_len();
        size += self.use_sloped_extrapolation.byte_len();
        size
    }

    #[allow(unused_variables)]
    fn encode<S: Write>(&self, stream: &mut S) -> EncodingResult<usize> {
        let mut size = 0;
        size += self.use_server_capabilities_defaults.encode(stream)?;
        size += self.treat_uncertain_as_bad.encode(stream)?;
        size += self.percent_data_bad.encode(stream)?;
        size += self.percent_data_good.encode(stream)?;
        size += self.use_sloped_extrapolation.encode(stream)?;
        Ok(size)
    }

    #[allow(unused_variables)]
    fn decode<S: Read>(stream: &mut S) -> EncodingResult<Self> {
        let use_server_capabilities_defaults = Boolean::decode(stream)?;
        let treat_uncertain_as_bad = Boolean::decode(stream)?;
        let percent_data_bad = Byte::decode(stream)?;
        let percent_data_good = Byte::decode(stream)?;
        let use_sloped_extrapolation = Boolean::decode(stream)?;
        Ok(AggregateConfiguration {
            use_server_capabilities_defaults: use_server_capabilities_defaults,
            treat_uncertain_as_bad: treat_uncertain_as_bad,
            percent_data_bad: percent_data_bad,
            percent_data_good: percent_data_good,
            use_sloped_extrapolation: use_sloped_extrapolation,
        })
    }
}
