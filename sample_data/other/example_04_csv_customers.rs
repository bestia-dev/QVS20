// example_04_csv_customers
// clear; cargo run --example example_04_csv_customers

mod duration_mod;
use duration_mod::*;

use qvs20::*;

use std::fs;
use unwrap::unwrap;
//use serde::Serialize;
#[allow(unused_imports)]
use ansi_term::Colour::{Blue, Green, Red, Yellow};
use serde_derive::{Deserialize, Serialize};
//use env_logger::Env;
//use log::info;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CustomerRecord {
    #[serde(rename = "Date Received")]
    pub date_received: String,
    #[serde(rename = "Product Name")]
    pub product_name: String,
    #[serde(rename = "Sub Product")]
    pub sub_product: String,
    #[serde(rename = "Issue")]
    pub issue: String,
    #[serde(rename = "Sub Issue")]
    pub sub_issue: String,
    #[serde(rename = "Consumer Complaint Narrative")]
    pub consumer_complaint_narrative: String,
    #[serde(rename = "Company Public Response")]
    pub company_public_response: String,
    #[serde(rename = "Company")]
    pub company: String,
    #[serde(rename = "State Name")]
    pub state_name: String,
    #[serde(rename = "Zip Code")]
    pub zip_code: String,
    #[serde(rename = "Tags")]
    pub tags: String,
    #[serde(rename = "Consumer Consent Provided")]
    pub consumer_consent_provided: String,
    #[serde(rename = "Submitted via")]
    pub submitted_via: String,
    #[serde(rename = "Date Sent to Company")]
    pub date_sent_to_company: String,
    #[serde(rename = "Company Response to Consumer")]
    pub company_response_to_consumer: String,
    #[serde(rename = "Timely Response")]
    pub timely_response: String,
    #[serde(rename = "Consumer Disputed")]
    pub consumer_disputed: String,
    #[serde(rename = "Complaint ID")]
    pub complaint_id: i32,
}

/// customers
fn main() {
    println!("---start example_04_csv_customers---");

    let ns_start = ns_start();
    let mut customer_records: Vec<CustomerRecord> = vec![];
    let csv_text = unwrap!(fs::read_to_string("sample_data/read/P9-Consumers.csv"));
    let ns_before_serde = ns_print(
        ns_start,
        &format!("  read_1 csv file bytes: {}", csv_text.len()),
    );
    let mut rdr = csv::Reader::from_reader(csv_text.as_bytes());
    for result in rdr.deserialize() {
        let record: CustomerRecord = unwrap!(result);
        // println!("{:?}", &record);
        customer_records.push(record);
    }
    let ns_before_qvs20 = ns_print(ns_before_serde, "  read_2 rdr.deserialize()");

    // prepare schema manually
    let schema = unwrap!(TableSchema::schema_from_qvs20_str(
        r#"[S][customer records][big csv table]
[String][String][String][String][String][String][String][String][String][String][String][String][String][String][String][String][String][Integer]
[][][][][][][][][][][][][][][][][][]
[][][][][][][][][][][][][][][][][][]
[Date Received][Product Name][Sub Product][Issue][Sub Issue][Consumer Complaint Narrative][Company Public Response][Company][State Name][Zip Code][Tags][Consumer Consent Provided][Submitted via][Date Sent to Company][Company Response to Consumer][Timely Response][Consumer Disputed][Complaint ID]
"#,
    ));
    // println!("{:#?}", schema);
    let mut wrt = WriterForQvs20::new();
    schema.write_schema_to_writer(&mut wrt);

    for row in customer_records.iter() {
        wrt.write_string(&row.date_received);
        wrt.write_string(&row.product_name);
        wrt.write_string(&row.sub_product);
        wrt.write_string(&row.issue);
        wrt.write_string(&row.sub_issue);
        wrt.write_string(&row.consumer_complaint_narrative);
        wrt.write_string(&row.company_public_response);
        wrt.write_string(&row.company);
        wrt.write_string(&row.state_name);
        wrt.write_string(&row.zip_code);
        wrt.write_string(&row.tags);
        wrt.write_string(&row.consumer_consent_provided);
        wrt.write_string(&row.submitted_via);
        wrt.write_string(&row.date_sent_to_company);
        wrt.write_string(&row.company_response_to_consumer);
        wrt.write_string(&row.timely_response);
        wrt.write_string(&row.consumer_disputed);
        wrt.write_integer(row.complaint_id as i64);
        wrt.write_delimiter();
    }
    let ns_before_write = ns_print(ns_before_qvs20, "  write_1 to string qvs20");
    let qvs20_string = wrt.return_and_finish();
    unwrap!(fs::write(
        "sample_data/write/customer_records.qvs20",
        &qvs20_string
    ));
    let ns_write_qvs20 = ns_print(
        ns_before_write,
        &format!(
            "  write_2 file customer_records.qvs20 bytes: {}",
            qvs20_string.len()
        ),
    );

    let mut wtr = csv::Writer::from_writer(vec![]);
    for row in customer_records.iter() {
        unwrap!(wtr.serialize(row));
    }
    let csv_text = unwrap!(String::from_utf8(unwrap!(wtr.into_inner())));
    let ns_before_write1_csv = ns_print(ns_write_qvs20, "  write_1 wtr.serialize()");
    unwrap!(fs::write(
        "sample_data/customer_records_copy.csv",
        &csv_text
    ));
    let ns_end_csv_write = ns_print(
        ns_before_write1_csv,
        &format!(
            "  write_2 file customer_records_copy.csv bytes: {}",
            csv_text.len()
        ),
    );

    let qvs20_text = unwrap!(fs::read_to_string(
        "sample_data/write/customer_records.qvs20"
    ));
    let ns_before_qvs20 = ns_print(ns_end_csv_write, "  read_1 qvs20 file");
    let _customer_records_table = unwrap!(Table::from_qvs20_str_with_schema(&qvs20_text));
    let _ns_after_from_qvs20 = ns_print(ns_before_qvs20, "  read_2 from_qvs20_str_with_schema");

    println!("---end example_04_csv_customers---");
}
