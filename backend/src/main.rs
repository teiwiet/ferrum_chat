use rocket_ws::WebSocket;

#[rocket::get("/")]
fn chat(ws:WebSocket){
    ws.channel()
}
#[rocket::main]
async fn  main() {
    let _ = rocket::build()
        .mount("/",rocket::routes![
            chat
        ])
        .launch()
        .await;
}
