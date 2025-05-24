use std::net::SocketAddr;
use salvo::prelude::*;
use anyhow::Result;
use salvo::conn::tcp::TcpAcceptor;
use once_cell::sync::OnceCell;
static BASIC_PATH: OnceCell<String> = OnceCell::new();
#[handler]
async fn download(req:&mut Request, res: &mut Response) {
    let path = req.query::<String>("path");
    match (path, BASIC_PATH.get()) {
        (Some(p), Some(prefix))=> {
            res.send_file(format!("{}/{}", prefix, p), req.headers()).await;
        }
        (_, None) => {
            res.status_code(StatusCode::BAD_REQUEST);
            res.render("Basic Path does not config");
        }
        (None,_) => {
            res.status_code(StatusCode::NOT_FOUND);
            res.render("");
        }
    }
}
pub struct HttpServer {
}

impl HttpServer {
    pub fn set_basic_path(path: String) -> bool{
        BASIC_PATH.set(path).is_ok()
    }
    pub fn try_bind(address: SocketAddr) -> Result<(TcpAcceptor, SocketAddr)> {
        let std_listener = std::net::TcpListener::bind(address)?;
        let address = std_listener.local_addr()?;
        let tokio_listener = tokio::net::TcpListener::from_std(std_listener)?;
        let acceptor = TcpAcceptor::try_from(tokio_listener)?;
        Ok((acceptor, address))
    }
    pub async fn start_http_server(acceptor: TcpAcceptor) -> Result<()> {
        let router = Router::new()
            .push(Router::with_path("/dl").get(download));
        Server::new(acceptor).serve(router).await;
        Ok(())
    }
}