use actix_web::{get, post, Responder, HttpResponse, HttpServer, App, web, guard};
use std::sync::Mutex;
use config::{config, scoped_config};
// Cara pembuatan route dengan macros
// #[get("/")]
// async fn route_hello() -> impl Responder {
//     HttpResponse::Created().body("Hello world! TEFA")
// }

#[post("/echo")]
async fn handler_echo(req_body: String) -> impl Responder {
    let result = format!("{} BINUS", req_body);
    HttpResponse::Ok().body(result)
}
// Cara pembuatan route tidak dengan macros
async fn route_manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

// struct AppState {
//     app_name: String,
// }

// #[get("/data")]
// async fn index(data: web::Data<AppState>) -> String {
//     let app_name = &data.app_name; // <- get app_name
//     format!("Hello {app_name}!") // <- response with app_name
// }

// #[get("/aplikasi")]
// async fn idx(data: web::Data<AppState>) -> String {
//     let app_name = &data.app_name; // <- get app_name
//     format!("Ini aplikasi apa {app_name}!") // <- response with app_name
// }

struct AppStateWithCounter {
    counter: Mutex<i32>, // <- Mutex is necessary to mutate safely across threads
}

async fn counter(data: web::Data<AppStateWithCounter>) -> String {
    let mut counter = data.counter.lock().unwrap(); // <- get counter's MutexGuard
    *counter += 1; // <- access counter inside MutexGuard

    format!("Request number: {counter}") // <- response with count
}
// cara untuk daftarin module diluar main
mod config;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Note: web::Data created _outside_ HttpServer::new closure
    let counter = web::Data::new(AppStateWithCounter {
        counter: Mutex::new(0),
    });

    HttpServer::new(move || {
        App::new()
            .configure(config)
            .service(web::scope("/api").configure(scoped_config))
            .route(
                "/",
                web::get().to(|| async { HttpResponse::Ok().body("/") }),
            )
            // .service(
            //     web::scope("/")
            //         .guard(guard::Header("Host", "www.rust-lang.org"))
            //         .route("", web::to(|| async { HttpResponse::Ok().body("www") })),
            // )
            // .service(
            //     web::scope("/")
            //         .guard(guard::Header("Port", "users.rust-lang.org"))
            //         .route("", web::to(|| async { HttpResponse::Ok().body("user") })),
            // )
            // .route("/", web::to(HttpResponse::Ok))
            // .app_data(web::Data::new(AppState {
            //     app_name: String::from("Binus web aplikasi"),
            // }))
            // .app_data(counter.clone()) // <- register the created data
            // .route("/counter", web::get().to(counter))
            // .service(index)
            // .service(idx)
            //define di app kalau menggunakan macros
            // .service(route_hello)
            // .service(handler_echo)
            // .service(
            //     web::scope("/app")
            //         .service(route_hello)
            //         .service(handler_echo)
            // )
            // define di app jika tidak menggunakan macros (lebih direkomendasikan)
            // .route("/hey", web::get().to(route_manual_hello))
            // .service(
            //     web::scope("/halo")
            //         .route("/hey", web::get().to(route_manual_hello))
            // )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
