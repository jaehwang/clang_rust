use clang::{Clang, Index};

// entity를 traverse하는 함수
fn traverse_entity(entity: &clang::Entity) {
    // 현재 entity 출력
    if entity.get_kind() == clang::EntityKind::FunctionDecl || entity.get_kind() == clang::EntityKind::CallExpr {
        // println!("{:?}", entity.get_kind());
        // println!("{:?}", entity.get_location());
        
        if entity.get_kind() == clang::EntityKind::FunctionDecl {
            if let (Some(name), Some(location)) = (entity.get_name(), entity.get_location()) {
                if let Some(file) = location.get_file_location().file {
                    println!("Caller: {:?} {:?}", name, file.get_path());
                }
            }
        } else if entity.get_kind() == clang::EntityKind::CallExpr {
            if let Some(function) = entity.get_reference() {
                if let Some(name) = function.get_name() {
                    println!("  Callee: {:?}", name);
                }
            }
        } else {
            //pass
        }
    }

    // 자식 entity들을 traverse
    for child in entity.get_children() {
        traverse_entity(&child);
    }
}

fn main() {
    // Clang 인스턴스 생성
    let clang = Clang::new().unwrap();
    let index = Index::new(&clang, false, false);

    // main.c 파일 파싱
    let tu = index.parser("main.c")
        .arguments(&["-I", "/path/to/include", "-D", "DEBUG"])
        .parse()
        .unwrap();

    traverse_entity(&tu.get_entity());
}
