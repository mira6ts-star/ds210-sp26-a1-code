// This imports the Debug and Display formatting traits from the standard library.
// Debug is used for developer-style printing, and Display is for user-style printing.
use std::fmt::{Debug, Display};

// This enum represents the type of data a column can store.
// In this project, a column can either hold Strings or Integers.
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum ColumnType {
    String,
    Integer,
}

// This imports the Debug and Display formatting traits from the standard library.
// Debug is used for developer-style printing, and Display is for user-style printing.
use std::fmt::{Debug, Display};

// This enum represents the type of data a column can store.
// In this project, a column can either hold Strings or Integers.
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum ColumnType {
    String,
    Integer,
}

// This enum represents one actual cell value inside the dataset.
// A value can either be a String or an Integer.
// Example:
// Value::String("Alice")
// Value::Integer(90)
#[derive(Clone, PartialEq, Hash, Eq, Debug, PartialOrd, Ord)]
pub enum Value {
    String(String),
    Integer(i32),
}
impl Value {
    // converts a Value into a regular String
    // If the Value is already a String, it returns that text.
    // If the Value is an Integer, it converts the number into text.
    // This is useful for printing values in the dataset.
    pub fn to_string(&self) -> String {
        match self {
            Value::String(value) => value.to_string(),
            Value::Integer(value) => value.to_string(),
        }
    }
}

// A Row represents one row in the CSV.
// Internally, it just stores a vector of Value objects.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Row {
    values: Vec<Value>,
}
impl Row {
     // Creates a new Row from a vector of values.
    // Example:
    // Row::new(vec![Value::String("Alice".to_string()), Value::Integer(90)])
    pub fn new(values: Vec<Value>) -> Row {
        return Row { values };
    }

    // Returns a reference to the full vector of values in the row.
    // Because it returns &Vec<Value>, it lets us look at the values
    // without taking ownership of them.
    pub fn get_values(&self) -> &Vec<Value> {
        return &self.values;
    }

    // Returns a reference to one value in the row at a given index.
    // Example:
    // row.get_value(0) gets the first value in the row.
    // This does not move the value out of the row, it just borrows it.
    pub fn get_value(&self, index: usize) -> &Value {
        return &self.values[index];
    }

    // takes ownership of the row and returns the inner vector of values.
    // Because self is consumed here, the row cannot be used afterward.
    // This is useful when you want to move all the data out instead of borrowing it.
    pub fn move_values(self) -> Vec<Value> {
        return self.values;
    }
}

// Dataset represents the full CSV dataset.
// columns: Stores each column name and its type.
// Example: [("name", String), ("grade", Integer)]
// rows: Stores all the actual rows of data.
pub struct Dataset {
    columns: Vec<(String, ColumnType)>,
    rows: Vec<Row>,
}
impl Dataset {
    pub fn new(columns: Vec<(String, ColumnType)>) -> Dataset {
        return Dataset {
            columns,
            rows: Vec::new(),
        };
    }
    pub fn add_row(&mut self, row: Row) {
        self.rows.push(row);
    }

    pub fn columns(&self) -> &Vec<(String, ColumnType)> {
        return &self.columns;
    }
    pub fn column_type(&self, column_name: &String) -> &ColumnType {
       let i = self.column_index(column_name);
        return &self.columns[i].1;
    }
    pub fn column_index(&self, column_name: &String) -> usize {
        for i in 0..self.columns.len() {
            let (cname, _ctype) = &self.columns[i];
            if cname == column_name {
                return i;
            }
        }
        panic!("Column {} not found", column_name);
    }

    pub fn iter(&self) -> std::slice::Iter<'_, Row> {
        return self.rows.iter();
    }

    pub fn into_iter(self) -> std::vec::IntoIter<Row> {
        return self.rows.into_iter();
    }

    pub fn len(&self) -> usize {
        return self.rows.len();
    }
}

impl Debug for Dataset {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Dataset:")?;
        write!(f, "|")?;
        for (colname, coltype) in &self.columns {
            let description = format!("{colname}: {coltype:?}");
            write!(f, " {description: <28}|")?;
        }
        writeln!(f, "")?;

        write!(f, "|")?;
        for _ in &self.columns {
            write!(f, "=============================|")?;
        }
        writeln!(f, "")?;

        for row in &self.rows {
            write!(f, "|")?;
            for value in row.get_values() {
                write!(f, " {: <28}|", value.to_string())?;
            }
            writeln!(f, "")?;
        }
        return Ok(());
    }
}
impl Display for Dataset {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return Debug::fmt(self, f);
    }
}

impl PartialEq for Dataset {
    fn eq(&self, other: &Self) -> bool {
        if self.columns != other.columns {
            return false;
        }

        let mut rows = self.rows.clone();
        rows.sort();
        let mut rows2 = other.rows.clone();
        rows2.sort();

        return rows == rows2;
    }
}
