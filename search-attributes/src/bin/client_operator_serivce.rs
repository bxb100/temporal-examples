use helper::util::client::get_client;
use std::collections::HashMap;
use temporal_client::OperatorService;
use temporal_sdk_core_protos::temporal::api::enums::v1::IndexedValueType;
use temporal_sdk_core_protos::temporal::api::operatorservice::v1::AddSearchAttributesRequest;
use tracing::info;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();

    let mut client = get_client().await?;

    let search_attributes = AddSearchAttributesRequest {
        search_attributes: HashMap::from([
            (
                "CustomKeywordField".to_string(),
                IndexedValueType::Keyword as i32,
            ),
            (
                "CustomKeywordListField".to_string(),
                IndexedValueType::KeywordList as i32,
            ),
            ("CustomBoolField".to_string(), IndexedValueType::Bool as i32),
            (
                "CustomDatetimeField".to_string(),
                IndexedValueType::Datetime as i32,
            ),
            ("CustomTextField".to_string(), IndexedValueType::Text as i32),
        ]),
        namespace: "default".to_string(),
    };

    client
        .add_search_attributes(search_attributes.clone())
        .await?;

    info!(
        "Added Search Attributes: \n{:#?} to Temporal Server",
        search_attributes
    );

    Ok(())
}
