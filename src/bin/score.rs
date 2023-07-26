//str/neg num/>100/more than 1 num/enter nothing
//for negative score run cargo run --bin score -- -5
//for normal score run cargo run --bin score 5
fn main(){
    let args: Vec<String> = std::env::args().collect();
    let ans_arg = if args.len() < 2 { "" } else { &args[1] };
    let ans: i32 =  match ans_arg.parse::<i32>(){
        Ok(value) => value ,
        Err(_) => {
            print!("Enter a numeric score value");
            std::process::exit(1)
        }
    };

    if ans > 100 || ans < 0{
        print!("Invalid score")
    }else if ans >= 95 {
        print!("Excellent with A+")
    }else if ans >= 81 {
        print!("A")
    }else if ans >= 71 {
        print!("B")
    }else if ans >= 61 {
        print!("C")
    }else if ans >= 50 {
        print!("D")
    }else if ans >= 0 {
        print!("Failed with F")
    }

}