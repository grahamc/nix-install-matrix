#[macro_use]
extern crate structopt;
extern crate regex;

use std::io::Write;
use std::ffi::OsString;
use std::io::BufRead;
use std::io;
use std::io::BufReader;
use std::path::PathBuf;
use structopt::StructOpt;
use std::fs;
use std::path::Path;
use std::collections::HashMap;
use std::collections::HashSet;
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

fn nl2br(logs: &str) -> String {
    logs
        .split("\n")
        .map(|s| format!("{}<br>\n", s))
        .collect::<Vec<String>>()
        .join("\n")
}

fn sample_log(logs: &str) -> String {
    logs
        .split("\n")
        .collect::<Vec<&str>>()
        .into_iter()
        .take(3)
        .collect::<Vec<&str>>()
        .join("\n")
}

fn sample_log_end(logs: &str) -> String {
    let split_iter = logs
        .trim()
        .split("\n")
        .collect::<Vec<&str>>();

    if split_iter.len() > 4 {
        format!(
            "(truncated)\n{}",
            split_iter
                .into_iter()
                .rev()
                .take(3)
                .collect::<Vec<&str>>()
                .into_iter()
                .rev()
                .collect::<Vec<&str>>()
                .join("\n")
        )
    } else {
            split_iter
                .join("\n")
    }
}

fn read_file_string(filep: &mut File) -> String {
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

fn flatten_tree(top: FileTreeNode) -> HashMap<String, File> {
    match top {
        FileTreeNode::File(name, handle) => {
            let mut map: HashMap<String, File> = HashMap::new();
            map.insert(name, handle);
            return map;
        }
        FileTreeNode::Directory(name, node) => {
            let mut map: HashMap<String, File> = HashMap::new();

            for (_, entry) in node.files.into_iter() {
                for (sname, sentry) in flatten_tree(entry).into_iter() {
                    map.insert(
                        format!("{}/{}", name, sname),
                        sentry
                    );
                }
            }

            map
        }
    }
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
                        FileTreeNode::File(enviromentmetafilename, mut handle) => {
                            env.details.insert(enviromentmetafilename, read_file_string(&mut handle));
                        }

                        // Enter ./log-output/test-environment/test-results/
                        FileTreeNode::Directory(testresultfilename, testrunnode) => {
                            if testresultfilename != "test-results" {
                                panic!("Directory should be named test-results, is named {}", testresultfilename);
                            }

                            for (testname, testrunnode) in testrunnode.files.into_iter() {
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
                                                    FileTreeNode::File(enviromentmetafilename, mut handle) => {
                                                        runs.details.insert(enviromentmetafilename, read_file_string(&mut handle));
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
                                    env.runs.insert(testname, runs);
                                } else {
                                    panic!("why is there a file here");
                                }
                            }

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

#[derive(Debug)]
struct TestEnvironments {
    environments: Vec<TestEnvironment>
}

#[derive(Debug)]
struct TestEnvironment {
    name: String,
    details: HashMap<String, String>,
    runs: HashMap<String, TestRun>
}

#[derive(Debug)]
struct TestRun {
    details: HashMap<String, String>,
    tests: HashMap<String, TestResult>
}

#[derive(Debug)]
struct TestResult {
    duration: u16,
    exitcode: u8,
    log: File
}

#[derive(Debug)]
struct InMemoryTestResult {
    duration: u16,
    exitcode: u8,
    log: String
}

#[derive(Debug)]
struct ResultTable {
    environments: HashSet<String>, // HashMap<String, File>)>,
    environment_details: HashMap<String, HashMap<String, String>>,
    scenarios: HashSet<String>,
    testcases: HashSet<String>,
    // environment, scenario, testcase
    results: HashMap<TestResultIdentifier, InMemoryTestResult>
}

#[derive(Debug,Eq,PartialEq,Hash)]
struct TestResultIdentifier {
    environment: String,
    scenario: String,
    testcase: String,
}

impl ResultTable {
    fn get_result(&self, environment: &str, scenario: &str, testcase: &str) -> Option<&InMemoryTestResult> {
        let id = TestResultIdentifier {
            environment: environment.to_string(),
            scenario: scenario.to_string(),
            testcase: testcase.to_string(),
        };
        self.results.get(&id)
    }

    fn get_environment_details(&self, environment: &str) -> Option<&HashMap<String, String>> {
        self.environment_details.get(&environment.to_string())
    }
}

fn results_table(envs: TestEnvironments) -> ResultTable {
    let mut results = ResultTable {
        testcases: HashSet::new(),
        scenarios: HashSet::new(),
        environments: HashSet::new(),
        environment_details: HashMap::new(),
        results: HashMap::new()
    };

    for environment in envs.environments.into_iter() {
        results.environments.insert(environment.name.clone());

        results.environment_details.insert(environment.name.clone(), environment.details);

        for (scenario, run) in environment.runs {
            results.scenarios.insert(scenario.clone());

            for (case, mut test) in run.tests.into_iter() {
                results.testcases.insert(case.clone());
                let id = TestResultIdentifier {
                    environment: environment.name.clone(),
                    scenario: scenario.clone(),
                    testcase: case,
                };

                let value = InMemoryTestResult {
                    exitcode: test.exitcode,
                    duration: test.duration,
                    log: read_file_string(&mut test.log),
                };

                results.results.insert(id, value);
            }
        }
    }

    results
}

fn write_data(table: &ResultTable, out: &mut File) -> Result<(), io::Error> {
    let env_results: Vec<String> = table.environments
        .iter()
        .map(|environment| format!(r#"
<thead>
  <tr class="environment-row">
    <th colspan={scenario_td_count} class="environment-name" id="{environment}">{environment}</th>
</tr>
    <td colspan={scenario_td_count}>
{environment_details}
</td>
</tr>
<tr>
    <td>&nbsp;</td>{scenario_names}
  </tr>
</thead><tbody>
{test_result_rows}
</tbody>

"#,
                                   environment=environment,
                                   environment_details=table.get_environment_details(environment).unwrap()
                                   .iter()
                                   .map(|(name, data)| {
                                       if data.trim().clone().split("\n").collect::<Vec<&str>>().len() == 1 {
                                           format!("<p><strong>{name}:</strong> <tt>{data}</tt></p>",
                                                   name=name, data=data)
                                       } else {
                                           format!(r##"<p><strong>{name}:</strong><a href="#detail-{environment}-{name}">(see full dataset)</a></p>"##,
                                                   environment=environment,
                                                   name=name)
                                       }
                                   })
                                   .collect::<Vec<String>>()
                                   .join("\n"),
                                   scenario_td_count = table.scenarios.len() + 1,
                                   scenario_names = table.scenarios.iter()
                                   .map(|name| format!("<td class=scenario-name>{}</td>", name))
                                   .collect::<Vec<String>>()
                                   .join("\n"),
                                   test_result_rows = table.testcases
                                   .iter()
                                   .map(|testcase_name| format!(r#"
<tr class="result-status-row"><td class="testcase-name">{testcase_name}</td>
{test_run_results}
</tr>
"#,
                                                                testcase_name = testcase_name,
                                                                test_run_results =
                                                                table.scenarios.iter()

                                                                .map(|scenario_name|
                                                                     {
                                                                         match table.get_result(environment, scenario_name, testcase_name) {
                                                                             Some(ref result) => {
                                                                                 let passfail: String;
                                                                                 let passtext: String;
                                                                                 if result.exitcode == 0 {
                                                                                     passfail = format!("pass");
                                                                                     passtext = format!("pass");
                                                                                 } else {
                                                                                     passfail = format!("fail");
                                                                                     passtext = format!("exit code {}", result.exitcode);
                                                                                 };

                                                                                 format!(r##"
<td class="test-result test-result-{passfail}">
<div class="test-result-summary"><span class=test-result-{passfail}>{passfailtext}</span> in {test_duration}s <a href="#{environment}-{scenario}-{testcase}">(full log)</a></div/>
<tt class="test-result-log-sample">{logs}<tt>
</td>

"##,
                                                                                         environment=environment,
                                                                                         scenario=scenario_name,
                                                                                         testcase=testcase_name,
                                                                                         passfail=passfail,
                                                                                         passfailtext=passtext,
                                                                                         test_duration=result.duration,
                                                                                         logs=nl2br(&sample_log_end(&result.log)))
                                                                             },
                                                                             None => {
                                                                                 format!(r#"
<td class=test-result>
None
</td>"#)
                                                                             }
                                                                         }
                                                                     })
                                                                .collect::<Vec<String>>()
                                                                .join("\n")
                                   )
                                   ).collect::<Vec<String>>().join("\n")
        ))
        .collect();

    out.write(format!(r#"<style>

body {{
  font-family: sans-serif;
}}

td, th {{
  padding: 10px;
  vertical-align: top;
}}

table {{
   border-collapse: collapse;
   width: 100%;
}}

tr.environment-row > td, tr.environment-row > th {{
    border-bottom: 5px double black;
}}

.environment-row {{
    border-top: 2px solid black;
}}

.scenario-name {{
    font-style: italic;
    text-align: center;
}}

.testcase-name {{
    font-style: italic;
    text-align: right;
    vertical-align: top;
}}

.environment-name {{
    text-align: left;
}}

.test-result {{
    border: 1px solid;
}}

.environment-details-row > td {{
  background-color: #ffffff;
}}

tbody > tr.result-status-row:nth-child(even) {{
    background-color: #f2f2f2
}}

.test-result-summary > a {{
  font-style: italic;
}}

.environment-details-row > td {{
  padding: 0px 0px 0px 0px;
  line-height: 0px;
}}

td.environment-details-cell {{
  line-height: 16px;
  vertical-align: top;
  width: 10%;
  padding: 10px; 10px 10px 10px;
}}

tr p {{
  margin: 2px;
}}

.test-result.test-result-fail {{
    background-color: #ffe4e4;
}}

.test-result.test-result-pass {{
    background-color: #eaffea;
}}

.test-result-log-sample {{
  font-size: 0.8em;
}}

.test-result-summary > a {{
  font-size: 0.8em;
}}

.environment-row > td, .environment-row > th {{
  vertical-align: bottom;
  text-align: center;
}}

.environment-name {{
    background-color: black;
    color: white;
}}

pre {{
  max-width: 100%;
  overflow: auto;
}}

</style>
<ul>
{environment_index}
</ul>
<table borders=1>
{environment_results}
</table>
<h1>all logs</h1>
{logs}
<h1>all details</h1>
{all_details}
"#,
                      environment_results=env_results.join("\n"),
                      logs=table.results
                      .iter()
                      .map(|(ref id, ref result)| {
                          format!(r##"
<h2 id="{environment}-{scenario}-{testcase}">{environment}-{scenario}-{testcase}</h2>
<pre>
{log}
</pre>
<a href="#{environment}-{scenario}-{testcase}">top of log</a>
"##,
                                  environment=id.environment,
                                  scenario=id.scenario,
                                  testcase=id.testcase,
                                  log=result.log
                          )
                      })
                      .collect::<Vec<String>>()
                      .join("\n"),
                      environment_index=table.environments
                      .iter()
                      .map(|environment| format!(r##"
<li><a href="#{environment}">{environment}</a></li>
"##,
                                                 environment=environment,
                      ))
                      .collect::<Vec<String>>()
                      .join("\n"),
                      all_details=table.environment_details
                      .iter()
                      .flat_map(|(environment, details)| {
                          details.iter().map(|(name,data)| {
                              format!(r##"
<h2 id="detail-{environment}-{name}">{environment}-{name}</h1>
<pre>{data}</pre>
<a href="#detail-{environment}-{name}">top of data</a>
"##,
                                      environment=environment,
                                      name=name,
                                      data=data
                              )
                          }).collect::<Vec<String>>()
                      })
                      .collect::<Vec<String>>()
                      .join("\n"),

    ).as_bytes()

    )?;

    Ok(())
}


fn main() {
    let opt = Opt::from_args();

    let tree = FileTree::new(&opt.input).unwrap();
    let mut out = File::create(&opt.output).unwrap();

    print!("{:?}", tree);
    let results = results_table(parse_results(tree));
    write_data(&results, &mut out).unwrap();
}
