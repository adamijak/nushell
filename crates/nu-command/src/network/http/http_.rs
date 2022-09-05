use nu_engine::get_full_help;
use nu_protocol::{
    ast::Call,
    engine::{Command, EngineState, Stack},
    Category, Example, IntoPipelineData, PipelineData, Signature, Span, Value,
};

#[derive(Clone)]
pub struct Http;

impl Command for Http {
    fn name(&self) -> &str {
        "http"
    }

    fn signature(&self) -> Signature {
        Signature::build("http").category(Category::Network)
    }

    fn usage(&self) -> &str {
        "Make HTTP request."
    }

    fn search_terms(&self) -> Vec<&str> {
        vec![
            "network", "http", "get", "post", "request", "web", "api", "rest",
        ]
    }

    fn run(
        &self,
        engine_state: &EngineState,
        stack: &mut Stack,
        call: &Call,
        _input: PipelineData,
    ) -> Result<nu_protocol::PipelineData, nu_protocol::ShellError> {
        Ok(Value::String {
            val: get_full_help(&Http.signature(), &Http.examples(), engine_state, stack),
            span: call.head,
        }
        .into_pipeline_data())
    }

    fn examples(&self) -> Vec<nu_protocol::Example> {
        vec![Example {
            description: "Make HTTP GET",
            example: "http get https://www.nushell.sh",
            result: Some(Value::String {
                val: "todo result".to_string(),
                span: Span::test_data(),
            }),
        }]
    }
}
