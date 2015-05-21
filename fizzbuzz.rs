fn div_by_three(num: &i32) -> bool {
    if num % 3 == 0 {true}
    else {false}
}

fn div_by_five(num: &i32) -> bool {
    if num % 5 == 0 {true}
    else {false}
}

fn div_by_fifteen(num: &i32) -> bool {
    if num % 15 == 0 {true}
    else {false}
}

fn main(){
    for num in 1..100{
      let word = 
        if div_by_three(&num) {"Fizz".to_string()}
        else if div_by_five(&num) {"Buzz".to_string()}
        else if div_by_fifteen(&num) {"Fizz Buzz".to_string()}
        else {num.to_string()};
      
      println!("{}", word);
    }
}
