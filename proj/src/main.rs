fn concatenate_strings(s1: &str, s2: &str) -> String{
    let mut result = String::new();
    result.push_str(&s1);
    result.push_str(&s2);
    result
}

fn main(){
    let string1 = String::from("Hello, ");
    let string2 = String::from("RiseIn!");

    let concatenate_string = concatenate_strings(&string1, &string2);
    println!("{}", concatenate_string);
}