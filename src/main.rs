

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


// 1D convolution. basic. single threaded
fn convolution_1d_v1(data_in:& Vec<f64>) -> Vec<f64>{
    let size_n = data_in.len();
    let max_index = size_n - 1;
    let step = size_n/100;
    let mut _forward_factor = vec![0.0; size_n];
    let mut _backward_factor = vec![0.0; size_n];

    // output data matrix. first and last element remains unchanged 
    let mut data_out = vec![0.0; size_n];
    data_out[0] = data_in[0];
    data_out[size_n-1] = data_in[size_n-1];

    let mut i=1;
    let mut if64: f64 = 0.0;
    let size_n_f64: f64 = size_n as f64;

    // setting up forward and backward factors (binomial distribution factors) ahead of use
    while i < max_index{
        if64=i as f64;
        _forward_factor[i]  = (size_n_f64 - if64 + 1.0) / if64; // i=0 will give undefined value
        _backward_factor[i] = (if64 + 1.0) / (size_n_f64 - if64); // i=N will give undefined value
        i += 1;
    }

    let mut j = 1; // to avoid error. j is greater than zero and less than max index
    let mut jf64 = 0.0;

    // computing output data matrix using input matrix and binomial distributions (forward and backward factors)
    while j < max_index{
        jf64 = j as f64;
        let prob     = jf64 / size_n_f64;
        let mut factor = 0.0;
        let mut binom = 0.0;
        let mut prev = 0.0;
        let mut binomNormalization_const = 1.0;
        let mut sum_sum = 0.0;
        

        // forward iteraion part. from `j` to `N`
        factor = prob / (1.0-prob);
        prev   = 1.0;
        let mut k=j;
        while k < max_index{
            binom     = prev * _forward_factor[k] * factor;
            binomNormalization_const += binom;
            sum_sum      += data_in[k] * binom;
            prev      = binom;
//            cout << binom << ", ";

            k += 1;
        }


        // backward iteration part. from `j-1` to `1`
        factor = (1.0-prob)/prob;
        prev   = 1.0;
        let mut k = j-1;
        while k > 0{
            binom     = prev * _backward_factor[k] * factor;
            binomNormalization_const += binom;
            sum_sum      += data_in[k] * binom;
            prev      = binom;
//            cout << binom << ", ";
            k-=1;
        }

        data_out[j] = sum_sum / binomNormalization_const;
        if j % step == 0 {
            println!("progress {}%", jf64*100.0/size_n_f64 );
            // print!("{}[2J", 27 as char);
            // print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
            // print!("\x1B[2J\x1B[1;1H");
        }
        j += 1;
    }
    data_out
}


fn compute_for_j_value(j:usize, _forward_factor:& Vec<f64>, _backward_factor:& Vec<f64>, data_in:& Vec<f64>) -> f64{
    let size_n_f64 = data_in.len() as f64;
    let max_index = data_in.len() - 1;
    let jf64 = j as f64;
    let prob     = jf64 / size_n_f64;
    let mut factor = 0.0;
    let mut binom = 0.0;
    let mut prev = 0.0;
    let mut binomNormalization_const = 1.0;
    let mut sum_sum = 0.0;
    

    // forward iteraion part. from `j` to `N`
    factor = prob / (1.0-prob);
    prev   = 1.0;
    let mut k=j;
    while k < max_index{
        binom     = prev * _forward_factor[k] * factor;
        binomNormalization_const += binom;
        sum_sum      += data_in[k] * binom;
        prev      = binom;
//            cout << binom << ", ";

        k += 1;
    }


    // backward iteration part. from `j-1` to `1`
    factor = (1.0-prob)/prob;
    prev   = 1.0;
    let mut k = j-1;
    while k > 0{
        binom     = prev * _backward_factor[k] * factor;
        binomNormalization_const += binom;
        sum_sum      += data_in[k] * binom;
        prev      = binom;
//            cout << binom << ", ";
        k-=1;
    }
    // return value
    sum_sum / binomNormalization_const
}

// compute convolution for j= j1 to j2
// includes j1 and excludes j2
fn convolution_j1_to_j2(j1:usize, j2:usize, step:usize,
    _forward_factor:& Vec<f64>, _backward_factor:& Vec<f64>, data_in:& Vec<f64>,
    data_out:&mut Vec<f64>
){
        let mut j = j1;
        let size_n = data_in.len();
        while j < j2{
            data_out[j] = compute_for_j_value(j, &_forward_factor, &_backward_factor, &data_in);
            
            if j % step == 0 {
                println!("progress {}%", j/size_n );
                // print!("{}[2J", 27 as char);
                // print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
                // print!("\x1B[2J\x1B[1;1H");
            }
            j += 1;
        }
}

// Get a tuple of (j1,j2)
// Say data length is 100 and thread count is 2 then
// => j1,j2=(0,50), (50, 100) will be  used
// thread count 3 => j1,j2=(0,40),(40,70),(70,100)
// how do we divide? simple put extra data in the first thread. Few extra data in one of the threads is not a problem
fn get_j1_j2(size_n: usize, threads: usize) -> Vec<(usize, usize)>{
    let per_thread = size_n/threads;
    let mut i = 0;
    let mut thelist: Vec<(usize, usize)> = vec![(0, 0)];
    let mut prec_j = 0;
    let mut current_j = per_thread;
    while i < (threads-1){
        thelist.push((prec_j, current_j));
        prec_j = current_j;
        current_j += per_thread;

        i += 1;
    }
    thelist[0]=(i*per_thread, size_n); // replace initial (0,0). I will fix it later although it's not really a problem

    println!("get_j1_j2 [");
    for (j1, j2) in &thelist{
        print!("({}, {})", j1, j2)
    }
    println!("]");


    thelist 

}

// 1D convolution. basic. multi threaded
fn convolution_1d_v2(data_in:& Vec<f64>, threads:usize) -> Vec<f64>{
    let size_n = data_in.len();
    let max_index = size_n - 1;
    let step = size_n/100;
    let mut _forward_factor = vec![0.0; size_n];
    let mut _backward_factor = vec![0.0; size_n];

    // output data matrix. first and last element remains unchanged 
    let mut data_out = vec![0.0; size_n];
    data_out[0] = data_in[0];
    data_out[size_n-1] = data_in[size_n-1];

    let mut i=1;
    let mut if64: f64 = 0.0;
    let size_n_f64: f64 = size_n as f64;

    // setting up forward and backward factors (binomial distribution factors) ahead of use
    while i < max_index{
        if64=i as f64;
        _forward_factor[i]  = (size_n_f64 - if64 + 1.0) / if64; // i=0 will give undefined value
        _backward_factor[i] = (if64 + 1.0) / (size_n_f64 - if64); // i=N will give undefined value
        i += 1;
    }

    let mut j = 1; // to avoid error. j is greater than zero and less than max index
    let mut jf64 = 0.0;

    // computing output data matrix using input matrix and binomial distributions (forward and backward factors)
    // multi-threading. just consider that each j is independent of others
    
    let j1j2tuple=get_j1_j2(size_n, threads);

    for (j1, j2) in j1j2tuple{
        convolution_j1_to_j2(j1, j2, step, &_forward_factor, &_backward_factor, &data_in, &mut data_out);
        
        if j % step == 0 {
            println!("progress {}%", jf64*100.0/size_n_f64 );
            // print!("{}[2J", 27 as char);
            // print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
            // print!("\x1B[2J\x1B[1;1H");
        }
        j += 1;
    }
    data_out
}


// 1D convolution. fast


// 2D convolution. basic


// 2D convolution. fast
    


// it's like unit test
fn test1_convolution(){
    
    // let mut data_in = vec![1.0; 1000];
    // let data_out = convolution_1d_v1(&data_in);

    get_j1_j2(100, 2);
    get_j1_j2(100, 3);
    get_j1_j2(100, 7);
    // let data_out2 = convolution_1d_v2(&data_in, 4);
    // let mut i = 0;
    // while i < data_out.len(){
    //     if (data_out[i] - data_out2[i]).abs() > 1e-5{
    //         println!("Data mismatch for i={}", i)
    //     }
    // }
    // println!("Output data [");
    // for a in data_out{
    //     print!("{}, ", a)
    // }
    // println!("]")
}


fn main() {
    println!("Convolution of big data using RUST");

    // help();
    // cmd_args();
//    test1_convolution();
    let start = Instant::now();

    test1_convolution();

    let duration = start.elapsed();

    // let now = SystemTime::now();
    // println!("Program finished at {}", now);
    println!("Time elapsed : {:?} ", duration);
    
}
