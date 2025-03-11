fn main() {
    
    //Create three variables with the names: val1, val2, and ans. 
    //We want to perform a simple operation of generating the modulo of val1 and val2. 
    //Set val1 to 5 and val2 to 2. Assign the answer to the ans variable. 
    //Before executing your code, what do you think the answer will be? ans = 1
    let val1: i64 = 5;
    let val2: i64 = 2;
    let ans: i64 = val1 % val2;
    println!("{}",ans);


    //Create a vector and put in the values "2, 4, 6, 8, 10". 
    //Once you have created the vector perform the following: print out the current values, 
    //remove the value 10, add the value 12, 
    //and then print the vector back out to confirm your results.
    let mut arr = vec![2, 4, 6, 8, 10];
    for element in &arr {
        println!("{}!", element);
    }
    arr.remove(4);
    arr.push(12);
    println!("{:?}", arr);

    let str1 = "Hello";
    println!("{}", concat_string(str1));

    control_flow(1);
    control_flow(24);
    control_flow(47);
    control_flow(55);
//Create a function called control_flow. This is going to take one argument that is an integer. Based on this integer, print out the following: "The value is one", "The value is greater than 50", "The value is less than 25", or "The value is greater than 25 but less than 50".
}

//Create a function called "concat_string". Create a string variable and assign the value "Hello" to it. The function is going to take one argument that is of type string and is going to return a String. Inside this function, concatenate the string " World". Print out the results in main() to confirm your results.
fn concat_string(str_prefix: &str) -> String {
    return format!("{} {}", str_prefix, " World");
}

fn control_flow(value: u64) {
    if value == 1 {
        println!("{}", "The value is one");
    }
    else if value < 25 {
        println!("{}", "The value is less than 25");
    }
    else if value > 25 && value < 50 {
        println!("{}", "The value is greater than 25 but less than 50");
    }
    else if value > 50 {
        println!("{}", "The value is greater than 50");
    }
}