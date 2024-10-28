# prt-ollama-chat-rust
API de Chat Completions
Este servidor permite interactuar con modelos de IA a través de Ollama, utilizando el endpoint /api/chat para enviar mensajes en un contexto de chat. La API soporta tanto respuestas en streaming como respuestas completas (no-streaming).

Estructuras de Datos
1. ChatRequest
   Define la estructura de la solicitud enviada al endpoint /api/chat.

rust
Copy code
#[derive(Deserialize)]
struct ChatRequest {
model: String,                  // Nombre del modelo requerido
messages: Vec<Message>,          // Mensajes en la conversación
tools: Option<Vec<Tool>>,        // Herramientas opcionales para el modelo
format: Option<String>,          // Formato de la respuesta (actualmente solo JSON)
options: Option<serde_json::Value>, // Parámetros adicionales para el modelo
stream: Option<bool>,            // Indica si la respuesta es en streaming
keep_alive: Option<String>,      // Mantiene el modelo en memoria por un tiempo determinado
}
2. Message
   Define la estructura de un mensaje dentro de la conversación.

rust
Copy code
#[derive(Serialize, Deserialize)]
struct Message {
role: String,                     // Rol del mensaje (system, user, assistant, tool)
content: String,                  // Contenido del mensaje
images: Option<Vec<String>>,      // Lista opcional de imágenes en Base64
tool_calls: Option<Vec<ToolCall>>, // Lista opcional de llamadas a herramientas
}
3. Tool
   Define una herramienta que el modelo puede utilizar durante la conversación.

rust
Copy code
#[derive(Serialize, Deserialize)]
struct Tool {
tool_type: String,               // Tipo de herramienta (p. ej., función)
function: Function,              // Función de la herramienta
}
4. Function
   Representa una función específica de una herramienta.

rust
Copy code
#[derive(Serialize, Deserialize)]
struct Function {
name: String,                     // Nombre de la función
description: String,              // Descripción de la función
parameters: serde_json::Value,    // Parámetros de la función
}
5. ToolCall
   Representa una llamada a una herramienta específica.

rust
Copy code
#[derive(Serialize, Deserialize)]
struct ToolCall {
function: Function,               // Función llamada por la herramienta
}
Endpoints
POST /api/chat
Genera la próxima respuesta en una conversación de chat utilizando el modelo proporcionado. Permite realizar una solicitud en streaming o no-streaming.

Parámetros de la Solicitud
model (string, requerido): Nombre del modelo a utilizar.
messages (array de objetos Message, requerido): Lista de mensajes que representa la conversación.
tools (array de objetos Tool, opcional): Herramientas adicionales que el modelo puede usar.
format (string, opcional): Formato de la respuesta. Actualmente, solo se soporta JSON.
options (objeto, opcional): Parámetros avanzados para la configuración del modelo.
stream (bool, opcional): Si se establece en true, la respuesta se devuelve en streaming.
keep_alive (string, opcional): Tiempo que el modelo se mantendrá en memoria después de la solicitud.
Ejemplo de Solicitud con curl
Solicitud en Streaming

bash
Copy code
curl -X POST http://localhost:8080/api/chat \
-H "Content-Type: application/json" \
-d '{
"model": "llama3.2",
"messages": [
{
"role": "user",
"content": "Why is the sky blue?"
}
],
"stream": true
}'
Solicitud No-Streaming (Respuesta Completa)

bash
Copy code
curl -X POST http://localhost:8080/api/chat \
-H "Content-Type: application/json" \
-d '{
"model": "llama3.2",
"messages": [
{
"role": "user",
"content": "Why is the sky blue?"
}
],
"stream": false
}'
Respuestas
Respuesta en Streaming

Una serie de objetos JSON devueltos en tiempo real, cada uno conteniendo una porción de la respuesta.
Respuesta Completa (No-Streaming)

Devuelve un único objeto JSON que contiene el mensaje completo y datos adicionales.
json
Copy code
{
"model": "llama3.2",
"created_at": "2023-08-04T08:52:19.385406455-07:00",
"message": {
"role": "assistant",
"content": "The sky is blue because of Rayleigh scattering."
},
"done": true,
"total_duration": 5191566416,
"load_duration": 2154458,
"prompt_eval_count": 26,
"prompt_eval_duration": 383809000,
"eval_count": 298,
"eval_duration": 4799921000
}
Servidor Actix Web
Código del Servidor
rust
Copy code
#[actix_web::main]
async fn main() -> std::io::Result<()> {
HttpServer::new(|| {
App::new()
.service(chat_response) // Registra el endpoint para chat completions
})
.bind("127.0.0.1:8080")? // Cambia el puerto si es necesario
.run()
.await
}
Este servidor inicia el servicio en el puerto 8080, escucha solicitudes en /api/chat, y envía las solicitudes a Ollama para procesarlas.

Notas
Streaming: Cuando stream es true, la respuesta se envía en fragmentos al cliente, útil para manejar conversaciones en tiempo real.
Herramientas: Las herramientas deben ser especificadas si se necesita funcionalidad adicional en la conversación.
Parámetros Avanzados: Los parámetros avanzados permiten personalizar la interacción con el modelo, como temperature, keep_alive, y otros, según la configuración de Ollama.
