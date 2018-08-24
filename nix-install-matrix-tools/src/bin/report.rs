#[macro_use]
extern crate structopt;

use std::io::BufRead;
use std::io;
use std::io::BufReader;
use std::path::PathBuf;
use structopt::StructOpt;
use std::fs;
use std::path::Path;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

#[derive(StructOpt, Debug)]
#[structopt(name = "basic")]
struct Opt {
    /// Test results directory
    #[structopt(short = "i", long = "input", name = "FILE", parse(from_os_str))]
    input: PathBuf,

    /// Output file
    #[structopt(short = "o", long = "output", parse(from_os_str))]
    output: PathBuf,
}

#[derive(Debug)]
struct TestEnvironments {
    environments: Vec<TestEnvironment>
}

#[derive(Debug)]
struct Details {
    details: HashMap<String, Vec<String>>
}

#[derive(Debug)]
struct TestEnvironment {
    name: String,
    details: Details,
    runs: HashMap<String, TestRun>
}

#[derive(Debug)]
struct TestRun {
    details: Details,
    tests: HashMap<String, TestResult>
}

#[derive(Debug)]
struct TestResult {
    duration: u16,
    exitcode: u8,
    log: Vec<String>
}

impl Details {
    fn new(dir: &Path) -> Result<Details, io::Error> {
        Ok(Details {
            details: fs::read_dir(dir)?
                .collect::<Result<Vec<_>, io::Error>>()?
                .into_iter()
                .filter(|entry| entry.path().is_file())
                .map::<Result<(String, Vec<String>), io::Error>, _>(|entry| {
                    let f = BufReader::new(File::open(entry.path())?);

                    Ok((entry.file_name().as_os_str().to_string_lossy().to_string(),
                        f.lines().collect::<Result<Vec<String>, io::Error>>()?
                    ))
                })
                .collect::<Result<HashMap<String, Vec<String>>, io::Error>>()?,
        })
    }
}

impl TestRun {
    fn new(dir: &Path) -> Result<TestRun, io::Error> {
        let mut logdir = dir.to_path_buf();
        logdir.push("nix-test-matrix-log");

        let mut testdir = logdir.clone();
        testdir.push("tests");

        Ok(TestRun{
            details: Details::new(&logdir)?,
            tests: fs::read_dir(testdir)?
                .collect::<Result<Vec<_>, io::Error>>()?
                .into_iter()
                .filter(|entry| entry.path().is_dir())
                .map::<Result<(String, TestResult), io::Error>, _>(|entry| {
                    Ok((entry.file_name().as_os_str().to_string_lossy().to_string(),
                        TestResult::new(&entry.path())?
                    ))
                })
                .collect::<Result<HashMap<String, TestResult>, io::Error>>()?,
        })
    }
}

impl TestResult {
    fn new(dir: &Path) -> Result<TestResult, io::Error> {
        let root = dir.to_path_buf();

        let mut duration_file = root.clone();
        duration_file.push("duration");
        let mut duration_fp = File::open(duration_file)?;
        let mut duration_string = String::new();
        duration_fp.read_to_string(&mut duration_string)?;

        let mut exitcode_file = root.clone();
        exitcode_file.push("exitcode");
        let mut exitcode_fp = File::open(exitcode_file)?;
        let mut exitcode_string = String::new();
        exitcode_fp.read_to_string(&mut exitcode_string)?;

        let mut log_file = root.clone();
        log_file.push("log");
        let f = BufReader::new(File::open(log_file)?);

        Ok(TestResult{
            duration: duration_string.trim().parse::<u16>().expect("Failed to parse duration"),
            exitcode: exitcode_string.trim().parse::<u8>().expect("Failed to parse exitcode"),
            log: f.lines().collect::<Result<Vec<String>, io::Error>>()?,
        })
    }
}

impl TestEnvironments {
    fn new(dir: &Path) -> Result<TestEnvironments, io::Error> {
        let test_environments = fs::read_dir(dir)?
            .collect::<Result<Vec<_>, io::Error>>()?
            .into_iter()
            .map::<Result<TestEnvironment, _>, _>(|entry| TestEnvironment::from_dir(&entry.path()))
            .collect::<Result<Vec<TestEnvironment>, io::Error>>()?
            ;

        Ok(TestEnvironments { environments: test_environments, })
    }
}


impl TestEnvironment {
    fn from_dir(dir: &Path) -> Result<TestEnvironment, io::Error> {
        let mut test_results_dir = dir.to_path_buf();
        test_results_dir.push("test-results");

        Ok(TestEnvironment{
            name: dir.file_name().unwrap().to_string_lossy().to_string(),
            details: Details::new(dir)?,
            runs: fs::read_dir(test_results_dir)?
                .collect::<Result<Vec<_>, io::Error>>()?
                .into_iter()
                .filter(|entry| entry.path().is_dir())
                .map::<Result<(String, TestRun), io::Error>, _>(|entry| {
                    Ok((entry.file_name().as_os_str().to_string_lossy().to_string(),
                        TestRun::new(&entry.path())?
                    ))
                })
                .collect::<Result<HashMap<String, TestRun>, io::Error>>()?,
        })
     }
}

fn main() {
    let opt = Opt::from_args();

    let environments = TestEnvironments::new(&opt.input).unwrap();
    for environment in environments.environments {
        println!("Test environment: {}", environment.name);
        for (name, contents) in environment.details.details.iter() {
            if contents.len() == 0 {
                println!("\t - {}", name);
            } else if contents.len() == 1 {
                println!("\t{}: {}", name, contents[0]);
            } else {
                println!("\t{} (first 5 lines):", name);
                let lines_to_print = contents.iter()
                    .take(5)
                    .map::<String,_>(|s| format!("\t\t{}", s)) // Note: leaks through 0x0d1b5b4b and other shell code
                    .collect::<Vec<_>>()
                    .join("\n");

                println!("{}", lines_to_print);
            }
        }

        println!("\n\n");
        for (name, run) in environment.runs.iter() {
            println!("\tTest run: {:?}", name);
            for (name, contents) in run.details.details.iter() {
                if contents.len() == 0 {
                    println!("\t\t - {} (no output)", name);
                } else if contents.len() == 1 {
                    println!("\t\t{}: {}", name, contents[0]);
                } else {
                    println!("\t\t{} (first 5 lines):", name);
                    let lines_to_print = contents.iter()
                        .take(5)
                        .map::<String,_>(|s| format!("\t\t\t{}", s)) // Note: leaks through 0x0d1b5b4b and other shell code
                        .collect::<Vec<_>>()
                        .join("\n");

                    println!("{}", lines_to_print);
                }
            }

            for (name, result) in run.tests.iter() {
                println!("\t\t\tTest result {} (exit: {}, duration: {}s)", name, result.exitcode, result.duration);
                    let lines_to_print = result.log.iter()
                        .take(5)
                        .map::<String,_>(|s| format!("\t\t\t\t{}", s)) // Note: leaks through 0x0d1b5b4b and other shell code
                        .collect::<Vec<_>>()
                        .join("\n");

                    println!("{}", lines_to_print);

                println!("\n\n");


            }
        }

        println!("\n\n\n\n\n");
    }
}
