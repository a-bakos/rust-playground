use actix_files::Files;
use actix_web::{web, App, HttpResponse, HttpServer};
use serde_json::json;

use handlebars::Handlebars;

async fn index(hb: web::Data<Handlebars<'_>>) -> HttpResponse {
    let data = json!({
        "project_name": "Actix CatDex",
        "cats": [
            {
                "name": "British Short Hair",
                "image_path": "/static/image/cat-1.jpg"
            },
            {
                "name": "Persian",
                "image_path": "/static/image/cat-2.jpg"
            },
            {
                "name": "Ragdoll",
                "image_path": "/static/image/cat-3.jpg"
            },
        ]
    });

    let body = hb.render("index", &data).unwrap();

    HttpResponse::Ok().body(body)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let mut handlebars = Handlebars::new();
    handlebars
        .register_templates_directory(".html", "./static/")
        .unwrap();
    let handlebars_ref = web::Data::new(handlebars);

    println!("Listening on port 8080");

    HttpServer::new(move || {
        App::new()
            .app_data(handlebars_ref.clone())
            .service(Files::new("/static", "static").show_files_listing())
            .route("/", web::get().to(index))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
