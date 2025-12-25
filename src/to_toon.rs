use crate::nu_to_toon::nu_value_to_toon_string_value;
use crate::ToonPlugin;
use nu_plugin::{EngineInterface, EvaluatedCall, PluginCommand};
use nu_protocol::{Category, LabeledError, PipelineData, Signature, Type};

pub struct ToToon;

impl PluginCommand for ToToon {
    type Plugin = ToonPlugin;

    fn name(&self) -> &str {
        "to toon"
    }

    fn signature(&self) -> Signature {
        Signature::build(PluginCommand::name(self))
            .category(Category::Formats)
            .input_output_type(Type::Any, Type::String)
            .input_output_type(Type::Bool, Type::String)
            .input_output_type(Type::Binary, Type::String)
            .input_output_type(Type::Float, Type::String)
            .input_output_type(Type::Int, Type::String)
            .input_output_type(Type::String, Type::String)
    }

    fn description(&self) -> &str {
        "Convert to TOON (Token-Oriented Object Notation)"
    }

    fn run(
        &self,
        _plugin: &ToonPlugin,
        _engine: &EngineInterface,
        call: &EvaluatedCall,
        input: PipelineData,
    ) -> Result<PipelineData, LabeledError> {
        match input {
            PipelineData::Empty => Ok(PipelineData::Empty),
            PipelineData::Value(value, _pipeline_metadata) => {
                let encoded = nu_value_to_toon_string_value(&value);
                Ok(PipelineData::Value(encoded, None))
            }
            PipelineData::ListStream(list_stream, _pipeline_metadata) => {
                let values = list_stream.map(|x| nu_value_to_toon_string_value(&x));
                Ok(PipelineData::ListStream(values, None))
            }
            _ => Err(
                LabeledError::new("Can only serialize values to TOON").with_label(
                    format!("requires value input; got {}", input.get_type()),
                    call.head,
                ),
            ),
        }
    }
}
