// example_02_reader_writer

// region: lmake_md_to_doc_comments include "DEVELOPMENT.md" //! B

// endregion: lmake_md_to_doc_comments include "DEVELOPMENT.md" //! B

use qvs20::ReaderForQvs20;
use qvs20::WriterForQvs20;
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
    println!("---start example_02_reader_writer---");
    // fill the vector with data
    let vec_of_cou_den_rows = fill_sample_data();

    write_schema(&vec_of_cou_den_rows);
    write_rows(&vec_of_cou_den_rows);
    write_one_file(&vec_of_cou_den_rows);

    read_with_reader("cou_den2_rows.qvs20");
    read_with_reader("cou_den2.qvs20");

    println!("---end example_02_reader_writer---");
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
fn write_schema(_vec_of_cou_den_rows: &Vec<CouDenRow>) {
    // Separate qvs20 schema file
    // return String from block, so the writer is dropped soon and correctly
    let schema_text = {
        let mut wrt = WriterForQvs20::new();
        wrt.write_vec_of_string_as_row(&vec![
            "S",
            "cou_den2",
            "example with country population density",
        ]);
        // data types
        wrt.write_vec_of_string_as_row(&vec!["String", "Decimal"]);
        // write sub_table_schemas
        wrt.write_vec_of_string_as_row(&vec!["", ""]);
        // write additional_properties
        wrt.write_vec_of_string_as_row(&vec!["", ""]);
        // write column_names
        wrt.write_vec_of_string_as_row(&vec!["Country", "Density"]);
        // return String from block
        wrt.return_and_finish()
    };
    unwrap!(fs::write(
        "sample_data/write/cou_den2_schema.qvs20",
        &schema_text
    ));
    println!("write cou_den2_schema.qvs20:");
    println!("{}", schema_text);
}

// write separate files for schema and rows - data
fn write_rows(vec_of_cou_den_rows: &Vec<CouDenRow>) {
    
    // return String from block, so the writer is dropped soon and correctly
    let rows_text = {
        // Separate file for qvs20 rows - data.
        let mut wrt = WriterForQvs20::new();
        // First row is file type and table name. Always end row with delimiter \n.
        wrt.write_string("R");
        wrt.write_string("cou_den2");
        wrt.write_delimiter();
        for row in vec_of_cou_den_rows.iter() {
            wrt.write_string(&row.country);
            wrt.write_decimal(row.density);
            wrt.write_delimiter();
        }
        //return the String from block
        wrt.return_and_finish()
    };
    unwrap!(fs::write(
        "sample_data/write/cou_den2_rows.qvs20",
        &rows_text
    ));
    println!("write cou_den2_rows.qvs20:");
    println!("{}", rows_text);
}

// write one file for schema and rows - data
fn write_one_file(vec_of_cou_den_rows: &Vec<CouDenRow>) {
    // return String from block, so the writer is dropped soon and correctly
    let text = {
        let mut wrt = WriterForQvs20::new();
        // 5 rows for schema
        wrt.write_vec_of_string_as_row(&vec![
            "T",
            "cou_den2",
            "example with country population density",
        ]);
        // data types
        wrt.write_vec_of_string_as_row(&vec!["String", "Decimal"]);
        // write sub_table_schemas
        wrt.write_vec_of_string_as_row(&vec!["", ""]);
        // write additional_properties
        wrt.write_vec_of_string_as_row(&vec!["", ""]);
        // write column_names
        wrt.write_vec_of_string_as_row(&vec!["Country", "Density"]);

        // qvs20 rows - data.
        for row in vec_of_cou_den_rows.iter() {
            wrt.write_string(&row.country);
            wrt.write_decimal(row.density);
            wrt.write_delimiter();
        }
        //return the String from block
        wrt.return_and_finish()
    };
    unwrap!(fs::write("sample_data/write/cou_den2.qvs20", &text));
    println!("write cou_den2.qvs20:");
    println!("{}", text);
}

/// read with reader
fn read_with_reader(file_name: &str) {
    let mut vec_of_cou_den_rows: Vec<CouDenRow> = vec![];
    // We don't need to use the schema file in our super simple code.
    // But we (the developers) have to read the schema file and learn the structure.
    // We don't need any other information, just read the schema file.

    let text = unwrap!(fs::read_to_string(&format!(
        "sample_data/write/{}",
        file_name
    )));
    let mut rdr = ReaderForQvs20::new(text.as_bytes());
    let mut vec_of_string = unwrap!(rdr.next_row_as_vec_of_string());

    // move out of vector. Warning: can be used only once!
    // The next time it will be the wrong value without any error.
    let file_type = std::mem::replace(&mut vec_of_string[0], s!());
    let table_name = std::mem::replace(&mut vec_of_string[1], s!());
    if table_name != "cou_den2" {
        panic!("wrong table name");
    }
    // the next column defines if the files has the schema or not
    if file_type=="R" {
        // only rows - data
    } else if file_type == "T" || file_type == "S" {
        // schema with 5 rows
        // move out of vector. Warning: can be used only once!
        // The next time it will be the wrong value without any error.
        let _description = std::mem::replace(&mut vec_of_string[1], s!());
        // drop the vector because it has not the originals values anymore
        drop(vec_of_string);
        let vec_of_data_types = unwrap!(rdr.next_row_as_vec_of_string());
        let _vec_of_sub_table_schemas = unwrap!(rdr.next_row_as_vec_of_string());
        let _vec_of_additional_properties = unwrap!(rdr.next_row_as_vec_of_string());
        let vec_of_column_names = unwrap!(rdr.next_row_as_vec_of_string());
        dbg!(&vec_of_data_types);
        dbg!(&vec_of_column_names);
    } else {
        panic!("first row is not correct");
    }

    // this is the start of rows - data
    while !rdr.peek_next_is_eof() {
        let mut vec_of_string = unwrap!(rdr.next_row_as_vec_of_string());
        // move out of vector. Warning: can be used only once!
        // The next time it will be the wrong value without any error.
        let country = std::mem::replace(&mut vec_of_string[0], s!());
        let density = std::mem::replace(&mut vec_of_string[1], s!());
        drop(vec_of_string);
        let density = unwrap!(Decimal::from_str(&density));
        vec_of_cou_den_rows.push(CouDenRow { country, density });
    }
    println!("Read with reader: {}", file_name);
    dbg!(vec_of_cou_den_rows);
}
