use actix_files::Files as ActixFiles;
use actix_web::{web, App, HttpServer};

use handlers::{add_task, change_done, delete_done, delete_task, edit_task, index, update_task};
use repository::Repository;

mod handlers;
mod repository;
mod templates;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    const PORT: u16 = 80;
    const HOST: &str = "0.0.0.0";

    dotenv::dotenv().ok();
    let database = Repository::try_init()
        .await
        .expect("Failed to initialize database, contact the author.");

    println!("Starting server at http://{HOST}:{PORT}");
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(database.clone()))
            .service(index)
            .service(add_task)
            .service(delete_task)
            .service(change_done)
            .service(delete_done)
            .service(edit_task)
            .service(update_task)
            .service(ActixFiles::new("/", "./src/static").prefer_utf8(true))
    })
    .bind((HOST, PORT))?
    .run()
    .await
}
