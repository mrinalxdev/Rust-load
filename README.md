# Rust Load Balancer ⚖️

## Overview 🤔

This project implements a simple HTTP load balancer in Rust using the Actix web framework. The load balancer distributes incoming HTTP request across a pool of backend servers. It supports basic features such as health checks, dynamic addition, and removal of servers, as well as weighted round robin load balancing. 

## Features 💻

- Round-robin load balancing algorithm.
- Weighted round-robin load balancing algorithm (optional).
- Health checks for backend servers.
- Dynamic addition and removal of backend servers.
- Asynchronous handling of HTTP requests.
- Logging of server status changes and errors.

## How to use 🧑‍💻

1. Clone the repository: `git clone https://github.com/your-repo.git`
2. Navigate to the project directory: `cd rust-load-balancer`
3. Ensure you have Rust and Cargo installed.
4. Modify the server configuration in the `main.rs` file to specify the backend servers and their weights.
5. Run the load balancer: `cargo run`

## Load Balancer Use Case
The load balancer is designed to distribute incoming HTTP requests across multiple backend servers to improve performance, scalability, and reliability of web applications. It can be used in various scenarios, including:
- **High Traffic Websites**: Websites experiencing high traffic volumes can benefit from load balancing to distribute the load evenly across multiple servers, preventing overload and ensuring optimal performance for users.
- **Fault Tolerance**: Load balancers can help improve fault tolerance by routing traffic away from unhealthy or failed servers, ensuring continuous availability of services even in the event of server failures.
- **Scalability**: Load balancers facilitate horizontal scaling by allowing additional servers to be added to the pool dynamically as traffic increases, enabling applications to handle growing workloads efficiently.
- **Session Persistence**: Some load balancers support session persistence, ensuring that requests from the same client are consistently routed to the same backend server, which is useful for maintaining session state in stateful applications.

## Optimization
The load balancer is optimized for performance and scalability with the following considerations:
- **Asynchronous Handling**: The load balancer asynchronously handles HTTP requests and health checks, allowing it to efficiently handle a large number of concurrent connections without blocking.
- **Dynamic Server Management**: The ability to dynamically add and remove backend servers enables the load balancer to adapt to changing traffic conditions and maintain optimal performance.
- **Weighted Round-Robin Load Balancing**: In addition to basic round-robin load balancing, the load balancer supports weighted round-robin, allowing administrators to assign different weights to backend servers based on their capacity and performance.
- **Health Checks**: The load balancer periodically checks the health of backend servers to ensure that only healthy servers receive traffic, reducing the likelihood of routing requests to failed or unresponsive servers.
- **Logging**: Detailed logging of server status changes and errors provides visibility into the operation of the load balancer, facilitating troubleshooting and monitoring of the system.
