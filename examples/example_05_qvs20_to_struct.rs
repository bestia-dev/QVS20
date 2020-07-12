// example_05_qvs20_to_struct
// clear; cargo run --example example_05_qvs20_to_struct

mod duration_mod;
use duration_mod::*;

use qvs20::*;

#[allow(unused_imports)]
use ansi_term::Colour::{Blue, Green, Red, Yellow};
use serde_derive::{Deserialize, Serialize};
use std::fs;
use unwrap::unwrap;
//use env_logger::Env;
//use log::info;

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct CustomerRecords {
    vec: Vec<CustomerRecord>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct CustomerRecord {
    pub date_received: String,
    pub product_name: String,
    pub sub_product: String,
    pub issue: String,
    pub sub_issue: String,
    pub consumer_complaint_narrative: String,
    pub company_public_response: String,
    pub company: String,
    pub state_name: String,
    pub zip_code: String,
    pub tags: String,
    pub consumer_consent_provided: String,
    pub submitted_via: String,
    pub date_sent_to_company: String,
    pub company_response_to_consumer: String,
    pub timely_response: String,
    pub consumer_disputed: String,
    pub complaint_id: i32,
}

/// customers
fn main() {
    println!("---start example_05_qvs20_to_struct---");

    let ns_start = ns_start();
    let qvs20_text = unwrap!(fs::read_to_string(
        "sample_data/write/customer_records.qvs20"
    ));
    let _customer_records = CustomerRecords::new_read(&qvs20_text);
    let _ns_before_serde = ns_print(
        ns_start,
        &format!("  read qvs20 file bytes: {}", qvs20_text.len()),
    );

    println!("---end example_05_qvs20_to_struct---");
}

// generated impl start
// from struct CustomerRecord
mod qvs20_generated {
    use crate::*;

    impl CustomerRecords {
        pub fn new() -> CustomerRecords {
            CustomerRecords { vec: vec![] }
        }
        pub fn new_read(qvs20_text: &str) -> CustomerRecords {
            let mut customer_records = CustomerRecords { vec: vec![] };
            customer_records.read(qvs20_text);
            //return
            customer_records
        }
        pub fn read(&mut self, qvs20_text: &str) {
            let mut rdr = ReaderForQvs20::new(qvs20_text.as_bytes());
            let mut schema = TableSchema::default();
            unwrap!(schema.read_schema(&mut rdr));

            let mut _active_row = 0;
            //while
            while rdr.peek_next_is_not_eof() {
                // data row
                let row = unwrap!(Self::get_one_data_row(&mut rdr, &schema));
                self.vec.push(row);

                //active_row += 1;
            }
        }

        pub fn get_one_data_row(
            rdr: &mut ReaderForQvs20,
            _schema: &TableSchema,
        ) -> Result<CustomerRecord, Qvs20Error> {
            let customer_record = CustomerRecord {
                // The order of fields and datatypes is important here and known beforehand.
                // All fields are mandatory.
                date_received: rdr.next_string()?,
                product_name: rdr.next_string()?,
                sub_product: rdr.next_string()?,
                issue: rdr.next_string()?,
                sub_issue: rdr.next_string()?,
                consumer_complaint_narrative: rdr.next_string()?,
                company_public_response: rdr.next_string()?,
                company: rdr.next_string()?,
                state_name: rdr.next_string()?,
                zip_code: rdr.next_string()?,
                tags: rdr.next_string()?,
                consumer_consent_provided: rdr.next_string()?,
                submitted_via: rdr.next_string()?,
                date_sent_to_company: rdr.next_string()?,
                company_response_to_consumer: rdr.next_string()?,
                timely_response: rdr.next_string()?,
                consumer_disputed: rdr.next_string()?,
                complaint_id: rdr.next_integer()? as i32,
            };
            rdr.next_row_delimiter()?;
            //return
            Ok(customer_record)
        }
    }
}
