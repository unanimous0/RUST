pub fn string_example1() {
    println!();
    println!("문자열 예제 1");

    let s = "안녕하세요";
    let ch = s.chars().nth(0).unwrap();
    let ch2 = s.chars().nth(1).unwrap();
    println!("{}", ch);
    println!("{}", ch2);

    let binr = &s[0..3];    // &str을 슬라이스해 취득한 데이터가 문자열로 출력됨
    println!("{}", binr);   // 이때 바이트 단위를 잘 지정해야함. 올바르지 않게 지정한 경우 컴파일 단계에서 에러 발생 ("byte index n is not a char boundary.")
}


pub fn string_example2() {
    println!();
    println!("문자열 예제 2");

    // 문자열 리터럴은 &str 타입
    let ss: &str = "안녕하세요.";

    // &str -> String
    let so1: String = String::from(ss);
    let so2: String = ss.to_string();

    // String -> &str
    let ss2: &str = &so1;                   // so1은 String이고 &s01은 &String인데, 러스트에서는 &String -> &str로 자동 변환이 일어남
    let ss3: &str = so1.as_str();

    // String -> &str    
    let ss4: &str = &so2;
    let ss5: &str = so2.as_str();

    // 출력
    println!("{}\n{}\n{}\n{}\n{}\n{}\n", so1, so2, ss2, ss3, ss4, ss5);

    // 참조 타입 포인터 주소를 표시
    println!("{:p}\n", ss);                 // .rodata 섹션 (정적 메모리)
    println!("{:p}\n{:p}\n", ss2, ss3);     // Heap 메모리 (String 내부의 데이터)
    println!("{:p}\n{:p}\n", ss4, ss5);     // Heap 메모리 (String 내부의 데이터)

    // println!("{:p}", so1);               
    
    /*
        에러 발생 -> so1은 String 타입이라서 직접 포인터 주소를 출력할 수 없음 (&so1 또는 so1.as_str() 등 참조 형식으로 바꿔줘야함)
        "{:p}" 형식은 *const T 또는 &T 같은 참조자만 허용
        Rust에서 "{:p}" 포맷 문자열을 사용하면 메모리 주소를 출력할 수 있어.
        하지만, "{:p}"는 참조 타입(&T, *const T)만 지원하지, 소유 타입(String)은 지원하지 않아.

        ! 중요
        so1: String은 힙(Heap)에 데이터가 저장되는 구조체이지만, so1 자체는 스택(stack)에 메타데이터(포인터, 길이, 용량)가 저장된 변수야.
        so1 자체는 스택에 저장된 구조체(String)의 메타데이터야.
        따라서 &so1을 사용하면 스택(stack)에서 so1이 위치한 주소를 출력할 수 있어.

        즉, so1이라는 변수 자체는 String 타입이지만 이 so1자체는 스택에 메타데이터를 저장한 변수임
        그 문자열 데이터 값 자체는 힙에 저장되어있지만, so1 자체는 스택에 저장된 메타데이터 변수라는 뜻임
        그래서 이 메타데이터에 대한 정보를 사용하려고 &를 쓰는 것이고.
     */
    

    /*
        [ss] & [ss2, ss3] & [ss4, ss5]의 포인터 주소가 세 가지로 다 다름

        ✔ String::from(ss)과 ss.to_string()은 새로운 힙 메모리를 할당하고 데이터를 복사하므로 새로운 메모리 주소를 가짐
        ✔ &so1과 so1.as_str()은 같은 so1의 내부 데이터를 가리키므로 같은 주소
        ✔ &so2와 so2.as_str()도 같은 so2의 내부 데이터를 가리키므로 같은 주소
        ✔ 하지만 so1과 so2는 서로 다른 String 객체이므로 ss2/ss3와 ss4/ss5는 서로 다른 주소!

        🔹 왜 ss의 주소가 so1, so2와 다를까?
	    1.	ss는 문자열 리터럴(&str)을 가리킴
	    •	"안녕하세요." 같은 문자열 리터럴은 프로그램 실행 중 변하지 않는 불변 데이터이므로 **바이너리의 데이터 영역(Static Memory, .rodata 섹션)**에 저장됨.
	    •	즉, ss는 실행 파일이 실행될 때 이미 특정한 메모리 주소에 고정되어 있음.
        2.	so1, so2는 힙(Heap)에 새로운 메모리를 할당
        •	String::from(ss) 또는 ss.to_string()을 호출하면, "안녕하세요."의 내용을 새로운 힙 메모리 공간에 복사하여 저장.
        •	따라서 so1과 so2는 각각 힙(Heap)에서 새로운 메모리 블록을 가리킴, 즉 ss가 참조하는 원본 문자열과는 다른 주소를 가짐.
     */

    /*
        Q. 그럼 so1 이라는 변수는 스택에 메타데이터를 저장하고 있는 변수이고, 실제 문자열 데이터 자체는 힙에 저장된거잖아.
            그래서 메타데이터 정보를 사용하려면 &so1을 쓰는게 맞지? 그러면 so1 자체를 출력할때 메타데이터가 아니라 실제 문자열 데이터가 출력되는건 어떤 원리야?

        A. chatgpt 답변 참고 (String의 Display 트레이트 구현 방식 참고)
        * so1 자체는 스택에 있는 메타데이터(포인터, 길이, 용량)지만,
        * Rust의 Display 트레이트가 so1을 &str로 변환해서 힙 메모리의 실제 문자열 데이터를 출력하는 원리야.
        * 그래서 println!("{}", so1);을 실행하면 힙 메모리에 있는 문자열 값이 출력되는 것처럼 보이는 거야! 
     */
}


pub fn string_example3() {
    println!();
    println!("문자열 예제 3");

    // 문자열을 1바이트씩 출력
    let pr = "구술이 서 말이라도 꿰어야 보배";
    for c in pr.bytes() {       // bytes 메서드는 반복자를 반환하므로 for문을 이용할 수 있음
        print!("{:2x} ", c);
    }

    // len() 메서드로 바이트 수 구하기
    println!("\n바이트 = {}Bytes", pr.len());   // 러스트에서 실수하기 쉬운 것 중 하나는 &str의 len 메서드
                                              // &str의 len 메서드는 문자 길이가 아니라 바이트 길이를 반환한다.
                                              // 'abc'와 같이 영어 문자열에 len 메서드를 이용하면 문자당 1바이트이므로 3을 반환하지만, 
                                              // '맛있다' 라는 한글에 len 메서드를 이용하면 문자당 3바이트이므로 9를 반환한다.
}


pub fn string_example4() {
    println!();
    println!("문자열 예제 4");

    // 문자열을 1자씩 출력
    let pr = "구슬이 서 말이라도 꿰어야 보배";

    // 1자씩 표시
    for c in pr.chars() {
        print!("[{}]", c);
    }

    // 글자 수 세기
    println!("\n글자 수 = {}자", pr.chars().count());   // chars() 메서드는 반복자를 반환하므로 count 메서드를 이용하면 요소의 수를 셀 수 있음



    // Vec<char>로 변환해서 처리
    let pr_chars: Vec<char> = pr.chars().collect();     // chars 메서드를 이용해 Vec<char> 타입으로 변환한 뒤 문자열을 처리함
    println!("\nVec<chars> : {:?}", pr_chars);
    for c in pr_chars.iter() {
        print!("({})", c);
    }

    // 글자 수 세기
    print!("\n글자 수 = {}자", pr_chars.len());     // 변수 pr_chars는 이미 글자별로 분리된 벡터 타입이므로 len 메서드를 이용하면 바이트 수가 아니라 벡터의 길이, 즉 글자 수를 반환한다.

    /*
        u8이 기호 없는 8비트 정수(1바이트)로 문자를 표현하는 반면, char 타입은 32비트(4바이트)로 1문자를 표시한다.
        이 부분에서도 러스트의 언어적 성격을 엿볼 수 있다.
        처음부터 Vec<char>를 문자열 내부 표현으로 하는 것이 아니라, Vec<u8>을 문자열로 표현한다.
        그리고 필요에 따라 Vec<char>로 변환해 처리하는 것이다.

        이게 무슨 말이냐면, pr 변수 자체는 원래 Vec<u8> 타입인데, 이걸 한 문자씩 출력할 수 없어. 그러니까 한 문자씩 출력하려고(=필요에 따라) Vec<char>로 변환해서 처리한 것
        그래서 pr은 Vec<u8>이고, pr_chars는 Vec<u8>이 Vec<char>로 변환된거야. 물론 용량은 pr_chars가 더 크겠지. 1바이트(u8) vs 4바이트(char) 이니까.
     */
}



// &'static str을 이용하는 함수
fn echo(s: &'static str) {
    println!("{}", s);
}

pub fn string_example5() {
    println!();
    println!("문자열 예제 4");

    // 문자열 리터럴 (&'static str)을 지정
    echo("웅변은 은이요");
    echo("침묵은 금이다");

    // 아래 주석은 에러가 발생함
    // let s = String::from("테스트");
    // echo(&s);
    
    /*
        &s는 라이프타임이 'static str 보다 짧음. 주기가 서로 다른 타입을 지정했으니 에러가 발생한 것.
        그래서 p.197처럼 인수 타입을 &String으로 바꿔주면 에러가 사라진다.

        let s = String::from("테스트");
        echo(&s);

        fn echo(s: &str) {
            println!("{}", s);
        }

        로 수정하면 에러가 없을 것.

        Q. 이때 &s는 &String 타입이고, echo 메서드의 인자 타입은 &str로 다른 것 아닌가 할 수 있는데,
        
        A. Rust에서는 &String 타입을 &str 타입으로 자동으로 변환(deref coercion) 해주기 때문에 코드가 정상적으로 실행됨

        1. &String과 &str의 관계
	    •	String 타입은 내부적으로 str(string slice)을 소유하는 구조체
	    •	따라서 &String은 필요에 따라 &str로 자동 변환될 수 있음

        즉, &s는 &String이지만, echo 함수가 &str을 요구하므로 Rust가 자동으로 &s를 &str로 변환되며, 결과적으로 echo(s.as_str())와 동일하게 동작함

        단, &String을 필요로 하는 함수에 &str을 넘길 때는 자동 변환이 안됨. (String이 str을 소유하는 구조체이므로 반대 방향의 변환이 불가능함)
        •	&String -> &str 변환은 가능하지만,
        •	&str -> &String 변환은 자동으로 되지 않음            
     */
}





