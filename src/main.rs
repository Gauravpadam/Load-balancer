use tokio::net::TcpListener;
mod modules;
use modules::routing_service::init_router;

#[tokio::main]
async fn main(){
    let app = init_router();
    let listener = TcpListener::bind("0.0.0.0:5173").await.unwrap();
    axum::serve(listener, app).await.unwrap()
}

