use crate::SearchAttributesWrapper;
use helper::payload_ext::PayloadExt;
use std::collections::BTreeMap;
use temporal_sdk::{WfContext, WfExitValue, WorkflowResult};
use temporal_sdk_core::protos::coresdk::AsJsonPayloadExt;

pub async fn example(ctx: WfContext) -> WorkflowResult<SearchAttributesWrapper> {
    let search_attributes = ctx.search_attributes();
    let custom_int = if search_attributes
        .indexed_fields
        .contains_key("CustomIntField")
    {
        search_attributes.indexed_fields["CustomIntField"].deserialize::<u32>()?
    } else {
        0u32
    };

    let iter = [
        // overwrite the existing CustomIntField: [2]
        (
            "CustomIntField".to_string(),
            (custom_int + 1).as_json_payload()?,
        ),
        // delete the existing CustomBoolField: [true]
        // TODO: check null bool is valid
        (
            "CustomBoolField".to_string(),
            None::<bool>.as_json_payload()?,
        ),
        #[allow(clippy::approx_constant)]
        // add a new value
        ("CustomDoubleField".to_string(), 3.14f32.as_json_payload()?),
    ];
    ctx.upsert_search_attributes(iter.clone());

    let mut map = BTreeMap::from(iter);
    map.extend(search_attributes.indexed_fields.clone());

    Ok(WfExitValue::Normal(SearchAttributesWrapper(map)))
}
