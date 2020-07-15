// example_04_manual_derive

// region: lmake_md_to_doc_comments include DEVELOPMENT.md D //!
//! ### 4. Manually write an implementation for a struct
//!
//! Having a struct for rows and one for vector of rows, it is fairly easy to write an implementation that reads and writes data to the qvs20 format. Most of the code is just boilerplate.  
//! `clear; cargo run --example example_04_manual_derive`  
//!
// endregion: lmake_md_to_doc_comments include DEVELOPMENT.md D //!

use qvs20::*;
use rust_decimal::prelude::*;
use std::fs;
use unwrap::unwrap;

#[derive(Debug)]
pub struct CouDenTable (Vec<CouDenRow>);

impl CouDenTable {
    /// constructor
    pub fn new() -> CouDenTable {
        //return
        CouDenTable (vec![] )
    }
    pub fn table_name()->&'static str{
        CouDenRow::table_name()
    }
    /// write schema
    pub fn write_schema(&self, file_name: &str) {
        let text = {
            let mut wrt = WriterForQvs20::new();
            self.write_schema_to_writer(&mut wrt, true);
            wrt.return_and_finish()
        };
        unwrap!(fs::write(file_name, &text));
        println!("write {}_schema.qvs20:",Self::table_name());
        println!("{}", text);
    }
    
    pub fn write_schema_to_writer(&self, wrt: &mut WriterForQvs20, schema_only:bool) {
        let schema = CouDenRow::get_schema();
        schema.write_schema_to_writer(wrt, schema_only);
    }
    /// write rows
    pub fn write_table_rows(&self, file_name: &str) {
        let text = {
            let mut wrt = WriterForQvs20::new();
            wrt.write_string("R");
            wrt.write_string(Self::table_name());
            wrt.write_delimiter();
            self.write_table_rows_to_writer(&mut wrt);
            wrt.return_and_finish()
        };
        unwrap!(fs::write(file_name, &text));
        println!("write {}_rows.qvs20:",Self::table_name());
        println!("{}", text);
    }

    fn write_table_rows_to_writer(&self, wrt: &mut WriterForQvs20) {
        for cou_den_row in self.0.iter() {
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
        println!("write {}.qvs20:",Self::table_name());
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
        if table_name != Self::table_name() {
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

        CouDenRow::read_row_from_reader(&mut rdr, &mut self.0);
    }
    pub fn read_schema_from_file(&mut self, file_name: &str) -> TableSchema {
        let text = unwrap!(fs::read_to_string(file_name));
        let schema = unwrap!(TableSchema::schema_from_qvs20_str(&text));
        // return
        schema
    }
}


#[derive(Debug)]
pub struct CouDenRow {
    pub country: String,
    pub density: Decimal,
}

impl CouDenRow {
    pub fn table_name()->&'static str{
        "cou_den4"
    }
    fn write_row_to_writer(&self, wrt: &mut WriterForQvs20) {
        // for every struct field
        wrt.write_string(&self.country);
        wrt.write_decimal(self.density);
        wrt.write_delimiter();
    }
    fn read_row_from_reader( rdr:&mut ReaderForQvs20,  rows:&mut Vec<Self>){
        while !rdr.peek_next_is_eof() {
            let country = rdr.next_string().unwrap();
            let density = rdr.next_decimal().unwrap();
            rdr.next_row_delimiter().unwrap();
            rows.push(CouDenRow { country, density });
        }
    }
    fn get_schema()->TableSchema{
        let schema = TableSchema{
            table_name: Self::table_name().to_string(),
            table_description:"example with country population density".to_string(),
            data_types:vec![DataType::String,DataType::Decimal],
            sub_table_schemas:vec![None,None],
            additional_properties:vec![String::new(),String::new()],
            column_names:vec!["country".to_string(), "density".to_string()],
            row_delimiter:b'\n',
            sub_table_row_delimiter:b'1',
            ..Default::default()
        };
        schema
    }
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
