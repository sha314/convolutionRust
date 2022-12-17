
use std::time::{Instant};
use std::env;

use std::collections::HashMap;


// list all command line options
fn help(){
    let help_str = String::from("Usage:
    convolution [--in <STRING>] [-a <INT>,<INT>,...] [-b <INT>,<INT>,...] [-h] [-t <INT>] [-i <STRING>]
        
        perform convolution based on provided options.
        
        Options                      Description
          -a, --without              columns that we want in the output file without performing convolution.
                                     No default value.
        
          -b, --with                 columns that we want in the output file with performing convolution.
                                     No default value.
        
          -d, --delimiter            Delimiter to use. Default value is ' '.
        
              --in                   name of the input file that we want to convolute. No default value.
        
          -i  --info                 Info to write as comment in the output file
        
              --out                  name of the output file. If not provided the string '_convoluted.txt' will be
                                     appended to the input file.
        
          -p, --precision            Floating point precision when writing in the data file. Default value is 10
        
          -s, --skip                 Number of rows to skip from the input file. Default value is 0.
        
          -t, --threads              Explicitly specify number of thread to use. Default is the max number of thread
                                     allowed by the system.
        
              --threshold            If weight factor that multiplies input data at each iteration is less than
                                    `threshold` then break that loop. Program performs way faster in this way.
                                     Negative value of the threshold will perform full convolution without skipping
                                     any step which increases time required to do this exponentially. Default value if (1e-15)
        
              --times                Number of times to perform convolution.
        
          -h, --help                 display this help and exit
        
          -v, --version              output version information and exit
        
          -w, --write                If provided input b data will be written to the output file.
        
        
        The INT argument is an integer.
        The STRING argument is a string of characters.
        
        A line that begins with '#' is considered a commented line.
        
        Exit status:
         0  if OK,
         1  if minor problems (e.g., cannot access subdirectory),
         2  if serious trouble (e.g., cannot access command-line argument).
         ");
    println!("{}", help_str);
}

// TODO
// use dictionary or hashmap to store key values to be used later
fn cmd_args(){
    println!("cmmand line args");
    let args: Vec<String> = env::args().collect();
    // Type inference lets us omit an explicit type signature (which
    // would be `HashMap<String, String>` in this example).
    let mut args_dict = HashMap::new();


    println!("{}", args.len());
    if args.len() == 1{
        help();
        return;
    }
    // for arg in args{
    //     print!("{}", &arg)
    // }

    let mut counter_i = 0;
    let increment = 1;
    loop{
        print!("{} ", &args[counter_i]);

        match  args[counter_i].as_str(){
            "-h" => {
                help();
            },
            "--help" => {
                help();
            },
            "-t" => {
                args_dict.insert("threads", args[counter_i+1].as_str());
            },
            "--threads" => {
                args_dict.insert("threads", args[counter_i+1].as_str());
            },
            _ => {println!("It's nothing");}
        }
        counter_i += increment;

        if counter_i >= args.len(){
            break;
        }
    }

    println!("Got the following key:value ");
    for (key,val) in args_dict{
        println!("{}:{}", key, val)
    }


}


fn convolution_execute_v1(){

}
    

    


fn main() {
    println!("Convolution of big data using RUST");

    // help();
    cmd_args();
//    test1_convolution();
    let start = Instant::now();
    convolution_execute_v1();
    let duration = start.elapsed();

    // let now = SystemTime::now();
    // println!("Program finished at {}", now);
    println!("Time elapsed : {:?} ", duration);
    
}
