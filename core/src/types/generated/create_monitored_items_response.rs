// This file was autogenerated from Opc.Ua.Types.bsd.xml
// DO NOT EDIT THIS FILE

use std::io::{Read, Write};

#[allow(unused_imports)]
use types::*;

#[derive(Debug, Clone, PartialEq)]
pub struct CreateMonitoredItemsResponse {
    pub response_header: ResponseHeader,
    pub results: Option<Vec<MonitoredItemCreateResult>>,
    pub diagnostic_infos: Option<Vec<DiagnosticInfo>>,
}

impl MessageInfo for CreateMonitoredItemsResponse {
    fn object_id(&self) -> ObjectId {
        ObjectId::CreateMonitoredItemsResponse_Encoding_DefaultBinary
    }
}

impl BinaryEncoder<CreateMonitoredItemsResponse> for CreateMonitoredItemsResponse {
    fn byte_len(&self) -> usize {
        let mut size = 0;
        size += self.response_header.byte_len();
        size += byte_len_array(&self.results);
        size += byte_len_array(&self.diagnostic_infos);
        size
    }

    #[allow(unused_variables)]
    fn encode<S: Write>(&self, stream: &mut S) -> EncodingResult<usize> {
        let mut size = 0;
        size += self.response_header.encode(stream)?;
        size += write_array(stream, &self.results)?;
        size += write_array(stream, &self.diagnostic_infos)?;
        Ok(size)
    }

    #[allow(unused_variables)]
    fn decode<S: Read>(stream: &mut S) -> EncodingResult<Self> {
        let response_header = ResponseHeader::decode(stream)?;
        let results: Option<Vec<MonitoredItemCreateResult>> = read_array(stream)?;
        let diagnostic_infos: Option<Vec<DiagnosticInfo>> = read_array(stream)?;
        Ok(CreateMonitoredItemsResponse {
            response_header: response_header,
            results: results,
            diagnostic_infos: diagnostic_infos,
        })
    }
}
