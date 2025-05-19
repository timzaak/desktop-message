use std::net::SocketAddr;
use salvo::prelude::*;
use anyhow::Result;
use salvo::conn::tcp::TcpAcceptor;
#[handler]
async fn hello_world(req:&mut Request, res: &mut Response) {
    let path = req.query::<String>("path");
    match path {
        Some(p)=> {
            res.send_file(p, req.headers()).await;
        }
        None => {
            res.status_code(StatusCode::NOT_FOUND);
            res.render("");
        }
    }
}
pub struct HttpServer {
}

impl HttpServer {
    pub fn try_bind(address: SocketAddr) -> Result<(TcpAcceptor, SocketAddr)> {
        let acceptor = tokio::runtime::Handle::current().block_on(async {
            let acceptor = TcpListener::new(address).try_bind().await;
            acceptor
        })?;
        let address = acceptor.local_addr()?;
        Ok((acceptor, address))
    }
    pub async fn start_http_server(acceptor: TcpAcceptor) -> Result<()> {
        let router = Router::new()
            .push(Router::with_path("/dl").get(hello_world));
        Server::new(acceptor).serve(router).await;
        Ok(())
    }
    
}