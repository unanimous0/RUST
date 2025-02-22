pub fn str_parse() {
    // 문자열 지정
    let s = "3.1415";

    // f64 타입으로 변환
    let num = s.parse::<f64>().expect("변환 실패");

    // 변환한 값을 서식에 맞춰 출력
    println!("{:.2}", num);
}

pub fn str_parse2() {
    let s = "3.1415a";
    let num = s.parse::<f64>();
    match num {
        Ok(result) => println!("{:.2}", result),
        Err(e) => println!("에러가 발생했습니다. 이유 : '{:?}'", e)
    }
}