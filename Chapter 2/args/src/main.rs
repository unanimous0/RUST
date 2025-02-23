// 명령줄 인수 취합
fn main() {
    args_ex1();
    args_ex2();
    args_ex3();
    args_ex4();
    args_ex5();
    args_ex6();
}

fn args_ex1() {
    println!();
    println!("Args 예제 1");

    // 명령줄 인수 획득
    let args = std::env::args();
    
    let mut total = 0.0;

    // 명령줄 인수를 순서대로 연산
    for (i, s) in args.enumerate() {
        // 0번째 명령어(프로그램) 자신이므로 무시
        if i==0 { continue; }
        
        // 명령줄 인수를 숫자로 변환
        let num: f64 = match s.parse() {
            Ok(v) => v,
            Err(_) => 0.0,
        };

        total += num;
    }

    // 결과 표시
    println!("{}", total);
}

fn args_ex2() {
    println!();
    println!("Args 예제 2");

    use std::env;

    let args = env::args();
    for arg in args {
        println!("{}", arg);
    }
}

fn args_ex3() {
    println!();
    println!("Args 예제 3");

    use std::env;

    // 명령줄 인수를 Vec<String>으로 취득
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
}

fn args_ex4() {
    println!();
    println!("Args 예제 4");

    use std::env;   // 명령줄 인수 취득용
    use std::fs;    // 파일 읽기용

    // 인수를 벡터로 취득
    let args: Vec<String> = env::args().collect();

    // 인수를 지정했는지 확인
    if args.len() < 2 {
        println!("읽어올 파일을 지정해주세요.");
        return;
    }

    // 두 번째 요소
    let filename = &args[1];

    /*
        &args[1];

        여기서 env::args()는 명령줄 인수를 받아들이고, 이 인수들을 Vec<String> (문자열 벡터)로 수집하고,
        이때 args는 힙(Heap) 메모리에 저장된 String 타입의 벡터임.

        args[1]은 String 타입의 요소를 가져오고,
        &args[1]은 args[1]이라는 String 타입의 요소에 대한 "참조"를 생성함.
        따라서 filename의 타입은 &String이 된다.

        ❓ 그럼 왜 참조를 사용할까?
            1.	메모리 복사 비용 절약:
            •	args[1]이 String이므로 직접 할당하면 메모리 복사가 발생해요.
            •	&args[1]을 사용하면 참조만 넘겨주므로 메모리 사용이 효율적이에요.
            
            2.	소유권(Ownership) 문제 방지:
            •	러스트에서는 소유권을 넘겨주면 원본 데이터를 사용할 수 없어요.
            •	&args[1]을 사용하면 소유권을 빌리는(borrowing) 형태이므로 원본 벡터(args)를 계속 사용할 수 있어요.

            3.	불필요한 힙 메모리 할당 방지:
            •	args[1]을 String으로 복사하지 않기 때문에 힙 메모리에 새로운 공간을 할당할 필요가 없어요.
            •	filename에 args[1]으로 대입하면, String으로 복사한 것이 되므로 힙 메모리에 추가적으로 새로운 공간이 할당됨.
     */

    // 파일을 읽어와 출력
    let text = fs::read_to_string(filename).unwrap();
    println!("{}", text);
}

fn args_ex5() {
    println!();
    println!("Args 예제 5");

    use std::env;   // 명령줄 인수 취득용
    use std::fs;    // 파일 읽기용

    // 인수를 벡터로 취득
    let args: Vec<String> = env::args().collect();

    // 인수를 지정했는지 확인
    if args.len() < 2 {
        println!("읽어올 파일을 지정해주세요.");
        return;
    }

    // 두 번째 요소
    let filename = &args[1];

    // 파일을 읽어와 출력
    let text = match fs::read_to_string(filename) {
        Ok(v) => v,
        Err(e) => e.to_string(),
        // Err(e) => e.as_str(),            // 이렇게 변환하면 struct 'std::io::Error' 안에 'as_str'이라는 함수가 없다고 나오기도 하고,
    };                                      // 에러 e를 String 타입으로 변환하는 이유는 Ok일 때의 값이 String 타입이기 때문.
                                            // 변수에 match를 이용해 값을 할당할 경우, Ok일 때와 Err일 때의 타입을 일치시켜야한다.
                                            
    println!("{}", text);
}

fn args_ex6() {
    println!();
    println!("Args 예제 6");

    use std::{env, fs};

    // 명령줄 인수 취득
    let args = env::args();
    
    let mut total: f64 = 0.0;

    // 모든 인수 처리
    for (i, fname) in args.enumerate() {
        if i == 0 { continue; }

        // 텍스트 파일을 읽어들임
        let text = fs::read_to_string(fname).unwrap();

        // 한 줄씩 분리
        let lines = text.split('\n');

        // 반복해서 계산
        for line in lines {
            // 숫자로 변경
            let n: f64 = match line.parse() {
                Ok(v) => v,
                Err(_) => 0.0,
            };

            total += n;
        }
    }

    println!("{}", total);
}