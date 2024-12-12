// Importing necessary modules from Actix Web
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

// The `hello` route handler for GET requests to the root URL ("/")
#[get("/")]
async fn hello() -> impl Responder {
    // Respond with a simple "Hello world!" message
    HttpResponse::Ok().body("Hello world!")
}

// The `repeat` route handler for POST requests to "/repeat"
#[post("/repeat")]
async fn repeat(req_body: String) -> impl Responder {
    // Return the body of the POST request back to the client
    HttpResponse::Ok().body(req_body)
}

// Custom handler for 404 Not Found errors
async fn not_found() -> impl Responder {
    // Respond with a 404 Not Found status and serve an HTML page from the `static` folder
    HttpResponse::NotFound()
        .content_type("text/html") // Set the content type to HTML
        .body(include_str!("../static/index.html")) // Include HTML content from a static file
}

// The main function that starts the Actix Web server
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Create and configure the Actix Web server
    let server = HttpServer::new(|| {
        // Create a new Actix Web application
        App::new()
            // Register the `hello` route to handle GET requests at the root URL
            .service(hello)
            // Register the `repeat` route to handle POST requests at "/repeat"
            .service(repeat)
            // Register a default service to handle all unrecognized routes (404 Not Found)
            .default_service(web::route().to(not_found)) // Calls `not_found` for unmatched routes
    })
    // Bind the server
    .bind("0.0.0.0:0")?;
    // Get the actual address the server is bound to
    let bound_address = server.addrs().first().unwrap().clone();
    println!("Server is running on http://{}", bound_address);
    // Start the server and await the result
    server.run().await
}
