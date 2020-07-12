// qvs20_writer_mod

use crate::qvs20_table_schema_mod::*;

use rust_decimal::prelude::*;
//use unwrap::unwrap;

pub struct WriterForQvs20{
    // writes to utf8 String
    output: String,
    column: usize,
    row_delimiter: char,
}

impl WriterForQvs20 {
    // constructor
    pub fn new() -> Self {
        Self::new_with_delimiter('\n')
    }
    pub fn new_with_delimiter(row_delimiter:char) -> Self {
        //return
        WriterForQvs20 {
            output:s!(),
            column: 0,
            row_delimiter,
        }
    }
    pub fn output_is_empty(&self)-> bool{
        self.output.is_empty()
    }
    /// Move the String out of the writer struct, that will be soon dropped.
    /// Never use it more than once !
    pub fn move_output_string_out_of_struct(&mut self)-> String{
        // the Rust pattern to move out a struct field is to use mem:: replace
        // because of the strict ownership
        // it is then replaced with the new value. So never use it more than once!
        std::mem::replace(&mut self.output, s!())
    }
    /// write delimiter
    pub fn write_delimiter(&mut self) {
        self.output.push(self.row_delimiter);
        self.column = 0;
    }
    /// write a field of type String
    pub fn write_string(&mut self, data: &str) {
        self.output.push('[');
        self.push_escaped_qvs20_str(data);
        self.output.push(']');
        self.column += 1;
    }
    /// write a field of type integer
    pub fn write_integer(&mut self, data: i64) {
        self.output.push('[');
        self.output.push_str(&data.to_string());
        self.output.push(']');
        self.column += 1;
    }
    /// write a field of type decimal
    pub fn write_decimal(&mut self, data: Decimal) {
        self.output.push('[');
        self.output.push_str(&data.to_string());
        self.output.push(']');
        self.column += 1;
    }
    /// write a field of type float
    pub fn write_float(&mut self, data: f64) {
        self.output.push('[');
        let mut buffer = ryu::Buffer::new();
        self.output.push_str(&buffer.format(data));
        self.output.push(']');
        self.column += 1;
    }
    /// write a sub table schema
    /// write a field of type String
    pub fn write_sub_table_schema(&mut self, schema: &TableSchema) {
        self.output.push('[');
        let mut wrt = WriterForQvs20::new_with_delimiter(schema.row_delimiter as char);
        //sub table start with delimiter
        wrt.write_delimiter();
        schema.write_schema_to_writer(&mut wrt);
        let output_sub_schema = wrt.move_output_string_out_of_struct();

        self.output.push_str(&output_sub_schema);
        self.output.push(']');
        self.column += 1;
    }
    /// escape the 6 special characters \\, \[, \], \n, \r, \t
    /// all this characters are ascii7
    /// therefore I can use a faster vector of bytes and not a string
    /// less escaping needed, faster the performance
    /// the parameter escaped is allocated before this fn call
    pub fn push_escaped_qvs20_str(&mut self, text: &str) {
        let mut pos_start = 0;

        // iter() is by bytes, not characters !
        for (i, item) in text.as_bytes().iter().enumerate() {
            if item == &b'\\'
                || item == &b'['
                || item == &b']'
                || item == &b'\n'
                || item == &b'\t'
                || item == &b'\r'
            {
                self.output.push_str(&text[pos_start..i]);
                pos_start = i + 1;
                self.output.push('\\');
                // for \t \n \r must replace this byte with a different byte
                if item == &b'\n' {
                    self.output.push('n');
                } else if item == &b'\t' {
                    self.output.push('t');
                } else if item == &b'\r' {
                    self.output.push('r');
                } else if item == &b'[' {
                    self.output.push('[');
                } else if item == &b']' {
                    self.output.push(']');
                } else if item == &b'\\' {
                    self.output.push('\\');
                }
            }
        }
        self.output.push_str(&text[pos_start..]);
    }
}
#[cfg(test)]
mod test {
    use super::*;
    //use unwrap::unwrap;

    #[test]
    pub fn t01_write_data_string_escaped() {
        let schema = TableSchema::new_simple_strings(3);
        let mut wrt = WriterForQvs20::new();
        wrt.write_string("three");
        wrt.write_string("o\\n[e]");
        wrt.write_string("t\nw\to\r");
        wrt.write_delimiter();
        let output = wrt.move_output_string_out_of_struct();
        assert_eq!(output, "[three][o\\\\n\\[e\\]][t\\nw\\to\\r]\n");
    }
    #[test]
    pub fn t02_write_schema() {
        let schema = TableSchema::new_simple_strings(3);
        let mut wrt = WriterForQvs20::new();
        schema.write_schema_to_writer(& mut wrt);
        let output = wrt.move_output_string_out_of_struct();
        assert_eq!(
            output,
            "[t1][simple table-only strings]\n[String][String][String]\n[][][]\n[][][]\n[1][2][3]\n"
        );
    }
    #[test]
    pub fn t03_write_schema_and_data() {
        let schema = TableSchema::new_simple_strings(3);
        let mut wrt = WriterForQvs20::new();
        schema.write_schema_to_writer(& mut wrt);
        wrt.write_string("three");
        wrt.write_string("o\\n[e]");
        wrt.write_string("t\nw\to\r");
        wrt.write_delimiter();
        let output = wrt.move_output_string_out_of_struct();
        assert_eq!(output, "[t1][simple table-only strings]\n[String][String][String]\n[][][]\n[][][]\n[1][2][3]\n[three][o\\\\n\\[e\\]][t\\nw\\to\\r]\n");
    }
}
