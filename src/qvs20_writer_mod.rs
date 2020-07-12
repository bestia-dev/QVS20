// qvs20_writer_mod

// writes to utf8 String

use crate::qvs20_table_rows_mod::*;
use crate::qvs20_table_schema_mod::*;

use rust_decimal::prelude::*;
//use unwrap::unwrap;

pub struct WriterForQvs20<'a> {
    output: &'a mut String,
    schema: &'a TableSchema,
    column: usize,
    row_delimiter: char,
}

impl<'a> WriterForQvs20<'a> {
    // constructor
    pub fn new(output: &'a mut String, schema: &'a TableSchema) -> Self {
        let row_delimiter = schema.row_delimiter as char;
        //return
        WriterForQvs20 {
            output,
            schema,
            column: 0,
            row_delimiter,
        }
    }
    pub fn write_schema(&mut self) {
        self.write_string(&self.schema.table_name);
        self.write_string(&self.schema.table_description);
        self.write_delimiter();
        for x in self.schema.data_types.iter() {
            self.write_string(&x.to_string());
        }
        self.write_delimiter();
        for x in self.schema.sub_table_schemas.iter() {
            match x {
                None => self.write_string(""),
                Some(schema) => self.write_sub_table_schema(&schema),
            }
        }
        self.write_delimiter();
        for x in self.schema.additional_properties.iter() {
            self.write_string(&x);
        }
        self.write_delimiter();
        for x in self.schema.column_names.iter() {
            self.write_string(&x);
        }
        self.write_delimiter();
    }
    pub fn write_table_rows(&mut self, table_rows: &'a TableRows) {
        if self.output.is_empty() {
            // when TableRows are in separate file from Schema
            // the 1st row has one field: TableName
            self.write_string(&self.schema.table_name);
            self.write_delimiter();
        }
        for row in table_rows.rows.iter() {
            for value in row.values.iter() {
                match value {
                    Value::String(s) => self.write_string(&s),
                    Value::Integer(i) => self.write_integer(*i),
                    Value::Decimal(d) => self.write_decimal(*d),
                    Value::Float(f) => self.write_float(*f),
                    /*
                    Bool(bool),
                    DateTimeFixedOffset(DateTime<FixedOffset>),
                    Date(NaiveDate),
                    Time(NaiveTime),
                    SubTable(TableRows),
                    */
                    _ => {}
                }
            }
            self.write_delimiter();
        }
    }
    pub fn write_table(&mut self, table_rows: &'a TableRows) {
        self.write_schema();
        self.write_table_rows(table_rows);
    }
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
        let mut output_sub_schema = String::with_capacity(100);
        {
            let mut wrt = WriterForQvs20::new(&mut output_sub_schema, &schema);
            //sub table start with delimiter
            wrt.write_delimiter();
            wrt.write_schema();
        }
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
        let mut output = String::with_capacity(1000);
        let mut wrt = WriterForQvs20::new(&mut output, &schema);
        wrt.write_string("three");
        wrt.write_string("o\\n[e]");
        wrt.write_string("t\nw\to\r");
        wrt.write_delimiter();
        assert_eq!(output, "[three][o\\\\n\\[e\\]][t\\nw\\to\\r]\n");
    }
    #[test]
    pub fn t02_write_schema() {
        let schema = TableSchema::new_simple_strings(3);
        let mut output = String::with_capacity(1000);
        let mut wrt = WriterForQvs20::new(&mut output, &schema);
        wrt.write_schema();
        assert_eq!(
            output,
            "[t1][simple table-only strings]\n[String][String][String]\n[][][]\n[][][]\n[1][2][3]\n"
        );
    }
    #[test]
    pub fn t03_write_schema_and_data() {
        let schema = TableSchema::new_simple_strings(3);
        let mut output = String::with_capacity(1000);
        let mut wrt = WriterForQvs20::new(&mut output, &schema);
        wrt.write_schema();
        wrt.write_string("three");
        wrt.write_string("o\\n[e]");
        wrt.write_string("t\nw\to\r");
        wrt.write_delimiter();
        assert_eq!(output, "[t1][simple table-only strings]\n[String][String][String]\n[][][]\n[][][]\n[1][2][3]\n[three][o\\\\n\\[e\\]][t\\nw\\to\\r]\n");
    }
}
