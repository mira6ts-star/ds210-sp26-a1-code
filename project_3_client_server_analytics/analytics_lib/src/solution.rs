use std::collections::HashMap;
use std::result;
use crate::dataset::{ColumnType, Dataset, Value, Row};
use crate::query::{Aggregation, Condition, Query};

// student 1 implemented this function
// takes a row, the dataset (which we need to look up column indexes), and a condition. returns true or false
// does a match on the 4 possible conditions
fn row_matches(row: &Row, dataset: &Dataset, condition: &Condition) -> bool {
    match condition {
        // finds which column we're checking
        Condition::Equal(col_name, expected_value) => {
            // grabs the value in that column for this row
            let idx = dataset.column_index(col_name);

            // sees if the column equals what we expect
            row.get_value(idx) == expected_value
        }

        // runs the inner conditions and flips the result. recursion so calls function on itself
        Condition::Not(inner) => !row_matches(row, dataset, inner),
        // both sides must be true
        Condition::And(left, right) => {
            row_matches(row, dataset, left) && row_matches(row, dataset, right)
        }
        // at least one side must be true
        Condition::Or(left, right) => {
            row_matches(row, dataset, left) || row_matches(row, dataset, right)
        }
    }
}

// student 1 implementation kept for this function
pub fn filter_dataset(dataset: &Dataset, filter: &Condition) -> Dataset {
    // creates a new dataset with the same columns as the input
    let mut result = Dataset::new(dataset.columns().clone());
    // loops through every row 
    for row in dataset.iter() {
        // for each row calls row_matches if its true adds that row to the result
        if row_matches(row, dataset, filter) {
            result.add_row(row.clone());
        }
    }

    // returns result
    result
}
 // student 1 implementation
pub fn group_by_dataset(dataset: Dataset, group_by_column: &String) -> HashMap<Value, Dataset> {
    // figures out which column index we're grouping by
    let col_idx = dataset.column_index(group_by_column);
    let columns = dataset.columns().clone();
    // creates an empty HashMap
    let mut groups: HashMap<Value, Dataset> = HashMap::new();

    // loops through every row
    for row in dataset.into_iter() {
        // for each row looks at its value in the group-by column that becomes the key
        let key = row.get_value(col_idx).clone();
        // if that key doesn't have a bucket yet create one. then add the row to that bucket
        groups
            .entry(key)
            .or_insert_with(|| Dataset::new(columns.clone()))
            .add_row(row);
    }
    // return the Hash Map of buckets
    groups
}

// student 2 implementation
// loops through each bucket in the HashMap
// goes through each grouped dataset and computes one aggregation result per group
// stores that number in a result HashMap and returns it
pub fn aggregate_dataset(dataset: HashMap<Value, Dataset>, aggregation: &Aggregation) -> HashMap<Value, Value> {
    let mut result = HashMap::new();

    for (group_value, group_dataset) in dataset {
        match aggregation {
            Aggregation::Count(_column_name) => {
                // counts how many rows are in the bucket
                let count = group_dataset.iter().count() as i32;
                result.insert(group_value, Value::Integer(count));
            }

            Aggregation::Sum(column_name) => {
                let col_index = group_dataset.column_index(column_name);
                let mut sum = 0;

                // loops through every row grab the value in the target column and add it up
                for row in group_dataset.iter() {
                    match row.get_value(col_index) {
                        Value::Integer(val) => sum += *val,
                        Value::String(_) => {
                            panic!("Cannot sum string values in column '{}'", column_name); 
                        }
                    }
                }
                result.insert(group_value, Value::Integer(sum));
            }

            // same as sum but we also count the rows and divide at the end
            Aggregation::Average(column_name) => {
                // reuses the same logic as sum but also keeps track of the count to compute the average at the end
                let col_index = group_dataset.column_index(column_name);
                let mut sum = 0;
                let mut count = 0;

                for row in group_dataset.iter() {
                    match row.get_value(col_index) {
                        Value::Integer(val) => {
                            sum += *val;
                            count += 1;
                        }
                        Value::String(_) => {
                            panic!("Cannot average string values in column '{}'", column_name); 
                        }
                    }
                }

                let average = sum / count;
                result.insert(group_value, Value::Integer(average));
            }
        }
    }

    result 
}

pub fn compute_query_on_dataset(dataset: &Dataset, query: &Query) -> Dataset {
    let filtered = filter_dataset(dataset, query.get_filter());
    let grouped = group_by_dataset(filtered, query.get_group_by());
    let aggregated = aggregate_dataset(grouped, query.get_aggregate());

    // Create the name of the columns.
    let group_by_column_name = query.get_group_by();
    let group_by_column_type = dataset.column_type(group_by_column_name);
    let columns = vec![
        (group_by_column_name.clone(), group_by_column_type.clone()),
        (query.get_aggregate().get_result_column_name(), ColumnType::Integer),
    ];

    // Create result dataset object and fill it with the results.
    let mut result = Dataset::new(columns);
    for (grouped_value, aggregation_value) in aggregated {
        result.add_row(Row::new(vec![grouped_value, aggregation_value]));
    }
    return result;
}
