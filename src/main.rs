#[macro_use]
extern crate serde;
#[macro_use]
extern crate serde_json;

mod problem;

use std::collections::BTreeSet;
// use std::env;
use std::fs;
use std::io;
use std::io::BufRead;
use std::io::Write;
use std::path::Path;

/// main() helps to generate the submission template .rs
fn main() {
    println!("Welcome to leetcode-rust system.");
    let mut solved_ids = get_solved_ids();
    loop {
        println!("Please enter a frontend problem id, or \"random\" to generate a random one.");
        // let mut is_random = false;
        let id: u32;
        let mut id_arg = String::new();
        io::stdin()
            .read_line(&mut id_arg)
            .expect("Failed to read line");
        let id_arg = id_arg.trim();
        match id_arg {
            "random" => {
                println!("You select random mode.");
                id = generate_random_id(&solved_ids);
                // is_random = true;
                println!("Generate random problem: {}", &id);
            }
            _ => {
                id = id_arg
                    .parse::<u32>()
                    .unwrap_or_else(|_| panic!("not a number: {}", id_arg));
                if solved_ids.contains(&id) {
                    println!(
                        "The problem you chose is invalid (the problem may have been solved \
                         or may have no rust version)."
                    );
                    continue;
                }
            }
        }

        let problem = problem::get_problem(id).unwrap_or_else(|| {
            panic!(
                "Error: failed to get problem #{} \
                 (The problem may be paid-only or may not be exist).",
                id
            )
        });
        let code = problem.code_definition.iter().find(|&d| d.value == "rust");
        if code.is_none() {
            println!("Problem {} has no rust version.", &id);
            solved_ids.push(problem.question_id);
            continue;
        }
        let code = code.unwrap();

        let file_name = format!(
            "n{:04}_{}",
            problem.question_id,
            problem.title_slug.replace("-", "_")
        );
        let file_path = Path::new("./src").join(format!("{}.rs", file_name));
        if file_path.exists() {
            panic!("problem already initialized");
        }

        let template = fs::read_to_string("./template.rs").unwrap();
        let source = template
            .replace("__PROBLEM_TITLE__", &problem.title)
            .replace("__PROBLEM_DESC__", &build_desc(&problem.content))
            .replace("__PROBLEM_DEFAULT_CODE__", &code.default_code)
            .replace("__PROBLEM_ID__", &format!("{:04}", problem.question_id))
            .replace("__EXTRA_USE__", &parse_extra_use(&code.default_code));

        let mut file = fs::OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&file_path)
            .unwrap();

        file.write_all(source.as_bytes()).unwrap();
        drop(file);

        let mut first_line: Option<usize> = None;
        let mut last_line: Option<usize> = None;
        let mut headers: Vec<String> = Vec::new();
        let mut modules: BTreeSet<String> = BTreeSet::new();
        let mut trailers: Vec<String> = Vec::new();
        let input = fs::OpenOptions::new()
            .read(true)
            .open("./src/lib.rs")
            .unwrap();
        let buffered = io::BufReader::new(input);
        for (i, line) in buffered.lines().map(|line| line.unwrap()).enumerate() {
            if line.starts_with("pub mod n") {
                if first_line.is_none() {
                    first_line = Some(i);
                }
                last_line = Some(i);
                modules.insert(line);
            } else if first_line.is_none() {
                headers.push(line);
            } else {
                trailers.push(line);
            }
        }
        if first_line.is_none() || last_line.is_none() {
            let mut lib_file = fs::OpenOptions::new()
                .write(true)
                .append(true)
                .open("./src/lib.rs")
                .unwrap();
            writeln!(lib_file, "pub mod {};", file_name).unwrap();
        } else {
            modules.insert(format!("pub mod {};", file_name));
            let mut lib_file = fs::OpenOptions::new()
                .write(true)
                .truncate(true)
                .open("./src/lib.rs")
                .unwrap();
            for line in headers
                .into_iter()
                .chain(modules.into_iter())
                .chain(trailers.into_iter())
            {
                writeln!(lib_file, "{}", line).unwrap();
            }
        }
        break;
    }
}

fn generate_random_id(except_ids: &[u32]) -> u32 {
    use rand::Rng;
    // use std::fs;
    let mut rng = rand::thread_rng();
    loop {
        let res: u32 = rng.gen_range(1, 1106);
        if !except_ids.contains(&res) {
            return res;
        }
        println!(
            "Generate a random num ({}), but it is invalid (the problem may have been solved \
             or may have no rust version). Regenerate..",
            res
        );
    }
}

fn get_solved_ids() -> Vec<u32> {
    let paths = fs::read_dir("./src").unwrap();
    let mut solved_ids = Vec::new();

    for path in paths {
        let path = path.unwrap().path();
        let s = path.to_str().unwrap();
        if !s.starts_with('n') {
            continue;
        }
        let id = &s[7..11];
        let id = id.parse::<u32>().unwrap();
        solved_ids.push(id);
    }
    solved_ids
}

fn parse_extra_use(code: &str) -> String {
    let mut extra_use_line = String::new();
    // a linked-list problem
    if code.contains("pub struct ListNode") {
        extra_use_line.push_str("\nuse super::util::linked_list::{ListNode, to_list};")
    }
    if code.contains("pub struct TreeNode") {
        extra_use_line.push_str("\nuse super::util::tree::{TreeNode, to_tree};")
    }
    if code.contains("pub struct Point") {
        extra_use_line.push_str("\nuse super::util::point::Point;")
    }
    extra_use_line
}

fn build_desc(content: &str) -> String {
    // TODO: fix this
    let mut description = content
        .replace("<br>", "")
        .replace("<br/>", "")
        .replace("<br />", "")
        .replace("<strong>", "")
        .replace("</strong>", "")
        .replace("<small>", "")
        .replace("</small>", "")
        .replace("<em>", "")
        .replace("</em>", "")
        .replace("</p>", "")
        .replace("<p>", "")
        .replace("<b>", "")
        .replace("</b>", "")
        .replace("<pre>", "")
        .replace("</pre>", "")
        .replace("<ul>", "")
        .replace("</ul>", "")
        .replace("<li>", "")
        .replace("</li>", "")
        .replace("<ol>", "")
        .replace("</ol>", "")
        .replace("<code>", "")
        .replace("</code>", "")
        .replace("<i>", "")
        .replace("</i>", "")
        .replace("<div>", "")
        .replace("</div>", "")
        .replace("<span>", "")
        .replace("</span>", "")
        .replace("<sub>", "")
        .replace("</sub>", "")
        .replace("</sup>", "")
        .replace("<sup>", "^")
        .replace("&nbsp;", " ")
        .replace("&gt;", ">")
        .replace("&lt;", "<")
        .replace("&quot;", "\"")
        .replace("&minus;", "-")
        .replace("&#39;", "'")
        .replace("\n\n", "\n")
        .replace("\n", "\n * ");
    while let Some(begin) = description.find("<a ") {
        let end = (&description[begin..]).find('>').unwrap();
        description.replace_range(begin..=(begin + end), "");
    }
    while let Some(begin) = description.find("<img ") {
        let end = (&description[begin..]).find('>').unwrap();
        description.replace_range(begin..=(begin + end), "");
    }
    while let Some(begin) = description.find("<span ") {
        let end = (&description[begin..]).find('>').unwrap();
        description.replace_range(begin..=(begin + end), "");
    }
    description
}
