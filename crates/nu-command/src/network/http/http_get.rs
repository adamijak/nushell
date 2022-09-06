use std::io::BufReader;

use nu_engine::CallExt;
use nu_protocol::{
    ast::Call,
    engine::{Command, EngineState, Stack},
    Category, PipelineData, RawStream, ShellError, Signature, Span, SyntaxShape, Value,
};
use reqwest::{
    blocking::{Client, Request, RequestBuilder, Response},
    Method,
};

use crate::BufferedReader;

#[derive(Clone)]
pub struct SubCommand;

impl Command for SubCommand {
    fn name(&self) -> &str {
        "http get"
    }

    fn signature(&self) -> nu_protocol::Signature {
        Signature::build("http get")
            .required("URL", SyntaxShape::String, "Request URL.")
            .filter()
            .category(Category::Network)
    }

    fn usage(&self) -> &str {
        "Make HTTP GET request."
    }

    fn run(
        &self,
        engine_state: &nu_protocol::engine::EngineState,
        stack: &mut nu_protocol::engine::Stack,
        call: &nu_protocol::ast::Call,
        input: nu_protocol::PipelineData,
    ) -> Result<nu_protocol::PipelineData, nu_protocol::ShellError> {
        run(engine_state, stack, call, input)
    }

    fn examples(&self) -> Vec<nu_protocol::Example> {
        Vec::new()
    }

    fn search_terms(&self) -> Vec<&str> {
        vec![]
    }
}

fn run(
    engine_state: &EngineState,
    stack: &mut Stack,
    call: &Call,
    input: PipelineData,
) -> Result<PipelineData, ShellError> {
    let uri: Option<Value> = Some(call.req(engine_state, stack, 0)?);

    let url_value = if let Some(val) = uri {
        val
    } else {
        return Err(ShellError::UnsupportedInput(
            "Expecting a url as a string but got nothing".to_string(),
            call.head,
        ));
    };
    let requested_url = url_value.as_string()?;
    let url = match url::Url::parse(&requested_url) {
        Ok(u) => u,
        Err(_e) => {
            return Err(ShellError::UnsupportedInput(
                "Incomplete or incorrect url. Expected a full url, e.g., https://www.example.com"
                    .to_string(),
                url_value.span()?,
            ));
        }
    };

    let client = Client::builder().user_agent("nushell").build().unwrap();
    let resp = client.request(Method::GET, url).send().unwrap();

    Ok(response_to_buffer(resp, &engine_state, call.span()))
}

fn response_to_buffer(
    response: Response,
    engine_state: &EngineState,
    span: Span,
) -> nu_protocol::PipelineData {
    let buffered_input = BufReader::new(response);

    PipelineData::ExternalStream {
        stdout: Some(RawStream::new(
            Box::new(BufferedReader {
                input: buffered_input,
            }),
            engine_state.ctrlc.clone(),
            span,
        )),
        stderr: None,
        exit_code: None,
        span,
        metadata: None,
    }
}
