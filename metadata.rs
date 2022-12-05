use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::Deserialize;
use salesforce_metadata_api::{Client, CustomObject, Field, CustomField};

#[derive(Deserialize)]
struct NewCustomObject {
    name: String,
    label: String,
    plural_label: String,
    fields: Vec<NewCustomField>,
}

#[derive(Deserialize)]
struct NewCustomField {
    name: String,
    label: String,
    type_: String,
}

fn create_custom_object(
    new_object: web::Json<NewCustomObject>,
    client: web::Data<Client>,
) -> impl Responder {
    let fields = new_object
        .fields
        .into_iter()
        .map(|field| CustomField {
            name: field.name,
            label: field.label,
            type_: field.type_,
            ..CustomField::default()
        })
        .collect();

    let custom_object = CustomObject {
        name: new_object.name,
        label: new_object.label,
        plural_label: new_object.plural_label,
        fields,
    };

    client
        .create_metadata(vec![custom_object])
        .unwrap();

    HttpResponse::Ok().json("Custom object created successfully")
}

fn main() {
    let client = Client::new("https://login.salesforce.com").unwrap();

    HttpServer::new(move || {
        App::new()
            .data(client.clone())
            .route("/create", web::post().to(create_custom_object))
    })
    .bind("127.0.0.1:8080")
    .unwrap()
    .run()
    .unwrap();
}
