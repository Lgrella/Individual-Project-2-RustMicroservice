use actix_web::{web, App, HttpResponse, HttpServer, Responder, post};
use serde::Deserialize;

#[derive(Deserialize)]
struct Age {
    age: u32,
}

async fn index() -> impl Responder {
    let html = r#"
        <!DOCTYPE html>
        <html lang="en">
        <head>
            <meta charset="UTF-8">
            <title>Check Age</title>
        </head>
        <body>
            <h1>Check Age Range!</h1>
            <form action="/age" method="post">
                <label for="age">Your Age:</label>
                <input type="text" id="age" name="age">
                <input type="submit" value="Submit">
            </form>
        </body>
        </html>
    "#;

    HttpResponse::Ok().content_type("text/html").body(html)
} 

// write a function that takes a birthday and returns the age where the input comes from a web form
#[post("/age")]
async fn age_range1(form: web::Form<Age>) -> impl Responder {
    let age = form.age;
    let mut age_range = "";
    if age < 18 {
        age_range = "You are a minor.";
    } else if age >= 18 && age <= 64 {
        age_range = "You are an adult.";
    } else {
        age_range = "You are a senior citizen.";
    }
    HttpResponse::Ok().body(age_range)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .service(age_range1)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}

