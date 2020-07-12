// example_02_reader_writer

// region: lmake_md_to_doc_comments include "DEVELOPMENT.md" //! B

// endregion: lmake_md_to_doc_comments include "DEVELOPMENT.md" //! B

use qvs20::*;
use rust_decimal::prelude::*;
use std::fs;
use unwrap::unwrap;
use qvs20::WriterForQvs20;
use qvs20::ReaderForQvs20;

#[derive(Debug)]
struct Qvs20Row {
    country: String,
    density: Decimal,
}

fn main() {
    println!("---start example_02_reader_writer---");
    // fill the vector with data
    let vec_of_rows = fill_sample_data();

    write_separate_files(&vec_of_rows);
    /*
    write_one_file(&vec_of_rows);

    read_with_find("cou_den2_rows.qvs20");
    read_with_find("cou_den2.qvs20");

    read_with_regex("cou_den2_rows.qvs20");
    read_with_regex("cou_den2.qvs20");
*/
    println!("---end example_02_reader_writer---");
}

fn fill_sample_data() -> Vec<Qvs20Row> {
    vec![
        Qvs20Row {
            country: "Slovenia".to_string(),
            density: unwrap!(Decimal::from_str("102.6398595")),
        },
        Qvs20Row {
            country: "Italy".to_string(),
            density: unwrap!(Decimal::from_str("205.4507479")),
        },
        Qvs20Row {
            country: "Falkland Islands".to_string(),
            density: unwrap!(Decimal::from_str("0.28")),
        },
        Qvs20Row {
            country: "Macao".to_string(),
            density: unwrap!(Decimal::from_str("20777.50026")),
        },
    ]
}

// write separate files for schema and rows - data
fn write_separate_files(vec_of_rows: &Vec<Qvs20Row>) {
    // Separate qvs20 schema file
    let mut wrt = WriterForQvs20::new();
    wrt.write_string("cou_den2");
    wrt.write_string("example with country population density");
    wrt.write_delimiter();
    // data types
    wrt.write_string("String");
    wrt.write_string("Decimal");
    wrt.write_delimiter();
    // write sub_table_schemas
    wrt.write_string("");
    wrt.write_string("");
    wrt.write_delimiter();
    // write additional_properties
    wrt.write_string("");
    wrt.write_string("");
    wrt.write_delimiter();
    // write column_names
    wrt.write_string("Country");
    wrt.write_string("Density");
    wrt.write_delimiter();
    let schema_text = wrt.move_output_string_out_of_struct();
    unwrap!(fs::write(
        "sample_data/write/cou_den2_schema.qvs20",
        &schema_text
    ));
    println!("cou_den2_schema.qvs20:");
    println!("{}", schema_text);

    // Separate file for qvs20 rows - data.
    let mut wrt = WriterForQvs20::new();
    // First row is table name. Always end row with delimiter \n.
    wrt.write_string("cou_den2");
    wrt.write_delimiter();
    for row in vec_of_rows.iter() {
        wrt.write_string(&row.country);
        wrt.write_string(&row.density.to_string());
        wrt.write_delimiter();
    }
    let rows_text = wrt.move_output_string_out_of_struct();
    unwrap!(fs::write(
        "sample_data/write/cou_den2_rows.qvs20",
        &rows_text
    ));
    println!("cou_den2_rows.qvs20:");
    println!("{}", rows_text);
}
/*
// write one file for schema and rows - data
fn write_one_file(vec_of_rows: &Vec<Qvs20Row>) {
    // qvs20 schema file is simple to write manually in a string.
    // Remember the rows meaning:
    // 1. table name, description
    // 2. data types of columns
    // 3. sub table schema
    // 4. additional properties
    // 5. column names
    // Nowhere extra spaces, delimiter is exactly \n.
    // Escaping the 6 special characters ([,],\,\n,\r,\t) is very very rare here.
    let mut text = String::with_capacity(200);
    text.push_str(
        "[cou_den2][example with country population density]
[String][Decimal]
[][]
[][]
[Country][Density]
",
    );
    // qvs20 rows - data.
    // We know the data and that there is no need for escaping the 6 special character.
    for row in vec_of_rows.iter() {
        text.push('[');
        text.push_str(&row.country);
        text.push(']');
        text.push('[');
        text.push_str(&row.density.to_string());
        text.push(']');
        text.push('\n');
    }
    unwrap!(fs::write("sample_data/write/cou_den2.qvs20", &text));
    println!("cou_den2.qvs20:");
    println!("{}", text);
}

// region: read with find

/// use find() for the next delimiter
/// this fn is calling 3 helper functions
fn read_with_find(file_name: &str) {
    let mut vec_of_rows: Vec<Qvs20Row> = vec![];
    // We don't need to use the schema file in our super simple code.
    // But we (the developers) have to read the schema file and learn the structure.
    // We don't need any other information, just read the schema file.

    let text = unwrap!(fs::read_to_string(&format!(
        "sample_data/write/{}",
        file_name
    )));
    // we can use the first row to check if we opened the right file
    let mut pos_cursor = 0;
    let string_value = read_next_column(&text, &mut pos_cursor);
    if string_value != "cou_den2" {
        panic!("wrong table name");
    }

    check_table_name_and_jump_over_schema_with_find(&text, &mut pos_cursor);

    // this is the start of rows - data
    while pos_cursor < text.len() {
        let country = read_next_column(&text, &mut pos_cursor);
        let density = read_next_column(&text, &mut pos_cursor);
        let density = unwrap!(Decimal::from_str(&density));
        read_delimiter(&text, &mut pos_cursor);
        vec_of_rows.push(Qvs20Row { country, density });
    }
    println!("Parsed from file with find: {}", file_name);
    dbg!(vec_of_rows);
}

/// The first column of the first row is always the table_name.
pub fn check_table_name_and_jump_over_schema_with_find(text: &str, pos_cursor: &mut usize) {
    // We can deduce if the schema is included in this file or not.
    // When there is a schema, the first row has 2 columns (table_name and description).
    // Else it has only one column (table_name).
    if &text[*pos_cursor..*pos_cursor + 1] == "\n" {
        // this file has only rows - data
        *pos_cursor += 1;
    } else {
        // this file has the schema
        // we don't need any data from the schema,
        // we will just jump over the schema 5 rows
        let mut row_count = 0;
        loop {
            if let Some(pos_row_delimiter) = find_from(&text, *pos_cursor, '\n') {
                *pos_cursor = pos_row_delimiter + 1;
                row_count += 1;
                if row_count == 5 {
                    break;
                }
            } else {
                panic!("error. the schema is not complete.");
            }
        }
        if row_count != 5 {
            panic!("error. the schema is not complete.");
        }
    }
}

/// find from position
pub fn find_from(source_str: &str, pos_cursor: usize, find_char: char) -> Option<usize> {
    let slice01 = &source_str[pos_cursor..];
    let opt_pos_found = slice01.find(find_char);
    if let Some(pos_found) = opt_pos_found {
        return Some(pos_cursor + pos_found);
    } else {
        return None;
    }
}

/// read next column that is delimiter with [ ]
/// we already know this data and we are 100% sure
/// there is no need for unescaping the 6 special characters
fn read_next_column(text: &str, pos_cursor: &mut usize) -> String {
    if &text[*pos_cursor..*pos_cursor + 1] != "[" {
        panic!("error. The first character must be [.")
    }
    let pos_start = *pos_cursor + 1;
    if let Some(pos_end) = find_from(text, pos_start, ']') {
        *pos_cursor = pos_end + 1;
        return text[pos_start..pos_end].to_string();
    } else {
        panic!("error. value must end with ].")
    }
}

/// row delimiter
fn read_delimiter(text: &str, pos_cursor: &mut usize) {
    if &text[*pos_cursor..*pos_cursor + 1] != "\n" {
        panic!("error. Expected row delimiter.")
    }
    *pos_cursor += 1;
}

// endregion: read with find

// region: read with regex

use regex::Regex;
/// read using regex
fn read_with_regex(file_name: &str) {
    let mut vec_of_rows: Vec<Qvs20Row> = vec![];
    // We don't need to use the schema file in our super simple code.
    // But we (the developers) have to read the schema file and learn the structure.
    // We don't need any other information, just read the schema file.

    let text = unwrap!(fs::read_to_string(&format!(
        "sample_data/write/{}",
        file_name
    )));
    // iterator of lines/rows
    let mut lines = text.lines();
    let first_row = unwrap!(lines.next());
    check_table_name_and_jump_over_schema_with_regex(first_row, &mut lines);

    while let Some(line) = lines.next() {
        // the developer knows from the schema there are 2 columns
        // regex capture group ([^\]]*?) for capturing the column value
        // any character except ] zero or more times not-greedy
        let regex_2_columns: Regex = unwrap!(Regex::new(r#"^\[([^\]]*?)\]\[([^\]]*?)\]$"#));
        if let Some(cap) = regex_2_columns.captures(line) {
            let country = unwrap!(cap.get(1)).as_str().to_string();
            let density = unwrap!(cap.get(2)).as_str();
            let density = unwrap!(Decimal::from_str(&density));
            vec_of_rows.push(Qvs20Row { country, density });
        }
    }
    println!("Parsed from file with regex: {}", file_name);
    dbg!(vec_of_rows);
}

/// The first column of the first row is always the table_name.
fn check_table_name_and_jump_over_schema_with_regex(
    first_row: &str,
    lines: &mut std::str::Lines<'_>,
) {
    let mut table_name = "";
    // regex capture group ([^\]]*?) for getting the table_name
    // any character except ] zero or more times not-greedy
    // check if the first row has only one column. Then we have only rows.
    let regex_table_name_1: Regex = unwrap!(Regex::new(r#"^\[([^\]]*?)\]$"#));
    if let Some(cap) = regex_table_name_1.captures(first_row) {
        table_name = unwrap!(cap.get(1)).as_str();
        println!("table_name no schema: {}", table_name);
    // The next line/row is data
    } else {
        // check if the first row has 2 columns. Then we have the schema.
        let regex_table_name_2: Regex = unwrap!(Regex::new(r#"^\[([^\]]*?)\]\[[^\]]*?\]$"#));
        if let Some(cap) = regex_table_name_2.captures(first_row) {
            table_name = unwrap!(cap.get(1)).as_str();
            println!("table_name with schema: {}", table_name);
            // jump over the first 5 rows.
            let _x = unwrap!(lines.next());
            let _x = unwrap!(lines.next());
            let _x = unwrap!(lines.next());
            let _x = unwrap!(lines.next());
            // The next line/row is data
        }
    }
    if table_name != "cou_den2" {
        panic!("error: wrong table name.");
    }
}

// endregion: read with regex
*/