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

fn parse_results(top: FileTreeNode) {
    if let FileTreeNode::Directory(name, environmentdirs) = top {
        let mut envs = TestEnvironments {
            environments: vec![],
        };

        for (name, node) in environmentdirs.files.into_iter() {
            println!("{:?}", name);
            if let FileTreeNode::Directory(envname, testdatadir) = node {
                let mut env = TestEnvironment {
                    name: name.to_string(),
                    details: HashMap::new(),
                    runs: HashMap::new(),
                };

                for (name, node) in testdatadir.files.into_iter() {
                    match node {
                        FileTreeNode::File(detailname, handle) => {
                            env.details.insert(detailname, handle);
                        }

                        FileTreeNode::Directory(_, testnodes) => {
                            /*
                            for (subname, subnode) in testnodes.files.into_iter() {
                                if let FileTreeNode::Directory(testname, testnode) = subnode {
                                    for (subname, subnode) in testnodes.files.into_iter() {
                                        if let FileTreeNode::Directory(_, testresults) = testnode.files.remove("nix-test-matrix-log").unwrap() {

                                            let mut run = TestRun {
                                                details: HashMap::new(),
                                                tests: HashMap::new(),
                                            };

                                            for (testname, testresults) in testresults.files.into_iter() {
                                                match node {
                                                    FileTreeNode::File(detailname, handle) => {
                                                        run.details.insert(detailname, handle);
                                                    }

                                                    FileTreeNode::Directory(testname, testnodes) => {

                                                    }
                                                }
                                            }

                                            println!("{:?}", run);
                                            env.runs.insert(testname, run);
                                        }
                                    }

                                } else {
                                    panic!("why is there a file here");
                                }
                            }
                            */
                        }
                    }
                }


                envs.environments.push(env);
            } else {
                panic!("Unexpected non-dir node");
            }
        }

        println!("{:?}", envs);
    } else {
        panic!("top level is not a directory");
    }
}

fn main() {
    let opt = Opt::from_args();

    let tree = FileTree::new(&opt.input).unwrap();
    // print!("{}", print_tree(&tree));
    parse_results(tree);
}
