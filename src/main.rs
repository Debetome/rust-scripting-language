#[allow(unused_imports)]
use std::fs::File;
use std::process;

#[allow(unused_imports)]
use std::io::{self, Read};
use std::env;

#[allow(unused_imports)]
use std::collections::HashMap;
use regex::Regex;

#[allow(dead_code)]
#[derive(Debug, Clone)]
struct Variable {
    name: String,
    value: String
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
struct Method {
    name: String,
    args: Vec<String>,
    scope: ScopeMemory
}

impl Method {
    pub fn new() -> Self {
        Self {
            name: String::new(),
            args: Vec::new(),
            scope: ScopeMemory::new()
        }
    }
}

#[derive(Debug)]
struct MethodBuilder<'m> {
    method: &'m mut Method
}

#[allow(dead_code)]
impl<'m> MethodBuilder<'m> {
    pub fn new(method: &'m mut Method) -> Self {
        Self { method }
    }

    pub fn set_name(&mut self, name: &str) {
        self.method.name = name.to_owned();
    }

    pub fn set_args(&mut self, args: &str) {
        let args = args
            .replace(" ", "")
            .replace("(", "")
            .replace(")", "")
            .split(",")
            .map(|s| s.to_string())
            .collect::<Vec<String>>();

        self.method.args = args;
    }

    pub fn set_scope(&self) {
        return;
    }
}

#[allow(dead_code)]
#[derive(Debug)]
enum StatementType {
    If,
    For,
    While
}

#[allow(dead_code)]
#[derive(Debug)]
struct Statement {
    statement_type: StatementType,
    scope: ScopeMemory
}

#[allow(dead_code)]
#[derive(Debug)]
struct MethodParser {    
    contents: String
}

#[allow(dead_code)]
#[allow(unused_variables)]
impl MethodParser {
    pub fn new<'c>(contents: &'c str) -> Self {
        Self { contents: contents.to_owned() }
    }

    fn parse_regex(&self, pattern: &str, err: &str, _index: usize) -> Result<String, String> {        
        let re = Regex::new(pattern).unwrap();
        if let Some(captures) = re.captures(self.contents.as_str()) {
            if let Some(matched) = captures.get(_index) {
                println!("match!");
                return Ok(matched.as_str().to_owned());
            }
        }

        return Err(String::from("missing method name"))
    }    

    pub fn parse(&self) -> Result<Method, Vec<String>> {
        let mut parse_errors = Vec::<String>::new();

        let name_pattern = r"\bmethod\s+(\w+)";
        let args_pattern = r"\((.*?)\)";
        let lines_pattern = r"(?sm)^\s{4}";

        let match_results = vec![
            self.parse_regex(name_pattern, "Method name not defined or not found", 1),
            self.parse_regex(args_pattern, "Wrong method naming syntax, '()' either missing at the end of method's name or not closed properly", 0),
            self.parse_regex(lines_pattern, "", 0)
        ];

        for _match in match_results.iter() {
            if let Err(match_err) = _match.as_ref() {
                parse_errors.push(match_err.to_owned());
            }
        }

        if parse_errors.len() > 0 { return Err(parse_errors) }

        let mut method = Method::new();
        let mut method_builder = MethodBuilder::new(&mut method);              

        method_builder.set_name(match_results.get(0).unwrap().as_ref().unwrap());
        method_builder.set_args(match_results.get(1).unwrap().as_ref().unwrap());

        Ok(method)
    }    
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
struct ScopeMemory {
    variables: Vec<Variable>,
    methods: Vec<Method>,
    code: String
}

#[allow(dead_code)]
impl ScopeMemory {
    pub fn new<'c>() -> Self {
        Self { 
            variables: Vec::new(), 
            methods: Vec::new(), 
            code: String::new()
        }
    }

    fn parse_methods(&mut self) -> Result <(), Vec<String>> {
        let mut parse_errors = Vec::<String>::new();

        let method_pattern = r"(?s)(method.*?end)";                                
        let re = Regex::new(method_pattern).unwrap();
        
        for captures in re.captures_iter(self.code.as_str()) {
            if captures.get(0).is_none() { continue; }

            let matched = captures.get(1).unwrap();
            let parser = MethodParser::new(matched.as_str());
            let parse_result = parser.parse();
            
            if let Ok(method) = parse_result.as_ref() {
                self.methods.push(method.to_owned());
            }

            if let Err(err) = parse_result {
                parse_errors.extend(err);
            }
        }

        self.code = re.replace_all(&self.code, "").to_string();
        Ok(())
    }

    fn parse_variables(&mut self) {

    }

    // test methods
    pub fn set_code(&mut self, code: &str) {
        self.code = code.to_string();
    }

    // end of test methods
    pub fn parse(&mut self) {
        let _ = self.parse_methods();        
        for method in self.methods.iter() {
            println!("{:#?}", method);
        }
    }
}

fn try_read_scripting_file(filename: &str) -> Result<String, String> {
    let file = File::open(filename);    
    let mut contents = String::new();

    let file_ext = filename.split(".")
        .map(|s| s.to_string())
        .collect::<Vec<String>>();        

    if file_ext.len() == 1 {
        return Err(String::from("This file does not have a valid extension"));
    }
    
    if let Some(ext) = file_ext.last() {
        if ext.as_str() != "sl" {
            return Err(String::from("Invalid file extension"));
        }
    }        

    if let Err(_) = file { return Err(String::from("Error finding scripting file")); }
        
    file.unwrap().read_to_string(&mut contents).unwrap();
    Ok(contents)
}

fn execute_script(_file_contents: String) {
    let mut memory = ScopeMemory::new();
    memory.set_code(_file_contents.as_str());
    memory.parse();    
}

fn main() {
    let args = env::args().collect::<Vec<String>>();
    if let Some(filename) = args.get(1) {
        let file_contents = try_read_scripting_file(filename.as_str());
        if let Err(error) = file_contents {
            eprintln!("[-] Invalid file: {}", error);
            process::exit(0);
        }

        execute_script(file_contents.unwrap());
    }
}
