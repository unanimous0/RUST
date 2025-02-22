// HashMap 사용을 위한 선언
use std::collections::HashMap;

// 투표 데이터를 상수로 선언
const V_DATA: &str = "C,C,A,A,A,B,C,C,B,B,B,C,B,C,B,A,C,C,B,C,C,C";

fn main() {
    hashmap_ex1();
    hashmap_ex2();
    hashmap_ex3();
    hashmap_ex4();
}

// 인기 투표 집계
fn hashmap_ex1() {
    println!();
    println!("HashMap 예제 1");

    // 집계용 HashMap 생성
    // let mut c_map = HashMap::new();
    let mut c_map: HashMap<&str, i32> = HashMap::new();

    // HashMap을 0으로 초기화
    c_map.insert("A", 0);
    c_map.insert("B", 0);
    c_map.insert("C", 0);

    // 투표 데이터 집계
    for w in V_DATA.split(',') {
        c_map.insert(w, c_map[w]+1);
    }

    // 집계 후 결과 표시
    for k in ["A", "B", "C"] {
        println!("{}: {:>2}", k, c_map[k]);
    }
}

fn hashmap_ex2() {
    println!();
    println!("HashMap 예제 2");

    // 우리말 월 이름 목록
    let months = ["해오름달", "시샘달", "꽃내음달", "잎새달", "푸른달", "누리달", 
                    "빗방울달", "타오름달", "거둠달", "온누리달", "눈마중달", "매듭달"];

    // HashMap 초기화
    let mut months_map: HashMap<&str, usize> = HashMap::new();

    // 월의 이름을 HashMap에 추가
    for (i, v) in months.iter().enumerate() {
        months_map.insert(v, i+1);
    }

    // 요소 표시
    for month in months {
        println!("{:5} : {:3}월", month, months_map[month]);
    }
}

fn hashmap_ex3() {
    println!();
    println!("HashMap 예제 3");

    // HashMap을 생성해 초기화
    let mut map: HashMap<&str, usize> = HashMap::new();
    map.insert("A", 30);
    map.insert("B", 50);

    // 키가 존재 여부 확인
    if map.get("D") == None {
        println!("D는 존재하지 않음");
    } else {
        println!("D = {}", map["D"]);
    }
}

fn hashmap_ex4() {
    println!();
    println!("HashMap 예제 4");

    // HashMap을 생성해 초기화
    let mut map: HashMap<&str, usize> = HashMap::new();
    map.insert("A", 30);
    map.insert("B", 50);

    let values = ["A", "B", "C", "D"];

    // 키가 존재 여부 확인
    for v in values {
        print!("{} = ", v);
        match map.get(v) {
            Some(v) => println!("{}", v),
            None => println!("D는 존재하지 않음"),
        };
    }
}