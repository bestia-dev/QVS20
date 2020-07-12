// example_06_derive_manual
// clear; cargo run --example example_06_derive_manual

//! The same code that the derive macro will eventually generate

use qvs20::*;
use unwrap::unwrap;

struct Customers {
    vec: Vec<Customer>,
}

struct Customer {
    one: i32,
    two: i32,
    three: String,
}

fn main() {
    println!("---start example_06_derive_manual---");
    write_to_file();
}
/// create table programmatically, then export to qvs20 and save to file
fn write_to_file() {
    let mut customers = Customers { vec: vec![] };
    customers.vec.push(Customer {
        one: 11,
        two: 12,
        three: s!("13"),
    });
    customers.vec.push(Customer {
        one: 21,
        two: 32,
        three: s!("43"),
    });
    //let qvs20_text =
}

impl Customers {
    pub fn new() -> Customers {
        Customers { vec: vec![] }
    }
    pub fn new_read(qvs20_text: &str) -> Customers {
        let mut customers = Self::new();
        customers.read(qvs20_text);
        //return
        customers
    }
    pub fn read(&mut self, qvs20_text: &str) {
        let mut rdr = ReaderForQvs20::new(qvs20_text.as_bytes());
        let mut schema = TableSchema::default();
        unwrap!(schema.read_schema(&mut rdr));

        let mut active_row = 0;
        while rdr.peek_next_is_not_eof() {
            let row = unwrap!(Self::get_one_data_row(&mut rdr, &schema));
            self.vec.push(row);
            active_row += 1;
        }
    }
    /// The order of fields and datatypes is important here and known beforehand.
    // All fields are mandatory, because it deserialize sequentially.
    fn get_one_data_row(
        rdr: &mut ReaderForQvs20,
        schema: &TableSchema,
    ) -> Result<Customer, Qvs20Error> {
        let customer = Customer {
            one: rdr.next_integer()? as i32,
            two: rdr.next_integer()? as i32,
            three: rdr.next_string()?,
        };
        rdr.next_row_delimiter()?;
        //return
        Ok(customer)
    }
}
