use serde::Deserialize;
use temporal_sdk_core_protos::temporal::api::common::v1::{Payload, Payloads};
use crate::payload_ext::PayloadExt;

pub trait PayloadsExt {
    fn first_input<T>(&self) -> Option<T>
    where
        T: for<'de> serde::de::Deserialize<'de>;
}

impl PayloadsExt for [Payload] {
    fn first_input<T>(&self) -> Option<T>
    where
        T: for<'de> Deserialize<'de>,
    {
        if let Some(payload) = self.first() {
            payload.deserialize().ok()
        } else {
            None
        }
    }
}

impl PayloadsExt for Vec<Payload> {
    fn first_input<T>(&self) -> Option<T>
    where
        T: for<'de> Deserialize<'de>,
    {
        self[..].first_input::<T>()
    }
}

impl PayloadsExt for Payloads {
    fn first_input<T>(&self) -> Option<T>
    where
        T: for<'de> Deserialize<'de>,
    {
        self.payloads.first_input::<T>()
    }
}
