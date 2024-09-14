use clang::{Clang, Index};
use serde::Deserialize;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;

// 호출 관계를 저장할 자료 구조
type CallGraph = HashMap<String, Vec<String>>;

#[derive(Deserialize)]
struct CompileCommand {
    // directory: String,
    command: String,
    file: String,
}

// compile_commands.json 파일을 파싱하는 함수
fn parse_compile_commands(path: &str) -> Vec<CompileCommand> {
    let file = File::open(path).expect("Failed to open compile_commands.json");
    let reader = BufReader::new(file);
    serde_json::from_reader(reader).expect("Failed to parse compile_commands.json")
}

// 필요한 인자만 필터링하는 함수
fn filter_arguments(command: &str) -> Vec<String> {
    let mut args = Vec::new();
    let mut iter = command.split_whitespace().peekable();

    while let Some(arg) = iter.next() {
        if arg.starts_with("-I") || arg.starts_with("-D") || arg.starts_with("-isysroot") || arg.starts_with("-isystem") {
            if arg.len() > 2 && !arg.starts_with("-isysroot") && !arg.starts_with("-isystem") {
                args.push(arg.to_string());
            } else {
                args.push(arg.to_string());
                if let Some(path) = iter.peek() {
                    args.push(path.to_string());
                    iter.next(); // 경로를 소비
                }
            }
        }
    }

    args
}

// entity를 traverse하는 함수
fn traverse_entity(entity: &clang::Entity, call_graph: &mut CallGraph, current_function: Option<&str>) {
    if entity.get_kind() == clang::EntityKind::FunctionDecl {
        if let Some(name) = entity.get_name() {

            let filepath = entity.get_location().unwrap().get_file_location().file;
            let filepath_str = filepath.map_or("".to_string(), |f| f.get_path().to_str().unwrap().to_string());
            let lineno = entity.get_location().unwrap().get_file_location().line;

            // 새로운 함수 선언을 발견하면 현재 함수를 업데이트
            let name_str = name.to_string() + " (" + &filepath_str + ":" + &lineno.to_string() + ")";

            call_graph.entry(name_str.clone()).or_default();
            for child in entity.get_children() {
                traverse_entity(&child, call_graph, Some(&name_str));
            }
        }
    } else if entity.get_kind() == clang::EntityKind::CallExpr {
        if let Some(function) = entity.get_reference() {
            if let Some(callee_name) = function.get_name() {
                if let Some(caller_name) = current_function {
                    call_graph.entry(caller_name.to_string()).or_default().push(callee_name.to_string());
                }
            }
        }
    } else {
        // 다른 종류의 entity는 무시
        for child in entity.get_children() {
            traverse_entity(&child, call_graph, current_function);
        }
    }
}

// 호출 그래프를 출력하는 함수
fn print_call_graph(call_graph: &CallGraph) {
    for (caller, callees) in call_graph {
        for callee in callees {
            println!("{} -> {}", caller, callee);
        }
    }
}

fn main() {
    // compile_commands.json 파일 파싱
    let compile_commands = parse_compile_commands("compile_commands.json");

    // Clang 인스턴스 생성
    let clang = Clang::new().unwrap();
    let index = Index::new(&clang, false, false);

     // 전체 호출 그래프
     let mut call_graph: CallGraph = HashMap::new();

     // 각 파일에 대해 호출 그래프 생성
     for command in compile_commands {
         let arguments = filter_arguments(&command.command);
         let tu = index.parser(&command.file)
             .arguments(&arguments)
             .parse()
             .expect("Failed to parse file");
 
         traverse_entity(&tu.get_entity(), &mut call_graph, None);
     }

    // 호출 그래프 출력
    print_call_graph(&call_graph);
}