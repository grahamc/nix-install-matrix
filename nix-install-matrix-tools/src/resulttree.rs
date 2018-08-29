
use filetree::FileTreeNode;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

#[derive(Debug)]
pub struct TestEnvironments {
    pub environments: Vec<TestEnvironment>
}

#[derive(Debug)]
pub struct TestEnvironment {
    pub name: String,
    pub details: HashMap<String, String>,
    pub runs: HashMap<String, TestRun>
}

#[derive(Debug)]
pub struct TestRun {
    pub details: HashMap<String, String>,
    pub tests: HashMap<String, TestResult>
}

#[derive(Debug)]
pub struct TestResult {
    pub duration: u16,
    pub exitcode: u8,
    pub log: File
}

pub fn read_file_string(filep: &mut File) -> String {
    let mut ret = String::new();
    filep.read_to_string(&mut ret).expect("should read");
    ret
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

pub fn parse_results(top: FileTreeNode) -> TestEnvironments {
    let mut envs = TestEnvironments {
        environments: vec![],
    };

    // Traverse down from . in to ./log-output
    if let FileTreeNode::Directory(_, environmentdirs) = top {

        let (files, directories) = environmentdirs.partition();
        if files.len() > 0 {
            panic!("Unexpected files: {:?}", files);
        }

        // Traverse from ./log-output/ in to ./log-output/test-environment
        for environmentnode in directories.into_iter() {
            let environmentname = environmentnode.name;
            let mut environmentdatanode = environmentnode.subtree;
            let mut env = TestEnvironment {
                name: environmentname.to_string(),
                details: HashMap::new(),
                runs: HashMap::new(),
            };

            let testresults = environmentdatanode.directory("test-results").expect("Missing test-results directory");
            let (detailfiles, directories) = environmentdatanode.partition();
            if directories.len() > 0 {
                panic!("Expected only one directory named test-results: {:?}", directories);
            }

            for file in detailfiles {
                env.details.insert(file.name, read_file_string(&mut File::open(file.path).unwrap()));
            }


            for (_testname, testrunnode) in testresults.subtree.files {
                let mut runs = TestRun {
                    details: HashMap::new(),
                    tests: HashMap::new(),
                };
                // Each directory in ./log-output/test-environment/test-results/test-run is a specific test run
                if let FileTreeNode::Directory(testname, testrunnode) = testrunnode {
                    // Enter ./log-output/test-environment/test-results/test-run/test-name/
                    for (_, nixtestmatrixlognode) in testrunnode.files.into_iter() {
                        // Enter ./log-output/test-environment/test-results/test-run/nix-test-matrix-log/
                        if let FileTreeNode::Directory(nixtestmatrixlogfilename, testrunmetadir) = nixtestmatrixlognode {
                            if nixtestmatrixlogfilename != "nix-test-matrix-log" {
                                panic!("Directory should be named nix-test-matrix-log");
                            }

                            for (_, metanode) in testrunmetadir.files.into_iter() {
                                match metanode {
                                    // Each file inside ./log-output/test-environment/test-results/test-run/nix-test-matrix-log/ is metadata
                                    FileTreeNode::File(enviromentmetafilename, path) => {
                                        runs.details.insert(enviromentmetafilename, read_file_string(&mut File::open(path).unwrap()));
                                    }

                                    // Enter ./log-output/test-environment/test-results/test-run/nix-test-matrix-log/tests
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

                                                // Enter ./log-output/test-environment/test-results/test-run/nix-test-matrix-log/tests/test-name
                                                FileTreeNode::Directory(testfilename, mut testnode) => {
                                                    if let Some(FileTreeNode::File(_, duration_path)) = testnode.files.remove("duration") {
                                                        if let Some(FileTreeNode::File(_, exitcode_path)) = testnode.files.remove("exitcode") {
                                                            if let Some(FileTreeNode::File(_, log_path)) = testnode.files.remove("log") {
                                                                runs.tests.insert(
                                                                    testfilename,
                                                                    TestResult {
                                                                        duration: read_file_u16(&mut File::open(duration_path).unwrap()),
                                                                        exitcode: read_file_u8(&mut File::open(exitcode_path).unwrap()),
                                                                        log: File::open(log_path).unwrap(),
                                                                    }
                                                                );
                                                            }
                                                        }
                                                    }
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
                    env.runs.insert(testname, runs);
                } else {
                    panic!("why is there a file here");
                }
            }

            envs.environments.push(env);
        }
    } else {
        panic!("top level is not a directory");
    }

    envs

}
