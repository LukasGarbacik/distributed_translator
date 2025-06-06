use std::env;
use std::path::PathBuf;
use std::process::Command;

pub fn thread_function(
    thread_id: usize,
    files: &[PathBuf],
    output_dir: &PathBuf,
    target_language: &String,
) {
    println!("DEBUG:Thread {} is running", thread_id);
    println!("DEBUG:files len: {}", files.len());

    for file in files {
        println!("Thread {} processing file: {}", thread_id, file.display());

        //switch case between local and docker envs
        let (python_cmd, script_path) = if env::var("IN_DOCKER").is_ok() {
            ("python3", "/usr/local/bin/translation.py") //docker
        } else {
            (
                "/Users/lukasgarbacik/Desktop/rust_project/venv/bin/python", //local
                "/Users/lukasgarbacik/Desktop/rust_project/src/translation.py",
            )
        };

        //Create python process and give script, file, output_dir, language -> returns Result
        //each process will hold its own core and will not block other threads
        let status = Command::new(python_cmd)
            .arg(script_path)
            .arg(file.to_string_lossy().to_string())
            .arg(
                output_dir
                    .join(file.file_name().unwrap_or_default())
                    .to_string_lossy()
                    .to_string(),
            )
            .arg(target_language)
            .status();

        match status {
            Ok(exit_status) => {
                //resultant status of the python process
                if exit_status.success() {
                    println!(
                        "Thread {} successfully processed: {}",
                        thread_id,
                        file.display()
                    );
                } else {
                    eprintln!("Thread {} failed to process: {}", thread_id, file.display());
                }
            }
            Err(e) => {
                eprintln!(
                    "Thread {} failed to execute Python script: {}",
                    thread_id, e
                );
            }
        }
    }
}
