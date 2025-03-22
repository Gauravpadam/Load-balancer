use crate::modules::response::ApiResponse;
use crate::modules::response::Message;

pub async fn hello_world() -> ApiResponse{
    ApiResponse::JsonData(vec![Message { message: "Hello world".to_string() }])
}
