mod pdk;

use base64::Engine;
use extism_pdk::*;
use pdk::types::*;
use serde_json::{Map, Value};

// Called when the tool is invoked.
pub(crate) fn call(input: CallToolRequest) -> Result<CallToolResult, Error> {
    extism_pdk::log!(
        LogLevel::Info,
        "called with args: {:?}",
        input.params.arguments
    );
    let args = input.params.arguments.unwrap_or_default();

    let data = match args.get("data") {
        Some(v) => v.as_str().unwrap(),
        None => return Err(Error::msg("`data` must be available")),
    };

    let encoded = base64::engine::general_purpose::STANDARD.encode(data);

    Ok(CallToolResult {
        is_error: None,
        content: vec![Content {
            annotations: None,
            text: Some(encoded),
            mime_type: Some("text/plain".into()),
            r#type: ContentType::Text,
            data: None,
        }],
    })
}

pub(crate) fn describe() -> Result<ListToolsResult, Error> {
    /*
    { tools: [{
        name: "base64",
        description: "base64 encode data",
        inputSchema: {
          type: "object",
          properties: {
            data: {
              type: "string",
              description: "data to convert to base64",
            },
            algorithm: {
              type: "string",
              description: "algorithm to use for hashing",
              enum: ["sha256", "sha512", "md5", "base64"],
            },
          },
          required: ["data", "algorithm"],
        }]
    }
    */
    let mut data_prop: Map<String, Value> = Map::new();
    data_prop.insert("type".into(), "string".into());
    data_prop.insert(
        "description".into(),
        "data to convert to base64".into(),
    );

    let mut props: Map<String, Value> = Map::new();
    props.insert("data".into(), data_prop.into());

    let mut schema: Map<String, Value> = Map::new();
    schema.insert("type".into(), "object".into());
    schema.insert("properties".into(), Value::Object(props));
    schema.insert("required".into(), Value::Array(vec!["data".into()]));

    Ok(ListToolsResult {
        tools: vec![ToolDescription {
            name: "hash".into(),
            description: "Hash data using various algorithms".into(),
            input_schema: schema,
        }],
    })
}
