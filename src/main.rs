use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer};
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tokio::time;
use log::{info, error};
use serde::{Deserialize, Serialize};


// Struct to represent a backend server
#[derive(Clone, Serialize, Deserialize)]
struct Server {
    address: String,
    is_alive: Arc<Mutex<bool>>,
    weight: usize, // Weight for weighted round-robin
}

impl Server {
    // Method to serve requests
    async fn serve(&self, _req: HttpRequest) -> HttpResponse {
        HttpResponse::Ok().body(format!("Hello from server: {}", self.address))
    }

    // Method to check server health
    async fn check_health(&self) -> bool {
        // Simulate health check
        // You can replace this with actual health check logic
        let is_alive = *self.is_alive.lock().unwrap();
        // Simulate asynchronous operation
        time::sleep(Duration::from_secs(1)).await;
        is_alive
    }
}

// Struct to represent the load balancer
#[derive(Default)]
struct LoadBalancer {
    servers: Arc<Mutex<Vec<Server>>>,
    round_robin_count: usize,
}

impl LoadBalancer {
    // Method to create a new load balancer
    fn new(servers: Vec<Server>) -> Self {
        LoadBalancer {
            servers: Arc::new(Mutex::new(servers)),
            round_robin_count: 0,
        }
    }

    // Method to select the next available server using round-robin algorithm
    async fn select_server(&mut self) -> Option<Server> {
        let mut servers = self.servers.lock().unwrap();
        let len = servers.len();
        if len == 0 {
            return None;
        }
        for _ in 0..len {
            self.round_robin_count = (self.round_robin_count + 1) % len;
            let idx = self.round_robin_count;
            if servers[idx].check_health().await {
                return Some(servers[idx].clone());
            }
        }
        None
    }

    // Method to add a new server to the load balancer
    fn add_server(&mut self, server: Server) {
        let mut servers = self.servers.lock().unwrap();
        servers.push(server);
    }

    // Method to remove a server from the load balancer
    fn remove_server(&mut self, address: &str) {
        let mut servers = self.servers.lock().unwrap();
        servers.retain(|server| server.address != address);
    }
}

// Handler function for HTTP requests
async fn handle_request(lb: web::Data<Arc<Mutex<LoadBalancer>>>, req: HttpRequest) -> HttpResponse {
    let mut lb = lb.lock().unwrap();
    if let Some(server) = lb.select_server().await {
        return server.serve(req).await;
    }
    HttpResponse::InternalServerError().finish()
}

// Function to perform health checks on servers
async fn health_checker(servers: Arc<Mutex<Vec<Server>>>) {
    loop {
        // Iterate through servers and check their health
        let mut servers = servers.lock().unwrap();
        for server in servers.iter_mut() {
            let is_alive = server.check_health().await;
            *server.is_alive.lock().unwrap() = is_alive;
            if !is_alive {
                error!("Server {} is down", server.address);
            }
        }
        drop(servers); // Release lock before sleeping
        time::sleep(Duration::from_secs(10)).await; // Sleep for 10 seconds before next health check
    }
}

// Function to set up logging
fn setup_logging() {
    // Initialize logger
    env_logger::Builder::from_default_env()
        .format_timestamp(None)
        .init();
}

// Function to parse server configuration
fn parse_config(config: &str) -> Vec<Server> {
    // Parse server configuration from a JSON string
    serde_json::from_str(config).unwrap_or_else(|err| {
        error!("Failed to parse server configuration: {}", err);
        Vec::new()
    })
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Setup logging
    setup_logging();
    info!("Starting load balancer...");

    // Read server configuration from file or environment variable
    let config = r#"[
        {"address": "http://localhost:8001", "weight": 1},
        {"address": "http://localhost:8002", "weight": 2}
    ]"#;
    let servers = parse_config(config);

    // Create a load balancer
    let lb = Arc::new(Mutex::new(LoadBalancer::new(servers.clone())));

    // Spawn health checker task
    let servers_arc = Arc::new(Mutex::new(servers));
    let health_checker_task = health_checker(servers_arc.clone());

    // Start Actix web server
    HttpServer::new(move || {
        App::new()
            .data(lb.clone())
            .route("/", web::get().to(handle_request))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await?;

    // Wait for health checker task to finish
    health_checker_task.await;

    Ok(())
}
