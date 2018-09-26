// This file was autogenerated from Opc.Ua.Types.bsd.xml by tools/schema/gen_types.js
// DO NOT EDIT THIS FILE

use std::io::{Read, Write};

use encoding::*;
#[allow(unused_imports)]
use basic_types::*;
use date_time::DateTime;
use node_id::NodeId;
use service_types::AggregateConfiguration;

#[derive(Debug, Clone, PartialEq)]
pub struct AggregateFilter {
    pub start_time: DateTime,
    pub aggregate_type: NodeId,
    pub processing_interval: Double,
    pub aggregate_configuration: AggregateConfiguration,
}

impl BinaryEncoder<AggregateFilter> for AggregateFilter {
    fn byte_len(&self) -> usize {
        let mut size = 0;
        size += self.start_time.byte_len();
        size += self.aggregate_type.byte_len();
        size += self.processing_interval.byte_len();
        size += self.aggregate_configuration.byte_len();
        size
    }

    #[allow(unused_variables)]
    fn encode<S: Write>(&self, stream: &mut S) -> EncodingResult<usize> {
        let mut size = 0;
        size += self.start_time.encode(stream)?;
        size += self.aggregate_type.encode(stream)?;
        size += self.processing_interval.encode(stream)?;
        size += self.aggregate_configuration.encode(stream)?;
        Ok(size)
    }

    #[allow(unused_variables)]
    fn decode<S: Read>(stream: &mut S, decoding_limits: &DecodingLimits) -> EncodingResult<Self> {
        let start_time = DateTime::decode(stream, decoding_limits)?;
        let aggregate_type = NodeId::decode(stream, decoding_limits)?;
        let processing_interval = Double::decode(stream, decoding_limits)?;
        let aggregate_configuration = AggregateConfiguration::decode(stream, decoding_limits)?;
        Ok(AggregateFilter {
            start_time,
            aggregate_type,
            processing_interval,
            aggregate_configuration,
        })
    }
}
