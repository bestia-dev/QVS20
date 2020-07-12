// example_02_write_from_table
// clear; cargo run --example example_02_write_from_table table01_simple_strings.qvs20
// clear; cargo run --example example_02_write_from_table table02_int_decimal_float.qvs20
// clear; cargo run --example example_02_write_from_table table03_sub_table.qvs20

//! From Table, write
//! 1. separate table_schema
//! 2. separate table_rows
//! 3. single string with schema and rows
//! to string.
//! Then it can be stored in files or send over network.

use std::env;
use std::fs;
use std::process;
use unwrap::unwrap;

fn main() {
    println!("---start example_02_write_from_table---");
    let args: Vec<_> = env::args().collect();

    if args.len() != 2 {
        println!(
            "Missing argument file name, usage:    
clear; cargo run --example example_02_write_from_table table01_simple_strings.qvs20
clear; cargo run --example example_02_write_from_table table02_int_decimal_float.qvs20
clear; cargo run --example example_02_write_from_table table03_sub_table.qvs20  "
        );
        process::exit(1);
    }

    let file_name = format!("sample_data/read/{}", &args[1]);
    println!("load file: {}", file_name);
    let text = unwrap!(fs::read_to_string(&file_name));
    let table = read_qvs20(&text);

    let mut output1 = String::with_capacity(1000);
    write_qvs20_only_schema(&table, &mut output1);
    // just print, instead of saving to a file
    println!("table_schema: \n{}", output1);

    let mut output2 = String::with_capacity(1000);
    write_qvs20_only_rows(&table, &mut output2);
    // just print, instead of saving to a file
    println!("table_rows:\n{}", output2);

    let mut output3 = String::with_capacity(1000);
    write_qvs20_table(&table, &mut output3);
    // just print, instead of saving to a file
    println!("table:\n{}", output3);

    println!("---end example_02_write_from_table---");
}

// only this code works directly with qvs20
use qvs20::*;

fn read_qvs20(text: &str) -> Table {
    unwrap!(Table::from_qvs20_str_with_schema(text))
}

fn write_qvs20_only_schema(table: &Table, output: &mut String) {
    let mut wrt = WriterForQvs20::new(output, &table.schema);
    wrt.write_schema();
}

fn write_qvs20_only_rows(table: &Table, output: &mut String) {
    let mut wrt = WriterForQvs20::new(output, &table.schema);
    wrt.write_table_rows(&table.table_rows);
}

fn write_qvs20_table(table: &Table, output: &mut String) {
    let mut wrt = WriterForQvs20::new(output, &table.schema);
    wrt.write_table(&table.table_rows);
}
