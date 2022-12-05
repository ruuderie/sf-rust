use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::Deserialize;
use salesforce_standard_api::{Client, QueryResult};

#[derive(Deserialize)]
struct Account {
    name: String,
    website: String,
}

fn get_accounts(client: web::Data<Client>) -> impl Responder {
    let result: QueryResult<Account> = client
        .query("SELECT name, website FROM Account")
        .unwrap();

    let accounts = result.records;

    HttpResponse::Ok().json(accounts)
}

fn main() {
    let client = Client::new("https://login.salesforce.com").unwrap();

    HttpServer::new(move || {
        App::new()
            .data(client.clone())
            .route("/accounts", web::get().to(get_accounts))
    })
    .bind("127.0.0.1:8080")
    .unwrap()
    .run()
    .unwrap();
}
