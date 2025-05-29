use std::net::SocketAddr;
use std::path::Path;
use salvo::prelude::*;
use anyhow::Result;
use salvo::conn::tcp::TcpAcceptor;
use once_cell::sync::OnceCell;
use tokio::fs;


#[derive(Clone)]
pub struct HTTPConfig {
    pub basic_path: String,
    pub auth_token: String,
}

static HTTP_CONFIG: OnceCell<HTTPConfig> = OnceCell::new();

struct AuthMiddleware;

#[async_trait]
impl Handler for AuthMiddleware {
    async fn handle(&self, req: &mut Request, depot: &mut Depot, res: &mut Response, ctrl: &mut FlowCtrl) {
        match HTTP_CONFIG.get() {
            Some(config) => {
                if config.auth_token.is_empty() {
                    ctrl.call_next(req, depot, res).await;
                    return;
                }
                if let Some(header_value) = req.headers().get("X-AUTH") {
                    if let Ok(header_str) = header_value.to_str() {
                        if header_str == config.auth_token {
                            ctrl.call_next(req, depot, res).await;
                        } else {
                            res.status_code(StatusCode::FORBIDDEN);
                            res.render("Forbidden: Invalid token");
                        }
                    } else {
                        res.status_code(StatusCode::BAD_REQUEST);
                        res.render("Forbidden: Invalid X-AUTH header format");
                    }
                } else {
                    res.status_code(StatusCode::FORBIDDEN);
                    res.render("Forbidden: Missing X-AUTH header");
                }
            }
            None => {
                ctrl.call_next(req, depot, res).await;
            }
        }
    }
}

#[handler]
async fn download(req:&mut Request, res: &mut Response) {
    let path_query = req.query::<String>("path");
    match (path_query, HTTP_CONFIG.get()) {
        (Some(p), Some(config))=> {
            res.send_file(format!("{}/{}", config.basic_path, p), req.headers()).await;
        }
        (_, None) => {
            res.status_code(StatusCode::BAD_REQUEST);
            res.render("HTTP Service not configured");
        }
        (None, Some(_)) => { 
            res.status_code(StatusCode::NOT_FOUND);
            res.render("");
        }
    }
}

#[handler]
async fn upload(req: &mut Request, res: &mut Response) {
    let config = match HTTP_CONFIG.get() {
        Some(cfg) => cfg.clone(),
        None => {
            res.status_code(StatusCode::BAD_REQUEST);
            res.render("HTTP Service not configured");
            return;
        }
    };
    let base_path_str = &config.basic_path;

    // Construct upload directory path
    let upload_dir = format!("{}/upload", base_path_str);

    // Create upload directory if it doesn't exist
    if !Path::new(&upload_dir).exists() {
        if let Err(e) = fs::create_dir(&upload_dir).await {
            res.status_code(StatusCode::INTERNAL_SERVER_ERROR);
            res.render(format!("Failed to create upload directory: {}", e));
            return;
        }
    }

    // Get the file from the request
    let file = match req.file("file").await {
        Some(file) => file,
        None => {
            res.status_code(StatusCode::BAD_REQUEST);
            res.render("No file uploaded with the name 'file'");
            return;
        }
    };

    let filename = match file.name()  {
        Some(name) => name,
        None => {
            res.status_code(StatusCode::BAD_REQUEST);
            res.render("upload file has no name");
            return;
        }
    };

    let save_path = format!("{}/{}", upload_dir, filename);
    // 确保 save_path 在 upload_dir 目录下面
    if Path::new(&save_path).parent() != Some(Path::new(&upload_dir)) {
        res.status_code(StatusCode::INTERNAL_SERVER_ERROR);
        res.render("bad file name");
    }

    if let Err(e) = std::fs::copy(file.path(), Path::new(&save_path))  {
        res.status_code(StatusCode::INTERNAL_SERVER_ERROR);
        res.render(format!("Failed to save file: {}", e));
        return;
    }

    res.status_code(StatusCode::OK);
}
pub struct HttpServer {
}

impl HttpServer {
    pub fn set_http_config(basic_path: String, auth_token: String) -> bool {
        let new_config = HTTPConfig { basic_path, auth_token };
        HTTP_CONFIG.set(new_config).is_ok()
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
            .hoop(AuthMiddleware) // Added AuthMiddleware
            .push(Router::with_path("/dl").get(download))
            .push(Router::with_path("/upload").hoop(max_size(1024 * 1024 * 100)).post(upload));
        Server::new(acceptor).serve(router).await;
        Ok(())
    }
}