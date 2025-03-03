pub fn ownership_check1() {
    println!();
    println!("소유권 예제 1");

    // 블록 1
    {
        let s1 = String::from("hello");
        let s3 = String::from("world");

        // 블록 2
        {
            let s2 = s1;
            println!("{}", s2);
        }   // s2의 값은 여기서 파기

        println!("{}", s3);
    }   // s3의 값은 여기서 파기
}


pub fn ownership_check2() {
    println!();
    println!("소유권 예제 2");

    // 소유권 시스템을 적용받는 타입
    let g1 = String::from("Hello");
    let g2 = g1;
    // println!("{}", g1);     // 오류 발생
    println!("g1을 출력하려 하면 오류 발생. 반면 g2는 \"{}\"의 값으로 이상없이 출력 됨", g2);
}


pub fn ownership_check3() {
    println!();
    println!("소유권 예제 3");

    // 소유권 시스템을 적용받지 않는 타입
    let g1 = "Hello";
    let g2 = g1;
    println!("{}", g1);     // 이상 없음
    println!("{}", g2);
}


pub fn ownership_check4() {
    println!();
    println!("소유권 예제 4");

    // 소유권이 이동하는 타입(문자열과 구조체 등)이라도 데이터를 복제(clone)해서 소유권 이동을 피할 수 있음
    let g1 = String::from("Hello");
    let g2 = g1.clone();

    println!("g2로 소유권 이동된 것이 아니라 복제된 것이므로 g1은 사용 가능 : {}", g1);     // 이상 없음
    println!("g2는 g1으로부터 복제되어 사용 가능 : {}", g2);    
}