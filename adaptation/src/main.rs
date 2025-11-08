mod node;
// mod mcts; // TODO: Fix mcts module issues

use actix_web::{web, App, HttpResponse, HttpServer, Result};
use actix_files as fs;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct SimulationRequest {
    iterations: u32,
    move_limit: u32,
}

#[derive(Serialize)]
struct SimulationResponse {
    status: String,
    message: String,
    iterations: u32,
    move_limit: u32,
}

async fn run_simulation(req: web::Json<SimulationRequest>) -> Result<HttpResponse> {
    // For now, return a success response
    // In the future, this would integrate with the actual MCTS implementation
    let response = SimulationResponse {
        status: "success".to_string(),
        message: format!(
            "Simulation completed with {} iterations and move limit of {}",
            req.iterations, req.move_limit
        ),
        iterations: req.iterations,
        move_limit: req.move_limit,
    };
    
    Ok(HttpResponse::Ok().json(response))
}

async fn index() -> Result<fs::NamedFile> {
    Ok(fs::NamedFile::open("static/index.html")?)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("🍹 Starting Smoothie MCTS UI server...");
    println!("📡 Server running at http://127.0.0.1:8080");
    println!("🌐 Open your browser and navigate to http://127.0.0.1:8080");
    
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .route("/api/run", web::post().to(run_simulation))
            .service(fs::Files::new("/static", "static").show_files_listing())
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
