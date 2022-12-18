
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

    let mut counter_i = 1;
    let increment = 1;
    loop{
        print!("current key {} ", &args[counter_i]);

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
            "--in" => {
                args_dict.insert("in_filename", args[counter_i+1].as_str());
            },
            "--out" => {
                args_dict.insert("out_filename", args[counter_i+1].as_str());
            },
            "--threshold" => {
                args_dict.insert("threshold", args[counter_i+1].as_str());
            },
            "--times" => {
                args_dict.insert("n_times", args[counter_i+1].as_str());
            },
            "-v" => {
                args_dict.insert("version", args[counter_i+1].as_str());
            },
            "--version" => {
                args_dict.insert("version", args[counter_i+1].as_str());
            },
            "-p" => {
                args_dict.insert("precision", args[counter_i+1].as_str());
            },
            "--precision" => {
                args_dict.insert("precision", args[counter_i+1].as_str());
            },
            "-a" => {
                args_dict.insert("without_cols", args[counter_i+1].as_str());
            },
            "--without" => {
                args_dict.insert("without_cols", args[counter_i+1].as_str());
            },
            "-b" => {
                args_dict.insert("with_cols", args[counter_i+1].as_str());
            },
            "--with" => {
                args_dict.insert("with_cols", args[counter_i+1].as_str());
            },
            "-d" => {
                args_dict.insert("delimiter", args[counter_i+1].as_str());
            },
            "--delimiter" => {
                args_dict.insert("delimiter", args[counter_i+1].as_str());
            },
            "-w" => {
                args_dict.insert("write_b_cols", "true");
            },
            "--write" => {
                args_dict.insert("write_b_cols", "true");
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
        println!("{} : {}", key, val)
    }


}


// 1D convolution. basic
fn convolution_1d_v1(data_in:Vec<f64>, thread_count:u8){
    let size_N = data_in.len();
    let mut _forward_factor = vec![0.0; size_N];
    let mut _backward_factor = vec![0.0; size_N];

    let mut i=1;
    let mut if64: f64 = 0.0;
    let sizeNf64: f64 = size_N as f64;
    while i < size_N{
        if64=i as f64;
        _forward_factor[i]  = (sizeNf64 - if64 + 1.0) / if64; // i=0 will give undefined value
        _backward_factor[i] = (if64 + 1.0) / (sizeNf64 - if64); // i=N will give undefined value
        i += 1;
    }

}


// 1D convolution. fast


// 2D convolution. basic


// 2D convolution. fast
    

fn test1_convolution(){
    let data_in = vec![0.0; 100];
    convolution_1d_v1(data_in, 2);
}


fn main() {
    println!("Convolution of big data using RUST");

    // help();
    cmd_args();
//    test1_convolution();
    let start = Instant::now();
    test1_convolution();
    let duration = start.elapsed();

    // let now = SystemTime::now();
    // println!("Program finished at {}", now);
    println!("Time elapsed : {:?} ", duration);
    
}
