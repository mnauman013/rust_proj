// Defining Struct with a single field for desire type of filtering
struct FilterCondition <T> {
    condition: T,
}

// want to filter strings use this struct and impl
/* 
struct FilterCondition {
    value: String,
}

impl FilterCondition {
fn is_match(&self, item: &String) -> bool {
        *item == self.value
    }
}
*/


// Implement the is_match method on struct

impl FilterCondition<i32> {
    fn is_match(&self, item: &i32) -> bool {
        item == &self.condition
    }    
}

// Defining the custom_filter function with a collection and the condition

fn custom_filter(collection: &Vec<i32>, filter: &FilterCondition<i32>) -> Vec<i32> {
    let mut filtered_collection = Vec::new();
    for item in collection {
        if filter.is_match(item){
            filtered_collection.push(*item);
        }
    }
    filtered_collection
}

// main function 
// creating a collection and initialize FilterCondition object with desire value

fn main(){
    let collection = vec![10, 20, 30, 40, 50, 60, 70, 80];
    let filter_condition = FilterCondition {condition: 25};

    let filtered_collection = custom_filter(&collection, &filter_condition);

    println!("Filtered result: {:?}", filtered_collection);
}