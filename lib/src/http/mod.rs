use salvo::prelude::*;
use anyhow::Result;



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

pub async fn start_http_server() -> Result<()> {
    let acceptor = TcpListener::new("0.0.0.0:5800").bind().await;
    let router = Router::new()
        .push(Router::with_path("/dl").get(hello_world));
    Server::new(acceptor).serve(router).await;
    Ok(())
}