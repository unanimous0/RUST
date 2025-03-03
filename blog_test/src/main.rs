fn main() {
    let mut v = 10;

    set_value(&mut v);

    println!("v = {}", v);
}

// 인수의 값을 100으로 변경하는 함수
fn set_value(arg: &mut i32) {
    *arg = 100;
}

/*
    실행 결과
    v = 100
*/