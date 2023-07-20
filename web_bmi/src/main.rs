extern crate actix_web;
extern crate lazy_static;
extern crate tera;

use actix_web::{App, get, HttpRequest, HttpResponse, HttpServer, post, Responder, web};
use lazy_static::lazy_static;
use serde::Deserialize;
use tera::{Context, Tera};

#[derive(Deserialize, Debug)]
pub struct FormBMI {
    height: f64,
    weight: f64,
}

// #[get("/")]
// async fn index(_: HttpRequest) -> Result<HttpResponse, Error> {
//     return Ok(HttpResponse::Ok()
//         .content_type("text/html; charset=utf-8")
//         .body(r#"
//         <html><body<h1>BMI 계산 및 비만도 판정"</h1>
//         <form action="calc" method="post">
//         <div>키 : <div><label><input type="text" name="height" placeholder="160" /></label></div></div>
//         <div>몸무게 : <div><label><input type="text" name="weight" placeholder="70" /></label></div> </div>
//         <div><label><input type="submit" /></label></div>
//         </form></body></html>
//         "#));
// }

// #[post("/calc")]
// async fn calc(q: web::Form<FormBMI>) -> Result<HttpResponse, Error> {
//     let h = q.height / 100.0;
//     let bmi = q.weight / (h * h);
//     let per = (bmi / 22.0) * 100.0;
//
//     return Ok(HttpResponse::Ok()
//         .content_type("text/html; charset=utf-8")
//         .body(format!("<h3> BMI = {:.1}, 비만율 = {:.0}%</h3>", bmi, per)));
// }

#[cfg(debug_assertions)]
const ROOT_PATH: &str = env!("CARGO_MANIFEST_DIR");

#[cfg(not(debug_assertions))]
const ROOT_PATH: &str = "/Users/joonho/Desktop/tera_root";

lazy_static! {
    pub static ref TEMPLATES: Tera = {
        // let mut tera: Tera = match Tera::new(concat!(ROOT_PATH, "/templates/**/*.html")) {
        //     Ok(v) => v,
        //     Err(e) => {
        //         println!("Template parsing error : {:?}", e);
        //         std::process::exit(1);
        //     },
        // };
        let mut tera: Tera = match Tera::new(format!("{}{}", ROOT_PATH, "/templates/**/*.html").as_str()) {
            Ok(v) => v,
            Err(e) => {
                println!("Template parsing error : {:?}", e);
                std::process::exit(1);
            },
        };

        tera.autoescape_on(vec!["html"]);
        tera.full_reload().unwrap();

        return tera;
    };
}

#[get("/")]
async fn index(_: HttpRequest) -> impl Responder {
    let context: Context = Context::new();
    let render = TEMPLATES.render("index.html", &context).unwrap();

    return HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(render);
}

#[post("/calc")]
async fn calc(q: web::Form<FormBMI>) -> impl Responder {
    let h = q.height / 100.0;
    let bmi = q.weight / (h * h);
    let per = (bmi / 22.0) * 100.0;
    let mut context: Context = Context::new();

    context.insert("bmi", format!("{:.1}", bmi).as_str());
    context.insert("per", format!("{:.0}", per).as_str());

    let render = TEMPLATES.render("result.html", &context).unwrap();

    return HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(render);
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // println!(
    //     "tera path : {}",
    //     concat!(env!("CARGO_MANIFEST_DIR"), "/templates/**/*.html")
    // );
    //
    // let tera = web::Data::new(Mutex::new(
    //     Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/**/*.html")).unwrap(),
    // ));

    HttpServer::new(move || {
        App::new()
            // .app_data(tera.clone())
            .service(index)
            .service(calc)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
