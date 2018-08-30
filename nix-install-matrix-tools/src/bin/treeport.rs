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

#[derive(Debug,Clone)]
struct InMemoryTestResult {
    environment: String,
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
    environments: Vec<String>, // HashMap<String, File>)>,
    environment_details: HashMap<String, HashMap<String, String>>,
    scenarios: Vec<String>,
    testcases: Vec<String>,
    // environment, scenario, testcase
    results: HashMap<TestResultIdentifier, InMemoryTestResult>,

    install_methods: Vec<String>,
    images: Vec<String>,
    result_by_install_methods_images: HashMap<DeepTestResultIdentifier, InMemoryTestResult>,
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


#[derive(Debug,Eq,PartialEq,Hash)]
struct DeepTestResultIdentifier {
    install_method: String,
    image: String,
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


    fn get_test_result(&self, install_method: &str, image: &str, scenario: &str, testcase: &str) -> Option<&InMemoryTestResult> {
        let id = DeepTestResultIdentifier {
            install_method: install_method.to_string(),
            image: image.to_string(),
            scenario: scenario.to_string(),
            testcase: testcase.to_string(),
        };

        self.result_by_install_methods_images.get(&id)
    }
}

fn results_table(envs: TestEnvironments) -> ResultTable {
    let mut results = ResultTable {
        testcases: Vec::new(),
        scenarios: Vec::new(),
        environments: Vec::new(),
        environment_details: HashMap::new(),
        results: HashMap::new(),
        result_by_install_methods_images: HashMap::new(),

        install_methods: Vec::new(),
        images: Vec::new(),
    };

    let mut testcase_names: HashSet<String> = HashSet::new();
    let mut scenario_names: HashSet<String> = HashSet::new();
    let mut environment_names: HashSet<String> = HashSet::new();

    let mut install_method_names: HashSet<String> = HashSet::new();
    let mut image_names: HashSet<String> = HashSet::new();

    for environment in envs.environments.into_iter() {
        environment_names.insert(environment.name.clone());

        let mut found_install_method: Option<String> = None;
        if let Some(install_method) = environment.details.get("install-method") {
            install_method_names.insert(install_method.clone());
            found_install_method = Some(install_method.clone());
        } else {
            println!("NO INSTALL METHOD FOR {}", environment.name);
        }

        let mut found_image_name: Option<String> = None;
        if let Some(image_name) = environment.details.get("image-name") {
            image_names.insert(image_name.clone());
            found_image_name = Some(image_name.clone());
        } else {
            println!("NO IMAGE NAME FOR {}", environment.name);
        }

        results.environment_details.insert(environment.name.clone(), environment.details);

        for (scenario, run) in environment.runs {
            scenario_names.insert(scenario.clone());

            for (case, mut test) in run.tests.into_iter() {
                testcase_names.insert(case.clone());
                let id = TestResultIdentifier {
                    environment: environment.name.clone(),
                    scenario: scenario.clone(),
                    testcase: case.clone(),
                };

                let value = InMemoryTestResult {
                    environment: environment.name.clone(),
                    exitcode: test.exitcode,
                    duration: test.duration,
                    log: read_file_string(&mut File::open(test.log).unwrap()),
                };

                results.results.insert(id, value.clone());
                if let Some(ref install_method) = found_install_method {
                    if let Some(ref image_name) = found_image_name {
                        let id = DeepTestResultIdentifier {
                            install_method: install_method.clone(),
                            image: image_name.clone(),
                            scenario: scenario.clone(),
                            testcase: case,
                        };

                        results.result_by_install_methods_images.insert(id, value);
                    }
                }
            }
        }
    }

    results.environments = environment_names.into_iter().collect();
    results.environments.sort_by(|a, b| a.chars().rev().cmp(b.chars().rev()));
    results.environments.reverse(); // lol
    results.testcases = testcase_names.into_iter().collect();
    results.testcases.sort();
    results.scenarios = scenario_names.into_iter().collect();
    results.scenarios.sort();

    results.images = image_names.into_iter().collect();
    results.images.sort();
    results.install_methods = install_method_names.into_iter().collect();
    results.install_methods.sort();

    results
}

fn write_data(table: &ResultTable, out: &mut File) -> Result<(), io::Error> {
    let install_method_headers = table.install_methods.iter()
        .map(|install_method| {
            format!(r##"
<th colspan="{scenario_count}">{install_method}</th>
"##,
                    scenario_count = table.scenarios.len(),
                    install_method=install_method,
            )
        })
        .collect::<Vec<String>>()
        .join("\n");
    let scenario_columns_repeated = table.install_methods.iter()
        .flat_map(|_install_method| {
            table.scenarios.iter().map(|scenario| {
                format!(r##"
<th><div>{scenario}</div></th>
"##,
                        scenario=scenario
                )
            }).collect::<Vec<String>>()
        })
        .collect::<Vec<String>>()
        .join("\n");

    let per_image_rows = table.images.iter()
        .map(|image| {

            let test_result_rows = table.testcases.iter()
                .map(|testcase| {

                    let test_results_per_install_method: String = table.install_methods.iter()
                        .flat_map(|install_method| {
                            table.scenarios.iter()
                                .map(|scenario| {

                                    let result = table.get_test_result(install_method, image, scenario, testcase);

                                    let status_class: &str;
                                    let symbol: &str;
                                    let target: &str;

                                    if let Some(res) = result {
                                        target = &res.environment;
                                        if res.exitcode == 0 {
                                            status_class = "test-result-pass";
                                            symbol = "P";
                                        } else {
                                            status_class = "test-result-fail";
                                            symbol = "F";
                                        }
                                    } else {
                                        status_class = "";
                                        symbol = "S";
                                        target = "";
                                    }


                                    format!(r##"<td class='test-square test-result {status_class}'><a href="#{target}"><span>{symbol}</span></a></td>"##,
                                            status_class=status_class,
                                            target=target,
                                            symbol=symbol,
                                    )
                                })
                                .collect::<Vec<String>>()
                        })
                        .collect::<Vec<String>>()
                        .join("\n");

                    format!(r##"
<tr>
<th>{testcase}</th>
{test_results_per_install_method}
</tr>
"##,
                            testcase=testcase,
                            test_results_per_install_method=test_results_per_install_method,
                    )
                })
                .collect::<Vec<String>>()
                .join("\n");

            format!(r##"
<tr>
<th rowspan="{test_count}" colspan="1">{image}</th>
</tr>
{test_result_rows}
"##,
                    image=image,
                    test_count=table.testcases.len() + 1,
                    test_result_rows=test_result_rows
            )
        })
        .collect::<Vec<String>>()
        .join("\n");

    let summary_table: String = format!(r##"
<table class="summary">
  <tr><td>&nbsp;</td><td>&nbsp;</td>{install_method_headers}</tr>
  <tr class="summary-scenarios"><td>&nbsp;</td><td>&nbsp;</td>{scenario_columns_repeated}</tr>
{per_image_rows}
</table>
"##,
                                        install_method_headers=install_method_headers,
                                        per_image_rows=per_image_rows,
                                        scenario_columns_repeated=scenario_columns_repeated,
    );

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

tr.summary-scenarios > th > div {{
  transform: rotate(-45deg);
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

tr.summary-scenarios > th {{
    width: 3em;
    max-width: 3em;
    min-width: 3em;
    vertical-align: bottom;
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


td.test-square {{
  height: 3em;
  max-height: 3em;
  min-height: 3em;

  width: 3em;
  max-width: 3em;
  min-width: 3em;
  padding: 0;
  text-align: center;
  vertical-align: middle;
}}

.summary {{
width: auto;
}}

.summary a {{
    display: block;
    height: 100%;
    width: 100%;
    position: relative;
}}

.summary a > span {{
  position: absolute;
  top: 50%;
  transform: translateY(-50%);
  left: 0;
  right: 0;
}}

</style>

{summary_table}

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
                      summary_table=summary_table,
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
