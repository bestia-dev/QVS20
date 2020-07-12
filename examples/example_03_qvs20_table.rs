// example_03_qvs20_table

// region: lmake_md_to_doc_comments include "DEVELOPMENT.md" //! C

// endregion: lmake_md_to_doc_comments include "DEVELOPMENT.md" //! C

//! The table structure is very flexible, because it is defined in runtime.
//! Its method from_qvs20_str_with_schema(&str) fills it from a single str
//! that is read from a file or network.
// clear; cargo run --example example_01_read_to_table table01_simple_strings.qvs20
// clear; cargo run --example example_01_read_to_table table02_int_decimal_float.qvs20
// clear; cargo run --example example_01_read_to_table table03_sub_table.qvs20

use std::env;
use std::fs;
use std::process;
use unwrap::unwrap;

/// starting function
fn main() {
    println!("---start example_01_read_to_table---");
    let args: Vec<_> = env::args().collect();

    if args.len() != 2 {
        println!(
            "Missing argument for qvs20 file name, usage:    
clear; cargo run --example example_01_read_to_table table01_simple_strings.qvs20
clear; cargo run --example example_01_read_to_table table02_int_decimal_float.qvs20
clear; cargo run --example example_01_read_to_table table03_sub_table.qvs20
"
        );
        process::exit(1);
    }

    let file_name = format!("sample_data/read/{}", &args[1]);
    println!("load file: {}", file_name);
    let text = unwrap!(fs::read_to_string(&file_name));

    let table = get_qvs20_table(&text);
    println!("{:#?}", table);

    println!("---end example_01_read_to_table---");
}

/// only this code works directly with qvs20
use qvs20::*;

fn get_qvs20_table(text: &str) -> Table {
    unwrap!(Table::from_qvs20_str_with_schema(&text))
}
