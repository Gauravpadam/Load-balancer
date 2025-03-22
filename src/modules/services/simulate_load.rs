use crate::modules::response::{ApiResponse, Message};
use axum::{extract::Query};
use ollama_rs::Ollama;
use ollama_rs::generation::completion::request::GenerationRequest;
use serde::Deserialize;


struct Model {
    name: String,
    instance: Ollama,
    temperature: f32,

}

#[derive(Deserialize)]
pub struct LLMQuery {
    query: String
}

impl Model {
    fn new(name: String, temperature: f32) -> Self{
        let _name = name.to_owned();
        Model{
            name: _name,
            instance: Ollama::new("http://ollama", 11434),
            temperature
        }
    }
}


pub async fn simulate_load(query: Query<LLMQuery>) -> ApiResponse {
    let prompt: LLMQuery = query.0;
    let ollama_model = Model::new("llama3".to_string(), 0.1);
    let request = GenerationRequest::new(ollama_model.name, prompt.query);
    let response: String = ollama_model.instance.generate(request).await.unwrap().response;
    ApiResponse::JsonData(vec![Message { message: response }])

}
