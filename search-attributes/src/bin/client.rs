use anyhow::Result;
use chrono::Local;
use helper::client_ext::ClientExt;
use helper::util::client::get_client;
use log::info;
use nanoid::nanoid;
use search_attributes::SearchAttributesWrapper;
use std::collections::HashMap;
use temporal_client::{WorkflowClientTrait, WorkflowOptions};
use temporal_sdk_core::protos::coresdk::AsJsonPayloadExt;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();
    env_logger::init();

    let client = get_client().await?;

    let workflow_id = format!("search-attributes-example-{}", nanoid!());

    let handle = client
        .start_workflow(
            vec![],
            "search-attributes".to_string(), // task queue
            workflow_id.to_owned(),           // workflow id
            "example".to_string(),            // workflow type
            None,
            WorkflowOptions {
                search_attributes: Some(HashMap::from([
                    ("CustomIntField".to_string(), 2u32.as_json_payload()?),
                    ("CustomKeywordListField".to_string(), vec!["keywordA", "keywordB"].as_json_payload()?),
                    ("CustomBoolField".to_string(), Some(true).as_json_payload()?),
                    ("CustomDatetimeField".to_string(), Local::now().as_json_payload()?),
                    ("CustomTextField".to_string(), "String field is for text. When queried, it will be tokenized for partial match. StringTypeField cannot be used in Order By".as_json_payload()?)
                ])),
                ..Default::default()
            }
        )
        .await?;

    let describe = client
        .describe_workflow_execution(workflow_id.clone(), Some(handle.run_id.clone()))
        .await?;

    let search_attributes = describe
        .workflow_execution_info
        .unwrap()
        .search_attributes
        .unwrap()
        .indexed_fields;

    info!(
        "searchAttributes at start: {}",
        SearchAttributesWrapper::from(search_attributes)
    );

    let res = client
        .get_workflow_result::<SearchAttributesWrapper>(workflow_id, handle.run_id)
        .await?;

    info!("searchAttributes at end: {}", res);

    Ok(())
}
