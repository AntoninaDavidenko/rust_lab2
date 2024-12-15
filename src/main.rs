use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::Deserialize;
use tera::{Tera, Context};

#[derive(Deserialize)]
struct CalcParams {
    a: f64,
    b: f64,
    op: String, // Оператор: "+", "-", "*", або "/"
}

// для обчислень
async fn calculate(params: web::Query<CalcParams>) -> impl Responder {
    let result = match params.op.as_str() {
        "-" => params.a - params.b,
        "+" => params.a + params.b,
        "*" => params.a * params.b,
        "/" => {
            if params.b == 0.0 {
                return HttpResponse::BadRequest().body("Division by zero");
            }
            params.a / params.b
        }
        _ => return HttpResponse::BadRequest().body("Invalid operation"),
    };
    HttpResponse::Ok().body(result.to_string())
}

// Основний маршрут для HTML-інтерфейсу
async fn calculator_page(tmpl: web::Data<Tera>) -> impl Responder {
    let ctx = Context::new();
    let rendered = tmpl.render("calculator.html", &ctx).unwrap();
    HttpResponse::Ok().content_type("text/html").body(rendered)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let tera = Tera::new("templates/**/*").unwrap();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(tera.clone()))
            .route("/", web::get().to(calculator_page))
            .route("/calculate", web::get().to(calculate))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

