use bytes::BytesMut;
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;
use tokio::codec::{Decoder, Encoder};

pub struct JsonLineCodec<T> {
    pd: PhantomData<T>,
}

impl<T> Default for JsonLineCodec<T> {
    fn default() -> JsonLineCodec<T> {
        JsonLineCodec { pd: PhantomData }
    }
}

impl<T: for<'a> Deserialize<'a> + std::fmt::Debug> Decoder for JsonLineCodec<T> {
    type Item = T;
    type Error = failure::Error;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<T>, Self::Error> {
        if src.is_empty() {
            Ok(None)
        } else {
            let result = match serde_json::from_reader(std::io::Cursor::new(src)) {
                Ok(t) => Ok(Some(t)),
                Err(e) => Err(e.into()),
            };
            println!("{:?}", result);
            result
        }
    }
}

impl<T: Serialize> Encoder for JsonLineCodec<T> {
    type Item = T;
    type Error = failure::Error;

    fn encode(&mut self, item: T, bytes: &mut BytesMut) -> Result<(), Self::Error> {
        let vec = serde_json::to_vec(&item)?;
        bytes.extend_from_slice(&vec);
        bytes.extend_from_slice(b"\r\n");
        Ok(())
    }
}
