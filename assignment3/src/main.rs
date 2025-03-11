fn main() {
    let mut v = vec![1,3,5,7];
    let res: bool = check(&v); //gives ownership to function
    println!("{:?}",res);
    v.push(15); //need to be mutable to allow push
    println!("{:?}",v);

    let i : i8 = 3;
    add_two(i); //copy as it is int
    println!("{:?}",i); //can use below
}

//Create a function that takes one argument called val that is of type Vec with the values: 1,3,5,7. 
//Inside of this function check the first value of the vector and see if it is equal to one. 
//If the value is equal to one, then return true, otherwise return false. 
//Next add the value 15 to the vector. Print out the vector to confirm your results.
fn check(val: &Vec<i32>) -> bool {
    if val[0]==1 {
        return true
    }
    else {
        return false
    }
}

//Create a function called "add_two". 
//This function is going to take one parameter that is i8 and add two to it. 
//For the function, do you have to pass the value by reference in order for you to maintain ownership 
//of it inside of main?
fn add_two(val: i8) {
    let v = val+2;
    println!("{:?}",v); //can use inside by copied value
}