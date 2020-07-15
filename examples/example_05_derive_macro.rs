// example_05_derive_macro

// region: lmake_md_to_doc_comments include DEVELOPMENT.md E //!
//! ### 5. Derive macro
//!
//! Rust can codegen (code generation) the implementation in compile time, so the developer don't need to do it manually.  
//! It is achieved by a procedural derive macro in a separate crate: qvs20_derive.  
//! `clear; cargo run --example example_05_derive_macro`  
//! Expand derive macro:  
//! `clear; cargo expand --example example_05_derive_macro`  
//!
// endregion: lmake_md_to_doc_comments include DEVELOPMENT.md E //!


use qvs20_derive::{Qvs20Row,Qvs20Table};
use qvs20::*;
use rust_decimal::prelude::*;
use std::fs;
use unwrap::unwrap;

#[derive(Qvs20Table, Debug)]
pub struct CouDenTable (Vec<CouDenRow>);

#[derive(Qvs20Row, Debug)]
#[Qvs20TableName = "cou_den5"]
#[Qvs20Description = "example with country population density"]
pub struct CouDenRow {
    pub country: String,
    #[Qvs20Type = "Decimal"]
    pub density: Decimal,
}

fn main() {
    println!("---start example_05_derive_macro---");
    // fill the vector with data
    let mut cou_den_table = CouDenTable::new();
    fill_with_sample_data(&mut cou_den_table);

    cou_den_table.write_schema("sample_data/write/cou_den5_schema.qvs20");
    cou_den_table.write_table_rows("sample_data/write/cou_den5_rows.qvs20");
    cou_den_table.write_one_file("sample_data/write/cou_den5.qvs20");

    let mut cou_den_table = CouDenTable::new();
    let schema = cou_den_table.read_schema_from_file("sample_data/write/cou_den5_schema.qvs20");
    println!("read sample_data/write/cou_den5_schema.qvs20:");
    dbg!(schema);
    cou_den_table.read_from_file("sample_data/write/cou_den5_rows.qvs20");
    println!("read sample_data/write/cou_den5_rows.qvs20:");
    dbg!(cou_den_table);
    
    let mut cou_den_table = CouDenTable::new();
    cou_den_table.read_from_file("sample_data/write/cou_den5.qvs20");
    println!("read sample_data/write/cou_den5.qvs20:");
    dbg!(cou_den_table);
    println!("---end example_05_derive_macro---");
}


/// this is not a part of the implementation, but just for the example
pub fn fill_with_sample_data(cou_den_table: &mut CouDenTable) {
    cou_den_table.0 = vec![
        CouDenRow {
            country: "Slovenia".to_string(),
            density: unwrap!(Decimal::from_str("102.6398595")),
        },
        CouDenRow {
            country: "Italy".to_string(),
            density: unwrap!(Decimal::from_str("205.4507479")),
        },
        CouDenRow {
            country: "Falkland Islands".to_string(),
            density: unwrap!(Decimal::from_str("0.28")),
        },
        CouDenRow {
            country: "Macao".to_string(),
            density: unwrap!(Decimal::from_str("20777.50026")),
        },
    ];
}
