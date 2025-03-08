// CarSpec 구조체 정의
struct CarSpec {
    model: i32,
    cc: i32,
    color: i32,
}

// 키와 몸무게 데이터를 가지는 구조체
struct Body {
    weight: f64,
    height: f64,
}

// bmi 판정용 구조체
struct BmiRange {
    min: f64,   // min 이상
    max: f64,   // max 미만
    label: &'static str,    // 판정
}


pub fn struct_example1() {
    println!();
    println!("구조체 예제 1");

    // CarSpec 객체 생성
    let car1 = CarSpec {
        model: 3001,
        cc: 1500,
        color: 0xFF0000,
    };

    let car2 = CarSpec {
        model: 3002,
        cc: 1200,
        color: 0x0000FF,
    };

    // 구조체 객체의 각 필드를 출력
    println!("car 1 : {}, {}cc, {:06x}", car1.model, car1.cc, car1.color);
    println!("car 2 : {}, {}cc, {:06x}", car2.model, car2.cc, car2.color);
}

pub fn struct_example2() {
    println!();
    println!("구조체 예제 2");

    // 구조체 초기화 -> 구조체를 초기화할 때는 모든 필드의 값을 지정된 타입으로 대입해야함 (값이 빠진 필드가 있는 경우 에러가 발생함)
    let koh = Body {
        weight: 80.0,
        height: 180.0,
    };

    let cho = Body {
        weight: 90.0,
        height: 190.0,
    };

    // bmi 출력
    println!("고 = {:.1}", calc_bmi(&koh));
    println!("조 = {:.1}", calc_bmi(&cho));
}

fn calc_bmi(body: &Body) -> f64 {
    let h = body.height / 100.0;
    body.weight / h.powf(2.0)
}

pub fn struct_example3() {
    println!();
    println!("구조체 예제 3");

    // 키와 몸무게 입력
    let height_cm = input("키(cm) : ");
    let weight = input("몸무게(kg) : ");

    // bmi 계산
    let height = height_cm / 100.0;
    let bmi = weight / height.powf(2.0);

    // 비만도 판정표를 벡터 타입으로 생성
    let bmi_list = vec![
        BmiRange {
            min: 0.0,
            max: 18.5,
            label: "저체중",
        },

        BmiRange {
            min: 18.5,
            max: 23.0,
            label: "정상",
        },

        BmiRange {
            min: 23.0,
            max: 25.0,
            label: "비만전단계",
        },

        BmiRange {
            min: 25.0,
            max: 30.0,
            label: "1단계 비만",
        },

        BmiRange {
            min: 30.0,
            max: 35.0,
            label: "2단계 비만",
        },

        BmiRange {
            min: 35.0,
            max: 99.0,
            label: "돼지",
        },
    ];

    // 비만도 판단
    let mut result = "계산 불가";
    for range in bmi_list {
        if range.min <= bmi && bmi < range.max {
            result = range.label;
            break;
        }
    }

    // 결과 표시
    println!("BMI = {:.1}, 비만도 = {}", bmi, result);
}

// 한 줄씩 읽어 f64 타입으로 반환
fn input(prompt: &str) -> f64 {
    println!("{}", prompt);
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).expect("입력 에러");
    s.trim().parse().expect("숫자 변환 에러")
}


pub fn struct_example4() {
    println!();
    println!("구조체 예제 4");
}


pub fn struct_example5() {
    println!();
    println!("구조체 예제 5");
}
