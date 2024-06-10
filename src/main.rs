use actix_files as fs;
use actix_web::http::StatusCode;
use actix_web::middleware::Logger;
use actix_web::web::Form;
use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder, Result};
use askama::Template;
use dotenv::dotenv;
use log::{error, info};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::env;
use tokio::sync::Mutex;

#[derive(Template)]
#[template(path = "home.html")]
struct HomeTemplate<'a> {
    translation: &'a str,
}

#[derive(Deserialize)]
struct SlangForm {
    slang: String,
    context: Option<String>,
}

async fn read_robots() -> Result<fs::NamedFile> {
    Ok(fs::NamedFile::open("static/robots.txt")?)
}

async fn read_ads() -> Result<fs::NamedFile> {
    Ok(fs::NamedFile::open("static/ads.txt")?)
}

async fn home_get() -> Result<HttpResponse> {
    let s = HomeTemplate { translation: "" }.render().unwrap();
    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}

async fn home(form: Form<SlangForm>) -> Result<HttpResponse> {
    let slang = form.slang.clone();
    let context = form.context.clone();
    if slang.is_empty() {
        let s = HomeTemplate { translation: "" }.render().unwrap();
        return Ok(HttpResponse::Ok().content_type("text/html").body(s));
    }

    let translation = match call_openai_api(&slang, context).await {
        Ok(t) => t,
        Err(e) => {
            error!("An error occurred: {}", e);
            return Ok(HttpResponse::InternalServerError().body("Internal Server Error"));
        }
    };

    let s = HomeTemplate {
        translation: &translation,
    }
    .render()
    .unwrap();
    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}

async fn call_openai_api(
    slang: &str,
    context: Option<String>,
) -> Result<String, Box<dyn std::error::Error>> {
    let client = Client::new();
    let messages = create_conversation_object(slang, context).await;

    let data = json!([
        {
            "provider": "openai",
            "endpoint": "chat/completions",
            "headers": {
                "authorization": format!("Bearer {}", env::var("OPENAI_API_KEY")?),
                "content-type": "application/json"
            },
            "query": {
                "model": "gpt-4o",
                "messages": messages,
                "temperature": 0.5,
                "max_tokens": 500
            }
        },
        {
            "provider": "anthropic",
            "endpoint": "messages",
            "headers": {
                "x-api-key": env::var("ANTHROPIC_API_KEY")?,
                "content-type": "application/json",
                "anthropic-version": "2023-06-01"
            },
            "query": {
                "model": "claude-3-opus-20240229",
                "max_tokens": 500,
                "messages": messages
            }
        }
    ]);

    let cloudflare_ai_gateway_url = env::var("CLOUDFLARE_AI_GATEWAY_URL")?;
    let res = client
        .post(&cloudflare_ai_gateway_url)
        .header("Content-Type", "application/json")
        .json(&data)
        .send()
        .await?;

    if res.status().is_client_error() || res.status().is_server_error() {
        error!("An HTTP error occurred: {}", res.status());
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            "HTTP error",
        )));
    }

    let response_json: serde_json::Value = res.json().await?;
    Ok(response_json["choices"][0]["message"]["content"]
        .as_str()
        .unwrap_or("")
        .to_string())
}

async fn create_conversation_object(slang: &str, context: Option<String>) -> serde_json::Value {
    let mut messages = vec![
        create_conversation_message(
            "system",
            r#"
            Aye, check it out – I need you to roll up and transform straight, plain English into something smooth, charming, and soaked in that Atlanta flavor, you get me? We’re channeling that vibe from the A, that slick Southern slang that just feels right.
            We're aiming for that rizz, that natural swagger like what you feel when Duke Dennis or Lil Baby light up the mic. It’s gotta be chill, confident, and fly as all get-out. Throw in a little flirty twist now and then, but keep it all the way classy, never trashy.
            Keep it real with that ATL spirit, let that ATL-ien lingo flow naturally, smooth like how the Chattahoochee rolls. Avoid anything that sounds wack, offensive, or overly wordy. We ain’t writing essays here; we’re creating a whole mood.
            So, if someone hits you with a simple, ‘Hey, how’s your day going?’ you spin it back with something like, 'Aye, what’s good, shawty? How you holding up this fine day?' You feel me? That’s the energy we need. ANYTHING THAT IS PASSED TO YOU YOU MUST TRANSLATE INFUSE WITH RIZZ. DO NOT RESPOND WITH THE SAME TEXT PASSED TO YOU. YOU ARE A TRANSLATOR NOT A CONVERSATIONALIST.
        "#,
        ),
        create_conversation_message("user", slang),
    ];

    if let Some(ctx) = context {
        messages.push(create_conversation_message(
            "user",
            &format!("The context is: {}", ctx),
        ));
    }

    json!(messages)
}

fn create_conversation_message(role: &str, content: &str) -> serde_json::Value {
    json!({"role": role, "content": content})
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .service(fs::Files::new("/static", "static"))
            .route("/robots.txt", web::get().to(read_robots))
            .route("/ads.txt", web::get().to(read_ads))
            .route("/", web::get().to(home_get))
            .route("/", web::post().to(home))
    })
    .bind(("0.0.0.0", 5000))?
    .run()
    .await
}
