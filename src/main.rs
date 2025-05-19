use std::thread;
use std::path::{PathBuf};
use std::sync::{Arc};
use std::fs;
use num_cpus;
use clap::Parser;

mod thread_handler;
use thread_handler::thread_function;


fn unpack_input(input_dir: &PathBuf) -> Vec<PathBuf> {
    let mut unpacked_files = Vec::new();
    let mut non_txt = 0;
    match fs::read_dir(input_dir) {
        Ok(files) => {
            for file_result in files {
                if let Ok(file) = file_result {    
                    if file.path().extension().map_or(false, |ext| ext == "txt") {
                        unpacked_files.push(file.path()); //only push if .txt
                    } else {
                        non_txt += 1; //count non txt files
                    }
                }
            }
        }

        Err(e) => {
            eprintln!("Error reading directory: {}", e);
            std::process::exit(1);
        }
        
    }
    println!("Number of non txt files: {}", non_txt);
    unpacked_files
}

fn distribute_files(files: &[PathBuf], num_cores : usize) -> Vec<Vec<PathBuf>> { //length min(files.len(), num_cores) on 2d return vec
    let mut ret_vec: Vec<Vec<PathBuf>> = Vec::new();
    if files.len() < num_cores {
        //create files.len() vecs
        //each vec has 1 file
        for file in files {
            ret_vec.push(vec![file.clone()]);
        }
    }
    else {
        for _ in 0..num_cores {
            ret_vec.push(Vec::new());
        }
        let base_files_per_core = files.len() / num_cores;
        let remainder = files.len() % num_cores;
        let mut index = 0;
        for i in 0..num_cores {
            for _ in 0..base_files_per_core {
                ret_vec[i].push(files[index].clone());
                index += 1;
            }
        }
        for i in 0..remainder {
            ret_vec[i].push(files[index].clone());
            index += 1;
        }
    }
    ret_vec
}

#[derive(Parser, Debug)]
struct Args {

    input_dir: PathBuf,
    output_dir: PathBuf,
    translated_language: String,
}


fn main() {
    // user input interaction (blocking)
    // accepts a a required input and output directory
    let args = Args::parse();

    if !args.input_dir.exists() {
        eprintln!("Input directory does not exist");

        std::process::exit(1); //exit with error when no input dir given

    }
    if !args.output_dir.exists() {// creates output and moves on
        std::fs::create_dir(&args.output_dir).expect("Failed to create output directory");
    }

    let unpacked_arc_files = unpack_input(&args.input_dir);
    
    
    // Get the number of CPU cores
    let num_cores = num_cpus::get();
    println!("Number of CPU cores: {}", num_cores);
    
    //block distribution only
    //less than cores -> last threads dont get started and get 0 files
    //more than cores -> "number of files" follows rr rules, not the files themselves

    let distributed_files = Arc::new(distribute_files(&unpacked_arc_files, num_cores)); 

    // Spawn one thread per core
    let threads: Vec<_> = (0..distributed_files.len())
        .map(|i| {
            //gives atomic access to the threads as a ref to the unpacked dir
            //only give slice to each thread with given range based on cores
            let files_ref = Arc::clone(&distributed_files);

            //Copies to avoid borrowing problems
            let output_dir = args.output_dir.clone();
            let language = args.translated_language.clone();

            thread::spawn(move || {
                thread_function(i, &files_ref[i], &output_dir, &language);
            })
        })
        .collect();
    
    // Join threads at the end block until all finished
    // 
    // Implement fault tolerance later with heartbeats or something
    for thread in threads {
        thread.join().unwrap();
    }

    println!("EOP");
}
