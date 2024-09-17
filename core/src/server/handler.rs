use std::{future::Future, pin::Pin};

use http_body_util::{combinators::BoxBody, BodyExt, Full};
use hyper::{
    body::{Bytes, Frame, Incoming},
    service::Service,
    Method, Request, Response,
};
use tokio_util::io::ReaderStream;

use crate::data::block_division::BlockDivisionInput;

use futures_util::TryStreamExt;

type HandlerError = Box<dyn std::error::Error + Send + Sync>;
type HandlerBody = BoxBody<Bytes, HandlerError>;
type HandlerResponse = Response<HandlerBody>;
type HandlerResult = Result<HandlerResponse, HandlerError>;

#[derive(Clone)]
pub struct PostHandler {}

impl Service<Request<Incoming>> for PostHandler {
    type Response = HandlerResponse;
    type Error = HandlerError;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn call(&self, request: Request<Incoming>) -> Self::Future {
        let result = Self::handle_request(request);
        Box::pin(result)
    }
}

impl PostHandler {
    pub fn new() -> PostHandler {
        PostHandler {}
    }

    async fn handle_request(request: Request<Incoming>) -> HandlerResult {
        let method = request.method().clone();
        let path = request.uri().path();
        let headers = request.headers().clone();

        println!("Path: {}", path);

        match (method, path) {
            (Method::POST, "bdd") => Self::bdd(request).await,
            (Method::POST, "echo") => Self::echo(request).await,
            (Method::GET, path) => Self::send_file(path.to_string()).await,
            _ => {
                return Ok(Self::not_found());
            }
        }
    }

    fn not_found() -> HandlerResponse {
        Response::builder()
            .status(hyper::StatusCode::NOT_FOUND)
            .body(Self::full_to_boxed_body("Resource not found."))
            .expect("Should produce response.")
    }

    fn bad_request() -> HandlerResponse {
        Response::builder()
            .status(hyper::StatusCode::BAD_REQUEST)
            .body(Self::full_to_boxed_body("Malformed request."))
            .expect("Should produce response.")
    }

    async fn send_file(path: String) -> HandlerResult {
        if path.contains("..") {
            //Reject attempts to access parent directories
            return Ok(Self::bad_request());
        } else {
            let path = ".".to_string() + path.as_str(); //need to prepend to get to this file system.
            eprintln!("Need to point this at a safe directory to avoid inappropriately exposing files in the working directory.");

            println!("Trying to open file {}", path);
            match tokio::fs::File::open(path).await {
                Ok(file) => {
                    let reader_stream: tokio_util::io::ReaderStream<tokio::fs::File> =
                        tokio_util::io::ReaderStream::new(file);
                    let boxed_body = Self::stream_to_boxed_body(reader_stream);

                    // Send response
                    let response = Response::builder()
                        .status(hyper::StatusCode::OK)
                        .body(boxed_body)
                        .unwrap();

                    Ok(response)
                }
                Err(e) => {
                    eprintln!("{:?}", e);
                    return Ok(Self::not_found());
                }
            }
        }
    }

    async fn get_request_body_as_string(
        request: Request<Incoming>,
    ) -> Result<String, HandlerError> {
        Ok(
            String::from_utf8(request.collect().await?.to_bytes().to_vec())
                .expect("Couldn't parse bytes."),
        )
    }

    fn full_to_boxed_body<T: Into<Bytes>>(chunk: T) -> HandlerBody {
        Full::new(chunk.into())
            .map_err(|never| match never {})
            .boxed()
    }

    fn stream_to_boxed_body(stream: ReaderStream<tokio::fs::File>) -> HandlerBody {
        let remapped_stream = stream.map_err(|e| match e {
            e => Box::new(e) as HandlerError,
        });
        let stream_body = http_body_util::StreamBody::new(remapped_stream.map_ok(Frame::data));
        stream_body.boxed()
    }

    async fn echo(request: Request<Incoming>) -> HandlerResult {
        let as_string = Self::get_request_body_as_string(request).await?;
        Ok(Response::new(Self::full_to_boxed_body(as_string)))
    }

    async fn bdd(request: Request<Incoming>) -> HandlerResult {
        let as_string = Self::get_request_body_as_string(request).await?;

        let bdd: BlockDivisionInput = match serde_json::from_str(&as_string) {
            Ok(data) => data,
            Err(_e) => {
                eprintln!("Could not parse package. {:?}", &as_string);
                return Ok(Response::new(Self::full_to_boxed_body("Invalid JSON")));
            }
        };

        return Ok(Response::new(Self::full_to_boxed_body(
            "Not yet implemented",
        )));
    }
}
