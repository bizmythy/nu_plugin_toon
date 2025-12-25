use crate::toon_to_nu::toon_to_nu_value;
use crate::ToonPlugin;
use nu_plugin::{EngineInterface, EvaluatedCall, PluginCommand};
use nu_protocol::{Category, LabeledError, PipelineData, Reader, Signature, Type, Value};
use std::io::Read;

pub struct FromToon;

impl PluginCommand for FromToon {
    type Plugin = ToonPlugin;

    fn name(&self) -> &str {
        "from toon"
    }

    fn signature(&self) -> Signature {
        Signature::build(PluginCommand::name(self))
            .category(Category::Formats)
            .input_output_type(Type::String, Type::Any)
            .input_output_type(Type::Binary, Type::Any)
    }

    fn description(&self) -> &str {
        "Convert from TOON (Token-Oriented Object Notation)"
    }

    fn run(
        &self,
        _plugin: &Self::Plugin,
        _engine: &EngineInterface,
        call: &EvaluatedCall,
        input: PipelineData,
    ) -> Result<PipelineData, LabeledError> {
        match input {
            PipelineData::Empty => Ok(input),
            PipelineData::ByteStream(byte_stream, _pipeline_metadata) => {
                let mut reader: Reader = byte_stream.reader().map_err(|e| {
                    LabeledError::new("Failed to read TOON")
                        .with_label(format!("byte stream error: {}", e), call.head)
                })?;

                let mut buffer = Vec::new();
                reader.read_to_end(&mut buffer).map_err(|e| {
                    LabeledError::new("Failed to read TOON")
                        .with_label(format!("byte stream error: {}", e), call.head)
                })?;

                let text = String::from_utf8(buffer).map_err(|e| {
                    LabeledError::new("Failed to parse TOON")
                        .with_label(format!("invalid UTF-8: {}", e), call.head)
                })?;

                let value = toon_to_nu_value(&text).map_err(|e| {
                    LabeledError::new("Failed to parse TOON")
                        .with_label(format!("TOON parsing error: {}", e), call.head)
                })?;
                Ok(PipelineData::Value(value, _pipeline_metadata))
            }
            PipelineData::Value(value, _pipeline_metadata) => match value {
                Value::String { val, .. } => {
                    let value = toon_to_nu_value(&val).map_err(|e| {
                        LabeledError::new("Failed to parse TOON")
                            .with_label(format!("TOON parsing error: {}", e), call.head)
                    })?;
                    Ok(PipelineData::Value(value, _pipeline_metadata))
                }
                Value::Binary { val, .. } => {
                    let text = String::from_utf8(val).map_err(|e| {
                        LabeledError::new("Failed to parse TOON")
                            .with_label(format!("invalid UTF-8: {}", e), call.head)
                    })?;
                    let value = toon_to_nu_value(&text).map_err(|e| {
                        LabeledError::new("Failed to parse TOON")
                            .with_label(format!("TOON parsing error: {}", e), call.head)
                    })?;
                    Ok(PipelineData::Value(value, _pipeline_metadata))
                }
                _ => Err(
                    LabeledError::new("Can only parse string or binary value as TOON").with_label(
                        format!("requires string or binary input; got {}", value.get_type()),
                        call.head,
                    ),
                ),
            },
            _ => Err(
                LabeledError::new("Can only parse byte stream as TOON").with_label(
                    format!("requires string or binary input; got {}", input.get_type()),
                    call.head,
                ),
            ),
        }
    }
}
