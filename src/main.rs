use clang::{Clang, Index};

// entity를 traverse하는 함수
fn traverse_entity(entity: &clang::Entity) {
    // 현재 entity 출력
    println!("{:?}", entity.get_kind());
    println!("{:?}", entity.get_location());

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
    let tu = index.parser("main.c").parse().unwrap();

    traverse_entity(&tu.get_entity());
}
