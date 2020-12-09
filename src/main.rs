fn main() {
    println!("Hello, world!");
    let x = 5;
    let y = 7;
    let z = x + y;

    println!("z is {}",z);
    next_birthday("Jake", 33);
    next_birthday("Vibhuti", 56);
    println!("The Answer is {}",square(3));
    discount(10);
    let mut i = 0;
    loop {
        println!("Hell Yeah, I am the best!!");
        i += 1;
        if i == 10 {
            break;
        }
    }

    for item in 1..11 {
        println!("Now serving no {}",item)
    }

    let m = 4;
    match m {
        1 => println!("1 is cool"),
        2 => println!("2 is better"),
        3 => println!("3 is super awesome"),
        _ => println!("okay"),
    }
}

fn next_birthday(name: &str,current_age: u8){
    let next_age = current_age + 1;
    println!("Hi {}, on your next birthday, you will be {}!",name, next_age);
}
fn square(num: i64) -> i64 {
    num*num
}


fn discount(day_of_month: u8){
    let amount = if day_of_month % 2 == 0 {
        50
    } 
    else{
        30
    };
    println!("Your discount is {}!",amount);
}

