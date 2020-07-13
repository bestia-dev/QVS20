// example_04_manual_derive

// region: lmake_md_to_doc_comments include "DEVELOPMENT.md" //! D

// endregion: lmake_md_to_doc_comments include "DEVELOPMENT.md" //! D

use qvs20::*;
use rust_decimal::prelude::*;
use std::fs;
use unwrap::unwrap;

#[derive(Debug)]
pub struct CouDenTable {
    pub rows: Vec<CouDenRow>,
}

#[derive(Debug)]
pub struct CouDenRow {
    pub country: String,
    pub density: Decimal,
}

fn main() {
    println!("---start example_04_manual_derive---");
    // fill the vector with data
    let mut cou_den_table = CouDenTable::new();
    fill_with_sample_data(&mut cou_den_table);

    cou_den_table.write_schema("sample_data/write/cou_den4_schema.qvs20");
    cou_den_table.write_table_rows("sample_data/write/cou_den4_rows.qvs20");
    cou_den_table.write_one_file("sample_data/write/cou_den4.qvs20");

    let mut cou_den_table = CouDenTable::new();
    let schema = cou_den_table.read_schema_from_file("sample_data/write/cou_den4_schema.qvs20");
    println!("read sample_data/write/cou_den4_schema.qvs20:");
    dbg!(schema);
    cou_den_table.read_from_file("sample_data/write/cou_den4_rows.qvs20");
    println!("read sample_data/write/cou_den4_rows.qvs20:");
    dbg!(cou_den_table);
    
    let mut cou_den_table = CouDenTable::new();
    cou_den_table.read_from_file("sample_data/write/cou_den4.qvs20");
    println!("read sample_data/write/cou_den4.qvs20:");
    dbg!(cou_den_table);

    println!("---end example_04_manual_derive---");
}

/// this is not a part of the implementation, but just for the example
pub fn fill_with_sample_data(cou_den_table: &mut CouDenTable) {
    cou_den_table.rows = vec![
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

impl CouDenTable {
    /// constructor
    pub fn new() -> CouDenTable {
        //return
        CouDenTable { rows: vec![] }
    }
    /// write schema
    pub fn write_schema(&self, file_name: &str) {
        let text = {
            let mut wrt = WriterForQvs20::new();
            self.write_schema_to_writer(&mut wrt, true);
            wrt.return_and_finish()
        };
        unwrap!(fs::write(file_name, &text));
        println!("write cou_den4_schema.qvs20:");
        println!("{}", text);
    }

    fn write_schema_to_writer(&self, wrt: &mut WriterForQvs20, schema_only:bool) {
        let schema = unwrap!(TableSchema::schema_from_qvs20_str(
            r#"[S][cou_den4][example with country population density]
[String][Decimal]
[][]
[][]
[Country][Density]
"#,
        ));
        schema.write_schema_to_writer(wrt, schema_only);
    }

    /// write rows
    pub fn write_table_rows(&self, file_name: &str) {
        let text = {
            let mut wrt = WriterForQvs20::new();
            wrt.write_string("R");
            wrt.write_string("cou_den4");
            wrt.write_delimiter();
            self.write_table_rows_to_writer(&mut wrt);
            wrt.return_and_finish()
        };
        unwrap!(fs::write(file_name, &text));
        println!("write cou_den4_rows.qvs20:");
        println!("{}", text);
    }

    fn write_table_rows_to_writer(&self, wrt: &mut WriterForQvs20) {
        for cou_den_row in self.rows.iter() {
            cou_den_row.write_row_to_writer(wrt);
        }
    }

    /// write one file for table
    pub fn write_one_file(&self, file_name: &str) {
        let text = {
            let mut wrt = WriterForQvs20::new();
            self.write_schema_to_writer(&mut wrt, false);
            self.write_table_rows_to_writer(&mut wrt);
            wrt.return_and_finish()
        };
        unwrap!(fs::write(file_name, &text));
        println!("write cou_den4.qvs20:");
        println!("{}", text);
    }

    pub fn read_from_file(&mut self, file_name: &str) {
        let text = unwrap!(fs::read_to_string(file_name));
        let mut rdr = ReaderForQvs20::new(text.as_bytes());
        let mut vec_of_string = unwrap!(rdr.next_row_as_vec_of_string());
        // move out of vector. Warning: can be used only once!
        // The next time it will be the wrong value without any error.
        let file_type = std::mem::replace(&mut vec_of_string[0], s!());
        let table_name = std::mem::replace(&mut vec_of_string[1], s!());
        if table_name != "cou_den4" {
            panic!("wrong table name");
        }
        // the next column defines if the files has the schema or not
        if file_type == "R" {
            // only rows - data
        } else if file_type == "T" || file_type == "S" {
            // schema with 5 rows
            // move out of vector. Warning: can be used only once!
            // The next time it will be the wrong value without any error.
            let _description = std::mem::replace(&mut vec_of_string[1], s!());
            // drop the vector because it has not the originals values anymore
            drop(vec_of_string);
            let _vec_of_data_types = unwrap!(rdr.next_row_as_vec_of_string());
            let _vec_of_sub_table_schemas = unwrap!(rdr.next_row_as_vec_of_string());
            let _vec_of_additional_properties = unwrap!(rdr.next_row_as_vec_of_string());
            let _vec_of_column_names = unwrap!(rdr.next_row_as_vec_of_string());
        } else {
            panic!("first row is not correct");
        }

        // this is the start of rows - data
        while !rdr.peek_next_is_eof() {
            let country = unwrap!(rdr.next_string());
            let density = unwrap!(rdr.next_decimal());
            unwrap!(rdr.next_row_delimiter());
            self.rows.push(CouDenRow { country, density });
        }
    }
    pub fn read_schema_from_file(&mut self, file_name: &str) -> TableSchema {
        let text = unwrap!(fs::read_to_string(file_name));
        let schema = unwrap!(TableSchema::schema_from_qvs20_str(&text));
        // return
        schema
    }
}
impl CouDenRow {
    pub fn write_row_to_writer(&self, wrt: &mut WriterForQvs20) {
        // for every struct field
        wrt.write_string(&self.country);
        wrt.write_decimal(self.density);
        wrt.write_delimiter();
    }
}
