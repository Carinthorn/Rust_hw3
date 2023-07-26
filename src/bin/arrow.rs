fn main(){
    let args: Vec<String> = std::env::args().collect();
    let line_args = if args.len() < 2{ "" } else { &args[1] };
    let line: i32 = match line_args.parse::<i32>(){
        Ok(num) => num.abs(), 
        Err(_) => {
            print!("Enter a numeric value");
            std::process::exit(1);
        }
    };
  
    for i in 1..(line*2+1){
        if i > line{
            for n in (1..=(line*2-(i))).rev(){
                print!("*");
            }
            if i == line*2{
                std::process::exit(1);
            }
            println!("{}", ("").trim_end());
        }else{
            for j in 1..(i+1){
                print!("*");
            }
            println!("{}", ("").trim_end());
        }
    }
}