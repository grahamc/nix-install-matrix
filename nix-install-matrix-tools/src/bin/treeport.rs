#[macro_use]
extern crate structopt;
extern crate regex;
extern crate nix_install_matrix_tools;

use nix_install_matrix_tools::filetree::FileTree;
use nix_install_matrix_tools::resulttree::TestEnvironments;
use nix_install_matrix_tools::resulttree::read_file_string;
use nix_install_matrix_tools::resulttree::parse_results;

use std::io::Write;
use std::io;
use std::path::PathBuf;
use structopt::StructOpt;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;

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

fn nl2br(logs: &str) -> String {
    logs
        .split("\n")
        .map(|s| format!("{}<br>\n", s))
        .collect::<Vec<String>>()
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

#[derive(Debug)]
struct InMemoryTestResult {
    duration: u16,
    exitcode: u8,
    log: String
}

#[derive(Debug)]
struct PartialInMemoryTestResult {
    duration: Option<u16>,
    exitcode: Option<u8>,
    log: Option<String>,
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


#[derive(Debug)]
struct PartialResultTable {
    result_table: ResultTable,
    // environment, scenario, testcase
    partial_results: HashMap<TestResultIdentifier, PartialInMemoryTestResult>,
    errors: HashMap<PathBuf, io::Error>,
    paths_not_utf8: Vec<PathBuf>,
    ignored_paths: Vec<PathBuf>,
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

    let results = results_table(parse_results(tree));
    write_data(&results, &mut out).unwrap();
}
