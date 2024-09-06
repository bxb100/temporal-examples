use log::debug;

use temporal_sdk_core::protos::coresdk::activity_result::{
    activity_resolution::Status::Completed, ActivityResolution,
};

pub trait ActivityResolutionExt {
    fn parse_result<'a, T>(&'a self) -> Result<T, anyhow::Error>
    where
        T: serde::Deserialize<'a>;
}

impl ActivityResolutionExt for ActivityResolution {
    fn parse_result<'a, T>(&'a self) -> Result<T, anyhow::Error>
    where
        T: serde::Deserialize<'a>,
    {
        if let Some(Completed(result)) = &self.status {
            if let Some(payload) = &result.result {
                // let data = from_utf8(&payload.data).unwrap();
                let result: T = serde_json::from_slice(&payload.data)?;
                // println!("Activity completed with: {:#?}", string_result.to_owned());
                return Ok(result);
            }
        } else {
            debug!("Activity failed with {:?}", self.status);
        }
        Err(anyhow::anyhow!("Activity failed"))
    }
}
