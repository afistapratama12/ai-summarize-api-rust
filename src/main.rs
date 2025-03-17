use warp;

mod driver;
mod handler;
mod model;
mod routes;
mod libs;
mod common;
mod middleware;

use driver::db;

#[tokio::main]
async fn main() {
    // init database
    let db = db::connect_db().await;
    // init routes
    let routes = routes::routes(db);

    // start server in port 3000
    warp::serve(routes)
        .run(([127, 0, 0, 1], 3000))
        .await;
}