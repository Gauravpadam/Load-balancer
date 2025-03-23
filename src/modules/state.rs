use once_cell::sync::Lazy;
use tokio::sync::Mutex;

#[derive(Clone)]
pub struct AppState{
    pub token: i32
}

pub static APP_STATE: Lazy<Mutex<AppState>> = Lazy::new(|| Mutex::new(AppState { token: 0 }));