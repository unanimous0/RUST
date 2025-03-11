fn main() {
    let mut height;

    // 반복문
    loop {
        println!("키(cm) : ");
        height = input_fault(0.0);  // 숫자 입력
        
        if height > 0.0 { break; }  // 숫자면 break
        println!("숫자만 입력 가능합니다.");  // 숫자 아니면 출력
    }

    // 표준 체중 계산
    let weight = 22.0 * (height / 100.0).powf(2.0);
    println!("표준 체중은 {:.1}kg 입니다.", weight);
}

// 표준 입력에서 문자열 얻기
fn input_str() -> String {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).expect("입력 에러");
    return s.trim_end().to_string();
}

// 표준 입력에서 실패 시 def 반환
fn input_fault(def: f64) -> f64 {
    let s = input_str();
    match s.trim().parse() {
        Ok(v) => v,
        Err(_) => def,
    }
}