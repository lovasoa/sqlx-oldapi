use crate::io::{BufMutExt, Encode};
use crate::postgres::io::PgBufMutExt;

pub struct SaslInitialResponse<'a> {
    pub response: &'a str,
    pub plus: bool,
}

impl Encode<'_> for SaslInitialResponse<'_> {
    fn encode_with(&self, buf: &mut Vec<u8>, _: ()) {
        buf.push(b'p');
        buf.put_length_prefixed(|buf| {
            // name of the SASL authentication mechanism that the client selected
            buf.put_str_nul(if self.plus {
                "SCRAM-SHA-256-PLUS"
            } else {
                "SCRAM-SHA-256"
            });
            let bytes = self.response.as_bytes();
            let len_i32 = i32::try_from(bytes.len()).expect("buffer too large");
            buf.extend_from_slice(&len_i32.to_be_bytes());
            buf.extend_from_slice(bytes);
        });
    }
}

pub struct SaslResponse<'a>(pub &'a str);

impl Encode<'_> for SaslResponse<'_> {
    fn encode_with(&self, buf: &mut Vec<u8>, _: ()) {
        buf.push(b'p');
        buf.put_length_prefixed(|buf| {
            buf.extend(self.0.as_bytes());
        });
    }
}
