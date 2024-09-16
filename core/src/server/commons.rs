use std::{
    future::Future,
    net::{IpAddr, SocketAddr},
    pin::Pin,
};

use http_body_util::Full;
use hyper::{
    body::{Bytes, Incoming},
    server::conn::http1,
    service::Service,
    Request, Response,
};
use hyper_util::rt::{TokioIo, TokioTimer};

use tokio::net::TcpListener;

pub(crate) async fn spawn_server<S>(
    ip: IpAddr,
    port: u16,
    service: &S,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>>
where
    S: 'static
        + Clone
        + Send
        + Service<
            Request<Incoming>,
            Response = Response<Full<Bytes>>,
            Error = Box<dyn std::error::Error + Send + Sync>,
            Future = Pin<
                Box<
                    dyn Future<
                            Output = Result<
                                Response<Full<Bytes>>,
                                Box<dyn std::error::Error + Send + Sync>,
                            >,
                        > + Send,
                >,
            >,
        >,
{
    let socket = SocketAddr::new(ip, port);
    let listener = TcpListener::bind(socket).await?;

    loop {
        let (tcp, _) = listener.accept().await?;
        //let (call_tcp_stream, _) = call_listener.accept().await?;

        //Need to spawn these as separate tasks...
        let io = TokioIo::new(tcp);
        let clone = service.clone();

        tokio::task::spawn(async move {
            // Handle the connection from the client using HTTP1 and pass any
            // HTTP requests received on that connection to the `hello` function
            if let Err(err) = http1::Builder::new()
                .timer(TokioTimer::new())
                .serve_connection(io, clone)
                .await
            {
                println!("Error serving connection: {:?}", err);
            }
        });
    }
}
