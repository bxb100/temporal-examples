use temporal_sdk_core::protos::coresdk::activity_result::{
    activity_resolution::Status::Completed, ActivityResolution, Success,
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
        if let Some(Completed(Success {
            result: Some(ref payload),
        })) = self.status
        {
            let result: T = serde_json::from_slice(&payload.data)?;
            return Ok(result);
        }

        Err(anyhow::anyhow!("Activity failed with {:?}", self.status))
    }
}
