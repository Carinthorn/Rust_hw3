
fn main(){
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 3{
        print!("Input: <start_f> <end_f> <delta>");
        return;
    }
    
    let mut f_start: f32 = match args[1].parse(){
        Ok(num) => num,
        Err(_) => {
            print!("Enter a numeric value for f_start");
            std::process::exit(1)
        }
    };

    let f_end: f32 = match args[2].parse() {
        Ok(num) => num,
        Err(_) => {
            print!("Enter a numeric value for f_end");
            std::process::exit(1)
        }
    };

    let delta: f32 = match args[3].parse::<f32>(){
        Ok(num) => num.abs(),
        Err(_) => {
            print!("Enter a numeric value for delta");
            std::process::exit(1)
        }
    };

    let mut end: bool = false;
    println!("{:>6}\t{:>6}","Fahr","Celcius");
    while !end {

        let celcius: f32 = (5.0/9.0)*(f_start - 32.0);
        println!("{:>6}\t{:>7}",  f_start, format!("{:.1}", celcius));
    
        if f_start < f_end{
            if f_start == f_end || f_start + delta > f_end{
                end = true;
            }
            f_start += delta;
            
        }else if f_start > f_end{
            if f_start == f_end || f_start - delta < f_end{
                end = true;
            }
            f_start -= delta;
        }else{
            end = true;
        }

        
    }

    




    
} 