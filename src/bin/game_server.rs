#[macro_use]
extern crate rocket;

use rocket::fs::FileServer;
use rocket::http::ContentType;

#[get("/")]
fn pong() -> (ContentType, &'static str) {
    //
    (
        ContentType::HTML,
        r#"
    <html lang="en">
    <head>
    <meta charset="utf-8">
    <title>demo</title>
    <style>
        html,
        body,
        canvas {
            margin: 0px;
            padding: 0px;
            width: 100%;
            height: 100%;
            overflow: hidden;
            position: absolute;
            background: black;
            z-index: 0;
        }
    </style>
    </head>
    <body>
    <canvas id="glcanvas" tabindex='1'></canvas>
    <!-- Minified and statically hosted version of https://github.com/not-fl3/macroquad/blob/master/js/mq_js_bundle.js -->
    <script src="https://not-fl3.github.io/miniquad-samples/mq_js_bundle.js"></script>
    <script>load("/static/pong.wasm");</script>
    </body>
    </html>
    "#,
    )
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![pong])
        .mount("/static", FileServer::from("./static"))
}
