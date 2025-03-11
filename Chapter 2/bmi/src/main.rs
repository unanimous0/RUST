pub mod str_parse_f;

// 비만도 진단 도구
fn main() {
    // 키와 몸무게 입력
    let height_cm = input("키(cm) : ");
    let weight = input("몸무게(kg) : ");
    
    // BMI 계산
    let height = height_cm / 100.0;
    let bmi = weight / height.powf(2.0);
    println!("BMI={:.1}", bmi);

    // 비만도 진단
    if bmi < 18.5 { println!("저체중"); }
    else if bmi < 23.0 { println!("정상"); }
    else if bmi < 25.0 { println!("비만 전단계"); }
    else if bmi < 30.0 { println!("비만 1단계"); }
    else if bmi < 35.0 { println!("비만 2단계"); }
    else { println!("비만 3단계"); }

    println!("");
    str_parse_f::str_parse();

    println!("");
    str_parse_f::str_parse2();
}


// 표준 입력에서 1줄씩 읽어 f64 타입으로 반환하는 함수
fn input(prompt: &str) -> f64 {
    // 메세지 출력
    println!("{}", prompt);

    // 입력 값 가져오기
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).expect("입력 에러");

    // 공백을 제거하고 숫자로 변환
    return s.trim().parse().expect("숫자가 아님");
}