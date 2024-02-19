/// 할 일 구조체
#[derive(Debug)]
pub struct Todo {
    /// 할 일 내용
    pub todo: String,
    /// 완료 여부
    status: bool,
}

/// 할 일 구현체
impl Todo {
    /// 할 일 생성
    #[must_use]
    pub fn new(todo: String, status: bool) -> Todo {
        Todo { todo, status }
    }
    /// 완료 여부를 문자열로 반환
    #[must_use]
    pub fn is_done(&self) -> &str {
        if self.status {
            "x"
        } else {
            " "
        }
    }
}

fn main() {
    println!("Hello, world!");
}
