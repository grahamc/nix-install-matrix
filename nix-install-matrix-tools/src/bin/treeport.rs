#[macro_use]
extern crate structopt;

use std::ffi::OsString;
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
struct FileTree {
    files: HashMap<String, FileTreeNode>
}

#[derive(Debug)]
enum FileTreeNode {
    File(String, File),
    Directory(String, FileTree)
}


#[derive(Debug)]
struct TestEnvironments {
    environments: Vec<TestEnvironment>
}

#[derive(Debug)]
struct TestEnvironment {
    name: String,
    details: HashMap<String, File>,
    runs: HashMap<String, TestRun>
}

#[derive(Debug)]
struct TestRun {
    details: HashMap<String, File>,
    tests: HashMap<String, TestResult>
}

#[derive(Debug)]
struct TestResult {
    duration: u16,
    exitcode: u8,
    log: File
}

fn read_file_u16(filep: &mut File) -> u16 {
    let mut ret = String::new();
    filep.read_to_string(&mut ret).expect("should read");
    ret.trim().parse::<u16>().expect("Failed to parse u16")
}

fn read_file_u8(filep: &mut File) -> u8 {
    let mut ret = String::new();
    filep.read_to_string(&mut ret).expect("should read");
    ret.trim().parse::<u8>().expect("Failed to parse u8")
}

impl FileTree {
    fn new(start: &Path) -> Result<FileTreeNode, io::Error> {
        let filename = start.file_name()
            .expect("why can't we find a filename")
            .to_owned();
        let filename_string = filename.to_string_lossy().to_string();

        if start.is_file() {
            return Ok(FileTreeNode::File(filename_string, File::open(start)?))
        } else {
            return Ok(FileTreeNode::Directory(filename_string, FileTree {
                files: fs::read_dir(start)?
                    .collect::<Result<Vec<_>, io::Error>>()?
                    .into_iter()
                    .map::<Result<(String, FileTreeNode), io::Error>, _>(|entry| {
                        Ok((
                            entry.file_name().to_string_lossy().to_string(),
                            FileTree::new(&entry.path())?
                        ))
                    })
                    .collect::<Result<HashMap<String, FileTreeNode>, io::Error>>()?,
            }))
        }
    }
}

fn print_tree(node: &FileTreeNode) -> String {
    let sub = match node {
        &FileTreeNode::File(ref name, ref _handle) => {
            format!("-> file:{}", name)
        }

        &FileTreeNode::Directory(ref name, ref tree) => {
            let lines = tree.files
                .iter()
                .flat_map(|(_name, node)| {
                    print_tree(node)
                        .split("\n")
                        .map(|line| format!("\t{}", line))
                        .collect::<Vec<String>>()
                        .into_iter()
                })
                .collect::<Vec<String>>()
                .join("\n");
            format!("-> dir:{}\n{}",
                    name,
                    lines
            )
        }
    };

    sub
}

fn parse_results(top: FileTreeNode) -> TestEnvironments {
    let mut envs = TestEnvironments {
        environments: vec![],
    };

    // Traverse down from . in to ./log-output
    if let FileTreeNode::Directory(_, environmentdirs) = top {

        // Traverse from ./log-output/ in to ./log-output/test-environment
        for (_, environmentnode) in environmentdirs.files.into_iter() {
            // Everything inside ./log-output/test-environment must be a directory
            if let FileTreeNode::Directory(environmentname, environmentdatanode) = environmentnode {
                let mut env = TestEnvironment {
                    name: environmentname.to_string(),
                    details: HashMap::new(),
                    runs: HashMap::new(),
                };

                for (_, environmentmetanode) in environmentdatanode.files.into_iter() {
                    match environmentmetanode {
                        // Each file inside ./log-output/test-environment/ is metadata
                        FileTreeNode::File(enviromentmetafilename, handle) => {
                            env.details.insert(enviromentmetafilename, handle);
                        }

                        // Enter ./log-output/test-environment/test-results/
                        FileTreeNode::Directory(testresultfilename, testrunnode) => {
                            let mut runs = TestRun {
                                details: HashMap::new(),
                                tests: HashMap::new(),
                            };

                            if testresultfilename != "test-results" {
                                panic!("Directory should be named test-results");
                            }

                            for (_, testrunnode) in testrunnode.files.into_iter() {
                                // Each directory in ./log-output/test-environment/test-run is a specific test run
                                if let FileTreeNode::Directory(testname, testrunnode) = testrunnode {
                                    // Enter ./log-output/test-environment/test-run/test-name/
                                    for (_, nixtestmatrixlognode) in testrunnode.files.into_iter() {
                                        // Enter ./log-output/test-environment/test-run/nix-test-matrix-log/
                                        if let FileTreeNode::Directory(nixtestmatrixlogfilename, testrunmetadir) = nixtestmatrixlognode {
                                            if nixtestmatrixlogfilename != "nix-test-matrix-log" {
                                                panic!("Directory should be named nix-test-matrix-log");
                                            }

                                            for (_, metanode) in testrunmetadir.files.into_iter() {
                                                match metanode {
                                                    // Each file inside ./log-output/test-environment/test-run/nix-test-matrix-log/ is metadata
                                                    FileTreeNode::File(enviromentmetafilename, handle) => {
                                                        runs.details.insert(enviromentmetafilename, handle);
                                                    }

                                                    // Enter ./log-output/test-environment/test-run/nix-test-matrix-log/tests
                                                    FileTreeNode::Directory(testsfilename, testsnode) => {
                                                        if testsfilename != "tests" {
                                                            panic!("Directory should be named tests");
                                                        }

                                                        for (_, testnode) in testsnode.files.into_iter() {
                                                            match testnode {
                                                                // There should be no files here
                                                                FileTreeNode::File(_, _) => {
                                                                    panic!("No file here");
                                                                }

                                                                // Enter ./log-output/test-environment/test-run/nix-test-matrix-log/tests/test-name
                                                                FileTreeNode::Directory(testfilename, mut testnode) => {
                                                                    if let Some(FileTreeNode::File(_, mut durationF)) = testnode.files.remove("duration") {
                                                                        if let Some(FileTreeNode::File(_, mut exitcodeF)) = testnode.files.remove("exitcode") {
                                                                            if let Some(FileTreeNode::File(_, mut logF)) = testnode.files.remove("log") {
                                                                                runs.tests.insert(
                                                                                    testfilename,
                                                                                    TestResult {
                                                                                        duration: read_file_u16(&mut durationF),
                                                                                        exitcode: read_file_u8(&mut exitcodeF),
                                                                                        log: logF
                                                                                    }
                                                                                );
                                                                            }
                                                                        }
                                                                    }

                                                                    // Each file inside ./log-output/test-environment/test-run/nix-test-matrix-log/tests/test-name is metadata
                                                                    // Each file inside ./log-output/test-environment/test-run/nix-test-matrix-log/tests/test-name is metadata
                                                                    // There should only be one directory, ./log-output/test-environment/test-run/nix-test-matrix-log/tests

                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        } else {
                                            panic!("why is there a file here");
                                        }
                                    }
                                } else {
                                    panic!("why is there a file here");
                                }
                            }
                            env.runs.insert(testresultfilename, runs);
                        }
                    }
                }


                envs.environments.push(env);
            } else {
                panic!("Unexpected non-dir node");
            }
        }
    } else {
        panic!("top level is not a directory");
    }

    envs
}

struct ResultTable {
    tests: Vec<String>,
    environments: Vec<String>,
    results: Vec<Vec<Option<TestResult>>>,
}

fn results_table(envs: TestEnvironments) -> ResultTable {
    let mut results = ResultTable {
        tests: vec![],
        environments: vec![],
        results: vec![],
    };

    let mut envid = 0;
    for (name, env) in envs {
        results.insert(envid, env);
        envid += 1;
    }
}

fn main() {
    let opt = Opt::from_args();

    let tree = FileTree::new(&opt.input).unwrap();
    // print!("{}", print_tree(&tree));
    println!("{:?}", parse_results(tree));
}
