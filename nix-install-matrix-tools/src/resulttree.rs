
use filetree::FileTreeNode;
use filetree::DirectoryNode;
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

        let (extra_files, environmentdirectories) = environmentdirs.partition();
        if extra_files.len() > 0 {
            panic!("Unexpected files: {:?}", extra_files);
        }

        // Traverse from ./log-output/ in to ./log-output/test-environment
        for mut environmentnode in environmentdirectories.into_iter() {
            envs.environments.push(test_environment_from_directory(environmentnode));
        }
    } else {
        panic!("top level is not a directory");
    }

    envs
}

fn test_environment_from_directory(mut environment_node: DirectoryNode) -> TestEnvironment {
    let mut env = TestEnvironment {
        name: environment_node.name.to_string(),
        details: HashMap::new(),
        runs: HashMap::new(),
    };

    let environment_scenario_directory = environment_node.subtree.directory("test-results").expect("Missing test-results directory");
    let (environment_detail_files, extra_directories) = environment_node.subtree.partition();
    if extra_directories.len() > 0 {
        panic!("Expected only one directory named test-results: {:?}", extra_directories);
    }

    for environment_detail_file in environment_detail_files {
        env.details.insert(environment_detail_file.name, read_file_string(&mut File::open(environment_detail_file.path).unwrap()));
    }

    let (extra_files, test_scenarios_directories) = environment_scenario_directory.subtree.partition();
    if extra_files.len() > 0 {
        panic!("unexpected files: {:?}", extra_files);
    }

    for mut scenario_directory_node in test_scenarios_directories {
        env.runs.insert(
            scenario_directory_node.name.clone(),
            test_run_from_directory(scenario_directory_node)
        );
    }

    env
}

fn test_run_from_directory(mut scenario_directory_node: DirectoryNode) -> TestRun {
    let mut run = TestRun {
        details: HashMap::new(),
        tests: HashMap::new(),
    };

    let mut scenario_matrix_log_directory = scenario_directory_node.subtree.directory("nix-test-matrix-log").unwrap();
    let (extra_files, extra_directories) = scenario_directory_node.subtree.partition();
    if extra_files.len() > 0 {
        panic!("unexpected files: {:?}", extra_files);
    }
    if extra_directories.len() > 0 {
        panic!("Expected only one directory, nix-test-matrix-log: {:?}", extra_directories);
    }

    let scenario_test_runs = scenario_matrix_log_directory.subtree.directory("tests").unwrap();
    let (scenario_details_files, extra_directories) = scenario_matrix_log_directory.subtree.partition();
    for detail in scenario_details_files {
        run.details.insert(detail.name, read_file_string(&mut File::open(detail.path).unwrap()));
    }
    if extra_directories.len() > 0 {
        panic!("Expected only one directory,  tests: {:?}", extra_directories);
    }

    let (extra_files, scenario_test_result_dirs) = scenario_test_runs.subtree.partition();
    if extra_files.len() > 0 {
        panic!("unexpected files: {:?}", extra_files);
    }

    for mut test_run_directory in scenario_test_result_dirs {
        run.tests.insert(
            test_run_directory.name.clone(),
            test_result_from_directory(test_run_directory)
        );
    }

    run
}

fn test_result_from_directory(mut test_run_directory: DirectoryNode) -> TestResult {
    let duration = test_run_directory.subtree.file("duration").unwrap();
    let exitcode = test_run_directory.subtree.file("exitcode").unwrap();
    let log = test_run_directory.subtree.file("log").unwrap();

    let (extra_files, extra_directories) = test_run_directory.subtree.partition();
    if extra_files.len() > 0 {
        panic!("unexpected files: {:?}", extra_files);
    }
    if extra_directories.len() > 0 {
        panic!("Expected only files named duration, exitcode, and log: {:?}", extra_directories);
    }

    TestResult {
        duration: read_file_u16(&mut File::open(duration.path).unwrap()),
        exitcode: read_file_u8(&mut File::open(exitcode.path).unwrap()),
        log: File::open(log.path).unwrap(),
    }
}
