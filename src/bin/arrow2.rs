fn main() {

    let args: Vec<String> = std::env::args().collect();
    let dot_num: &str = if args.len() < 2{ "" } else { &args[1] };
    let dot: i32 = match dot_num.parse::<i32>(){
        Ok(num) => num.abs(), 
        Err(_) => {
            print!("Enter a numeric value");
            std::process::exit(1);
        }
    };


    for i in 0..(dot*2){ 
        if i > dot - 1{
            
            for j in 0..(i - dot + 1){ 
                print!(" ");
            }
            for n in (0..(dot*2 - i - 1)).rev(){  
                print!("*");
            }
            println!("{}", ("").trim_end());
        }else{
            for j in 0..(dot - 1 - i){ 
                print!(" ");
            }
            for n in 0..(i+1){
                print!("*");
            }
            println!("{}", ("").trim_end());
        }
    }
    
}

