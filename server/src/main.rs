use actix_web::{get, App, HttpResponse, HttpServer};

#[get("/")]
async fn index() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(
            r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta http-equiv="X-UA-Compatible" content="IE=edge">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Hello world</title>
</head>
<body>
    This is just a hello world content!
</body>
</html>
"#,
        )
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Running Actix server");
    HttpServer::new(|| App::new().service(index))
        .bind("0.0.0.0:80")?
        .run()
        .await
}
