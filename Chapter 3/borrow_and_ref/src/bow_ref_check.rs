pub fn bow_ref_check1() {
    println!();
    println!("빌림과 참조 예제 1");

    // 함수 호출로 이동하는 소유권
    let g1 = String::from("ex1");
    show_message1(g1);        // 소유권이 이동함
    // println!("{}", g1);      // 에러 발생 : g1은 사용할 수 없음
}

fn show_message1(message: String) {
    println!("{}", message);
}


pub fn bow_ref_check2() {
    println!();
    println!("빌림과 참조 예제 2");

    let mut g1 = String::from("ex2");
    g1 = show_message2(g1); // 위에서 mut를 붙여 가변 변수로 선언한 이유 : 여기서 대입이 발생하므로 (값 내용에 변경이 없어도 대입은 초기화와는 다른 동작이므로 값이 변하는 것으로 간주)
    println!("{}", g1);     // 정상 실행
}

fn show_message2(message: String) -> String {
    println!("{}", message);
    return message;
}


pub fn bow_ref_check3() {
    println!();
    println!("빌림과 참조 예제 3");

    let mut g1 = String::from("ex3");
    show_message3(&mut g1);
    println!("Second : {}", g1);
}

fn show_message3(message: &mut String) {
    *message = String::from("Hello World");     // 가변참조변수 message로부터 실제 값을 얻기 위해 역참조(*)를 사용하고, 실제 값을 새로운 값으로 변경
    println!("First : {}", message);
}


pub fn bow_ref_check4() {
    println!();
    println!("빌림과 참조 예제 4");

    /*
        참조를 반환할 때 해당 값이 파기되어 반환해도 참조할 값이 사라져 에러가 발생하는 경우        
     */

    // let m = show_message4();
    // println!("{}", m);
}

// fn show_message4() -> &String{
//     let tmp = String::from("HHH");       // tmp의 수명은 이 블럭까지이므로
//     return &tmp;                         // 여기까지는 에러가 발생하지 않아도, 블럭이 끝나는 순간 tmp는 dropped 되었으므로 에러가 발생함
// }


// 역참조를 통한 인수 변경77
pub fn bow_ref_check5() {
    let mut x = 1024;
    x2(&mut x);
    println!("{}", x);
}

fn x2(x: &mut i32) {
    *x = *x * *x;
}