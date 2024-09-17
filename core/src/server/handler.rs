use std::{future::Future, pin::Pin};

use http_body_util::{combinators::BoxBody, BodyExt, Either, Empty, Full};
use hyper::{
    body::{Body, Bytes, Frame, Incoming},
    service::Service,
    Method, Request, Response,
};
use serde::{Deserialize, Serialize};

use crate::data::block_division::BlockDivisionInput;

use futures_util::TryStreamExt;

type HandlerResponse = Response<BoxBody<Bytes, std::io::Error>>;

#[derive(Clone)]
pub struct PostHandler {}

impl Service<Request<Incoming>> for PostHandler {
    type Response = HandlerResponse;
    type Error = Box<dyn std::error::Error + Send + Sync>;
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

    async fn handle_request(
        request: Request<Incoming>,
    ) -> Result<HandlerResponse, Box<dyn std::error::Error + Send + Sync>> {
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

    fn not_found() -> Response<Full<Bytes>> {
        Response::builder()
            .status(hyper::StatusCode::NOT_FOUND)
            .body(Full::new(Bytes::from("Resource not found.")))
            .expect("Should produce response.")
    }

    fn bad_request() -> Response<Full<Bytes>> {
        Response::builder()
            .status(hyper::StatusCode::BAD_REQUEST)
            .body(Full::new(Bytes::from("Malformed request.")))
            .expect("Should produce response.")
    }

    async fn send_file(
        path: String,
    ) -> Result<HandlerResponse, Box<dyn std::error::Error + Send + Sync>> {
        if path.contains("..") {
            //Reject attempts to access parent directories
            return Ok(Self::bad_request());
        } else {
            match tokio::fs::File::open(path).await {
                Ok(file) => {
                    let reader_stream = tokio_util::io::ReaderStream::new(file);

                    // Convert to http_body_util::BoxBody
                    let stream_body =
                        http_body_util::StreamBody::new(reader_stream.map_ok(Frame::data));
                    let boxed_body = stream_body.boxed();

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
    ) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        Ok(
            String::from_utf8(request.collect().await?.to_bytes().to_vec())
                .expect("Couldn't parse bytes."),
        )
    }

    async fn echo(
        request: Request<Incoming>,
    ) -> Result<HandlerResponse, Box<dyn std::error::Error + Send + Sync>> {
        let as_string = Self::get_request_body_as_string(request).await?;
        Ok(Response::new(BoxBody::new(Bytes::from(as_string))))
    }

    async fn bdd(
        request: Request<Incoming>,
    ) -> Result<HandlerResponse, Box<dyn std::error::Error + Send + Sync>> {
        let as_string = Self::get_request_body_as_string(request).await?;

        let bdd: BlockDivisionInput = match serde_json::from_str(&as_string) {
            Ok(data) => data,
            Err(_e) => {
                eprintln!("Could not parse package. {:?}", &as_string);
                return Ok(Response::new(BoxBody::new(Full::new(Bytes::from(
                    "Invalid JSON",
                )))));
            }
        };

        return Ok(Response::new(Full::new(Bytes::from("Not yet implemented"))));
    }
}
