// * Result 타입을 반환하는 parse 메서드 다루기
pub fn parse_method() {
    // 문자열에 숫자 값을 대입
    let s = "365";

    // i32 타입 숫자 값으로 반환
    let i: i32 = match s.parse() {
        Ok(v) => v,
        Err(_) => 0,
    };

    // f64 타입 숫자 값으로 변환
    let j: f64 = match s.parse() {
        Ok(v) => v, 
        Err(_) => 0.0,
    };

    println!("{}", i);
    println!("{}", j);
    print_type_of(i);
    print_type_of(j);
}

// 타입 확인
fn print_type_of<T> (_: T) {
    println!("입력된 변수의 타입은 {} 입니다.", std::any::type_name::<T>());
}

// pub fn parse_method2() {
//     let s = "365";
//     let i: i32 = match s.unwrap();
//     println!("{}", i);
// }