// example_03_json_population
// clear; cargo run --example example_03_json_population

//! read a struct vector with serde json
//! write schema to string in qvs20 format
//! write rows to string in qvs20 format
//! write string to file qvs20
//! for comparison write json to string
//! write json to file
//! read qvs20 file

mod duration_mod;
use duration_mod::*;

#[allow(unused_imports)]
use ansi_term::Colour::{Blue, Green, Red, Yellow};
use serde_derive::{Deserialize, Serialize};
use std::fs;
use unwrap::unwrap;
//use env_logger::Env;
//use log::info;

// region: enum, structs, const,...
#[derive(Serialize, Deserialize, Clone)]
pub struct CountryPopulation {
    pub country: String,
    pub population: String,
}

fn main() {
    println!("---start example_03_json_population---");
    // https://github.com/serde-rs/json/issues/160
    // serde json is not optimized for buffer reads ! For now 2020-05-18.
    // this takes too much time: 4710 ms: let f = unwrap!(File::open(path));unwrap!(serde_json::from_reader(f))
    // this is faster, but not good  562ms: let br = unwrap!(BufReader::new(File::open(path)));unwrap!(serde_json::from_reader(br))
    // Read the whole file in a string and parse is the fastest: 55ms


    // region: read from json file to vec struct
    let ns_1 = ns_start();
    let json_text = unwrap!(fs::read_to_string(
        "sample_data/read/country_population.json"
    ));
    let ns_2 = ns_print(ns_1, &s!("  read_1 json file bytes: {}", json_text.len()));
    let country_population: Vec<CountryPopulation> = unwrap!(serde_json::from_str(&json_text));
    ns_print(ns_2, "  read_2 serde_json::from_str()");
    // endregion: read from json file to vec struct

    // now write that in a qvs20 string and file
    let mut output = s!();
    write_qvs20_schema_and_rows_to_file(&mut output, &country_population);

    let ns_3 = ns_start();
    let json_text = unwrap!(serde_json::to_string(&country_population));
    let ns_4 = ns_print(ns_3, "  write_1 serde_json.to_string(");
    let file_name = "sample_data/write/country_population_copy.json";
    unwrap!(fs::write(file_name, &json_text));
    ns_print(ns_4, &s!("  write_2 file json bytes: {}", json_text.len()));

    // read qvs20 for comparison
    read_qvs20();

    println!("---end example_03_json_population---");
}

// only this code works directly with qvs20
use qvs20::*;

/// manually create schema and loop through data to serialize qvs20 to string from struct vector
/// TODO: use derive later
fn write_qvs20_schema_and_rows_to_file(
    output: &mut String,
    country_population: &Vec<CountryPopulation>,
) {
    let ns_1 = ns_start();
    // prepare schema manually into string
    let schema = unwrap!(TableSchema::schema_from_qvs20_str(
        r#"[population][country population table]
[String][Integer]
[][]
[Required][Required]
[Country][Population]
"#,
    ));
    let mut wrt = WriterForQvs20::new(output, &schema);
    wrt.write_schema();

    // prepare rows into string
    for row in country_population.iter() {
        wrt.write_string(&row.country);
        wrt.write_string(&row.population);
        wrt.write_delimiter();
    }
    let ns_2 = ns_print(ns_1, "  write_1 to string qvs20");

    // write string to file
    unwrap!(fs::write(
        "sample_data/write/country_population.qvs20",
        &output
    ));

    ns_print(ns_2, &s!("  write_2 qvs20 bytes: {}", output.len()));
}

/// read from file
fn read_qvs20() {
    let ns_1 = ns_start();
    let qvs20_text = unwrap!(fs::read_to_string(
        "sample_data/write/country_population.qvs20"
    ));
    let ns_2 = ns_print(ns_1, "  read_1 qvs20 file");
    // fill table struct
    let _population_table = unwrap!(Table::from_qvs20_str_with_schema(&qvs20_text));
    ns_print(ns_2, "  read_2 from_qvs20_str_with_schema");
}
