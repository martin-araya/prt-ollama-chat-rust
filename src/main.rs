use actix_web::{post, web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use reqwest::Client;
use serde_json::json;
use futures::StreamExt;

#[derive(Deserialize)]
struct ChatRequest {
    model: String,
    messages: Vec<Message>,
    tools: Option<Vec<Tool>>,
    format: Option<String>,
    options: Option<serde_json::Value>,
    stream: Option<bool>,
    keep_alive: Option<String>,
}

#[derive(Serialize, Deserialize)]
struct Message {
    role: String,
    content: String,
    images: Option<Vec<String>>,
    tool_calls: Option<Vec<ToolCall>>,
}

#[derive(Serialize, Deserialize)]
struct Tool {
    tool_type: String,
    function: Function,
}

#[derive(Serialize, Deserialize)]
struct Function {
    name: String,
    description: String,
    parameters: serde_json::Value,
}

#[derive(Serialize, Deserialize)]
struct ToolCall {
    function: Function,
}

#[post("/api/chat")]
async fn chat_response(req: web::Json<ChatRequest>) -> impl Responder {
    let client = Client::new();
    let url = "http://localhost:11434/api/chat";

    let body = json!({
        "model": req.model,
        "messages": req.messages,
        "tools": req.tools,
        "format": req.format,
        "options": req.options,
        "stream": req.stream.unwrap_or(true),
        "keep_alive": req.keep_alive.clone().unwrap_or_else(|| "5m".to_string()),
    });

    match client.post(url).json(&body).send().await {
        Ok(response) => {
            if req.stream.unwrap_or(true) {
                let mut stream = response.bytes_stream();

                let response_stream = async_stream::stream! {
                    while let Some(chunk) = stream.next().await {
                        match chunk {
                            Ok(bytes) => {
                                yield Ok::<_, actix_web::Error>(web::Bytes::from(bytes));
                            }
                            Err(_) => {
                                yield Err(actix_web::error::ErrorInternalServerError("Error reading stream"));
                            }
                        }
                    }
                };

                return HttpResponse::Ok()
                    .content_type("application/json")
                    .streaming(response_stream);
            }

            // Procesar la respuesta completa como JSON si `stream` es `false`
            match response.json::<serde_json::Value>().await {
                Ok(json) => {
                    if let Some(message) = json.get("message") {
                        HttpResponse::Ok().json(message.clone())
                    } else {
                        HttpResponse::InternalServerError().body("Field 'message' not found in Ollama response")
                    }
                },
                Err(_) => HttpResponse::InternalServerError().body("Error parsing Ollama response"),
            }
        }
        Err(_) => HttpResponse::InternalServerError().body("Failed to connect to Ollama"),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(chat_response) // Registra el endpoint para chat completions
    })
        .bind("127.0.0.1:8081")? // Cambia el puerto si es necesario
        .run()
        .await
}
