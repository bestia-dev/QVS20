// example_03_qvs20_table

// region: lmake_md_to_doc_comments include "DEVELOPMENT.md" //! C

// endregion: lmake_md_to_doc_comments include "DEVELOPMENT.md" //! C

use qvs20::Value;
use qvs20::*;
use rust_decimal::prelude::*;
use std::fs;
use unwrap::unwrap;

#[derive(Debug)]
struct CouDenRow {
    country: String,
    density: Decimal,
}

fn main() {
    println!("---start example_03_qvs20_table---");
    // fill the vector with data
    let vec_of_cou_den_rows = fill_sample_data();

    write_separate_files(&vec_of_cou_den_rows);
    write_one_file(&vec_of_cou_den_rows);

    read_to_table_separate("cou_den3_schema.qvs20", "cou_den3_rows.qvs20");
    read_to_table("cou_den3.qvs20");

    println!("---end example_03_qvs20_table---");
}

fn fill_sample_data() -> Vec<CouDenRow> {
    vec![
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
    ]
}

// write separate files for schema and rows - data
fn write_separate_files(vec_of_cou_den_rows: &Vec<CouDenRow>) {
    let schema = unwrap!(TableSchema::schema_from_qvs20_str(
        r#"[cou_den3][example with country population density]
[String][Decimal]
[][]
[][]
[Country][Density]
"#,
    ));
    let schema_text = schema.write_schema();
    unwrap!(fs::write(
        "sample_data/write/cou_den3_schema.qvs20",
        &schema_text
    ));
    println!("write cou_den3_schema.qvs20:");
    println!("{}", schema_text);

    let mut table_rows = unwrap!(TableRows::new("cou_den3", b'\n'));
    for cou_den_row in vec_of_cou_den_rows.iter() {
        let row = Row {
            values: vec![
                Value::String(cou_den_row.country.clone()),
                Value::Decimal(cou_den_row.density),
            ],
        };
        table_rows.rows.push(row);
    }
    let rows_text = table_rows.write_table_rows();
    unwrap!(fs::write(
        "sample_data/write/cou_den3_rows.qvs20",
        &rows_text
    ));
    println!("write cou_den3_rows.qvs20:");
    println!("{}", rows_text);
}
// write one file for table
fn write_one_file(vec_of_cou_den_rows: &Vec<CouDenRow>) {
    let schema = unwrap!(TableSchema::schema_from_qvs20_str(
        r#"[cou_den3][example with country population density]
[String][Decimal]
[][]
[][]
[Country][Density]
"#,
    ));

    let mut table_rows = unwrap!(TableRows::new("cou_den3", b'\n'));
    for cou_den_row in vec_of_cou_den_rows.iter() {
        let row = Row {
            values: vec![
                Value::String(cou_den_row.country.clone()),
                Value::Decimal(cou_den_row.density),
            ],
        };
        table_rows.rows.push(row);
    }
    let table = Table { schema, table_rows };

    let text = table.write_table();
    unwrap!(fs::write("sample_data/write/cou_den3.qvs20", &text));
    println!("write cou_den3.qvs20:");
    println!("{}", text);
}

/// read to table separate schema and rows
fn read_to_table_separate(schema_file_name: &str, rows_file_name: &str) {
    let schema_text = unwrap!(fs::read_to_string(&format!(
        "sample_data/write/{}",
        schema_file_name
    )));
    let schema = unwrap!(TableSchema::schema_from_qvs20_str(&schema_text));
    let rows_text = unwrap!(fs::read_to_string(&format!(
        "sample_data/write/{}",
        rows_file_name
    )));
    let table_rows = unwrap!(TableRows::rows_from_qvs20_str(&rows_text, &schema));
    let table = Table { schema, table_rows };
    let text = table.write_table();
    println!("read cou_den3_schema.qvs20 and cou_den3_rows.qvs20:");
    println!("{}", text);
}

/// read to table
fn read_to_table(file_name: &str) {
    let text = unwrap!(fs::read_to_string(&format!(
        "sample_data/write/{}",
        file_name
    )));
    let table = unwrap!(Table::from_qvs20_str_with_schema(&text));
    let text = table.write_table();
    println!("read cou_den3.qvs20:");
    println!("{}", text);
}
