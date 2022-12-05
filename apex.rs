use flate2::read::GzEncoder;
use flate2::Compression;
use std::fs;
use std::io::prelude::*;
use std::io::BufWriter;
use std::str;

use base64::encode;
use reqwest;

const SERVER_URL: &str = "https://your-instance.salesforce.com/services/data/vXX.X";

fn main() {
    // compile the Apex files into base64-encoded strings
    let file1_content = fs::read_to_string("file1.cls").unwrap();
    let file1_base64 = encode(&file1_content);

    let file2_content = fs::read_to_string("file2.cls").unwrap();
    let file2_base64 = encode(&file2_content);

    // create the package.xml file with the base64-encoded strings
    let package_xml = format!(
        r#"<?xml version="1.0" encoding="UTF-8"?>
        <Package xmlns="http://soap.sforce.com/2006/04/metadata">
            <types>
                <members>file1</members>
                <members>file2</members>
                <name>ApexClass</name>
            </types>
            <version>{}</version>
        </Package>"#,
        env!("CARGO_PKG_VERSION")
    );
    fs::write("package.xml", &package_xml).unwrap();

    // create the zip file with the package.xml file and the base64-encoded Apex files
    let zip_file = format!(
        r#"<?xml version="1.0" encoding="UTF-8"?>
        <ApexClass xmlns="http://soap.sforce.com/2006/04/metadata">
            <apiVersion>{}</apiVersion>
            <status>Active</status>
            <fileName>file1.cls</fileName>
            <content>{}</content>
        </ApexClass>
        <ApexClass xmlns="http://soap.sforce.com/2006/04/metadata">
            <apiVersion>{}</apiVersion>
            <status>Active</status>
            <fileName>file2.cls</fileName>
            <content>{}</content>
        </ApexClass>"#,
        env!("CARGO_PKG_VERSION"),
        file1_base64,
       
        let mut zip_file = Vec::new();
        {
            let mut encoder = GzEncoder::new(zip_file, Compression::default());
            encoder.write_all(zip_file_content.as_bytes()).unwrap();
        }
        zip_file = encoder.finish().unwrap();
        
        // POST the zip file to the /services/data/vXX.X/metadata/deployRequest endpoint
        let response = reqwest::Client::new()
            .post(format!("{}/metadata/deployRequest", SERVER_URL))
            .basic_auth("username", Some("password"))
            .header("Content-Type", "application/zip")
            .body(zip_file)
            .send()
            .unwrap();
        
        // retrieve the deploy result
        let deploy_result: String = response.json().unwrap();
        println!("Deploy result: {}", deploy_result);
        
        // retrieve the Apex classes from the org
        let response = reqwest::Client::new()
            .get(format!("{}/metadata/retrieveRequest", SERVER_URL))
            .basic_auth("username", Some("password"))
            .header("Content-Type", "application/json")
            .body(format!(
                r#"{{"retrieveRequest": {{"apiVersion": "{}", "unpackaged": {{"types": [{{"members": ["file1", "file2"], "name": "ApexClass"}}]}}}}}}"#,
                env!("CARGO_PKG_VERSION")
            ))
            .send()
            .unwrap();
        
        // retrieve the retrieve result
        let retrieve_result: String = response.json().unwrap();
        println!("Retrieve result: {}", retrieve_result);
// parse the retrieve result and extract the base64-encoded Apex files
let retrieve_result_xml = str::from_utf8(&retrieve_result).unwrap();
let file1_base64 = retrieve_result_xml
    .match(r#"<filePath>file1.cls</filePath>\s*<content>(.+?)</content>"#)
    .unwrap()
    .get(1)
    .unwrap();
let file2_base64 = retrieve_result_xml
    .match(r#"<filePath>file2.cls</filePath>\s*<content>(.+?)</content>"#)
    .unwrap()
    .get(1)
    .unwrap();

// decode the base64-encoded Apex files
let file1_decoded = base64::decode(file1_base64).unwrap();
let file2_decoded = base64::decode(file2_base64).unwrap();

// write the Apex files to disk
let mut file1 = fs::File::create("file1.cls").unwrap();
file1.write_all(&file1_decoded).unwrap();
let mut file2 = fs::File::create("file2.cls").unwrap();
file2.write_all(&file2_decoded).unwrap();

println!("Apex files retrieved and saved successfully");
// compile the Apex files
let file1_compiled = compile_apex_file("file1.cls");
let file2_compiled = compile_apex_file("file2.cls");

// print the compiled Apex files
println!("Compiled Apex files:");
println!("file1.cls: {}", file1_compiled);
println!("file2.cls: {}", file2_compiled);
        