use std::fmt::DebugStruct;

fn main() {
    let mut x = 10;
    x = 6;
    println!("The value of x is {}!", x);

    let y: i8 = -1;
    println!("The value of y is {}!", y);

    let z: u8 = 30;
    println!("The value of z is {}!", z);
    
    let tup = (100, 'A', true);
    println!("The value of tup is {}!", tup.1);

    let (a,b,c) = tup;

    let arr = [100, 2, 3];

    println!("The value of item 0 of arr is {}!", arr[0]);

    let mut nums = vec![1,2,3,4];
    nums.push(6);
    println!("The value of nums is {:?}!", nums);
    nums.pop();
    println!("The value of nums is {:?}!", nums);

    let v: Vec<i32> = (0..7).collect();
    println!("The value of v is {:?}!", v);

    for element in &v {
        println!("The value of element in v is {}!", element);
    }

    //slices
    let sv : &[i32] = &v[2..4];
    println!("The items in slice sv are {:?}!", sv);

    let name = String::from("Paulo");
    let course = "Rust".to_string();

    println!("{}", name);
    println!("{}", course);

    print_my_sentence("new phrase created by me!");

    println!("{}",gcd(20, 10));


}

fn print_my_sentence(sentence: &str){
    println!("{}", sentence);
}

fn gcd(mut a: u64, mut b: u64) -> u64 {
    while(a!=0){
        if (a<b){
            let c = a;
            a = b;
            b = c;
        }
        a = a % b;
    }
    b
}