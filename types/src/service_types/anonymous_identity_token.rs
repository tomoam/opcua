// This file was autogenerated from Opc.Ua.Types.bsd.xml by tools/schema/gen_types.js
// DO NOT EDIT THIS FILE

use std::io::{Read, Write};

use encoding::*;
#[allow(unused_imports)]
use basic_types::*;
use string::UAString;

/// A token representing an anonymous user.
#[derive(Debug, Clone, PartialEq)]
pub struct AnonymousIdentityToken {
    pub policy_id: UAString,
}

impl BinaryEncoder<AnonymousIdentityToken> for AnonymousIdentityToken {
    fn byte_len(&self) -> usize {
        let mut size = 0;
        size += self.policy_id.byte_len();
        size
    }

    #[allow(unused_variables)]
    fn encode<S: Write>(&self, stream: &mut S) -> EncodingResult<usize> {
        let mut size = 0;
        size += self.policy_id.encode(stream)?;
        Ok(size)
    }

    #[allow(unused_variables)]
    fn decode<S: Read>(stream: &mut S, decoding_limits: &DecodingLimits) -> EncodingResult<Self> {
        let policy_id = UAString::decode(stream, decoding_limits)?;
        Ok(AnonymousIdentityToken {
            policy_id,
        })
    }
}
