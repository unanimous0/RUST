fn main() {
    // 파일 이름 지정
    dictionary_ex1();
    dictionary_ex2();
    dictionary_ex3();
}


fn dictionary_ex1() {
    println!();
    println!("Dict 예제 1");
    
    // 파일 조작용 라이브러리 이용 선언
    use std::fs;

    let afile = "./fizzbuzz_python.txt";
    let bfile = "./fizzbuzz_rust.txt";

    // 파일 내용을 문자열로 읽어들임
    let astr = fs::read_to_string(afile).unwrap();
    let bstr = fs::read_to_string(bfile).unwrap();

    // 불필요한 공백 삭제
    let astr = astr.trim();
    let bstr = bstr.trim();

    /*
        Q. 위에서 let astr을 다시 astr.trim()으로 덮어쓰고 있는데, 그럼 mut가 있어야 하는 것 아니냐.

        A. Rust에서는 “shadowing”이라는 개념이 있어요.
        즉, let astr = astr.trim(); 구문은 기존의 astr 값을 변경하는 것이 아니라, 새로운 변수(같은 이름의 변수)를 새로 선언하는 것입니다.
        따라서, 원래의 astr나 bstr을 변경하는 것이 아니라, 해당 변수 이름으로 새로운 바인딩을 생성하는 것이므로 처음부터 mut를 붙일 필요가 없어요.

        shadowing과 참조(&)는 서로 다른 개념이에요.
	    Shadowing: Rust에서는 동일한 변수 이름을 다시 선언하여 새로운 값(혹은 타입의 값)을 바인딩할 수 있어요.
        이 경우, 원래의 변수는 수정되지 않고 새로운 값으로 대체(재정의)되는 것이기 때문에 mut가 필요 없어요.

	    참조(&)와 trim():
        trim() 메서드는 문자열의 공백을 제거한 **슬라이스(&str)**를 반환해요. 이 슬라이스는 원래 문자열의 일부를 참조하지만, 이는 shadowing과는 별개의 문제에요.
        결론적으로, shadowing은 변수 재선언을 통한 새로운 바인딩 생성 방법이며, 참조값 반환과는 관련이 없어서 mut 없이도 가능해요.

        위 코드에서 첫 번째 astr는 String 타입이고, astr.trim()은 공백이 제거된 문자열 슬라이스, 즉 &str 타입을 반환해요.
        그 후 같은 이름인 astr로 새롭게 바인딩(Shadowing)되기 때문에, 이 새로운 astr는 이제 &str 타입이 돼요.
        따라서 이후에는 이 astr 변수로 &str에 정의된 메서드만 사용할 수 있고, 원래의 String 메서드를 사용할 수 없게 돼요. 만약 두 타입의 기능을 모두 사용하고 싶다면, 다른 변수 이름으로 바인딩하는 방법을 고려해야 해요.

        새로운 astr는 이전에 읽어온 String의 일부를 가리키는 슬라이스(&str)예요.
        즉, astr.trim()은 원래 String 내부의 데이터를 빌려와서 공백이 제거된 부분만 참조하는 슬라이스를 반환해요. 
        비록 원래의 astr라는 이름은 shadowing을 통해 가려지지만, 원래의 String 값은 drop되지 않고 여전히 메모리에 남아 있어 새로운 슬라이스가 그 데이터를 올바르게 참조할 수 있게 돼요.
        즉, 원래의 String은 여전히 존재하며 그 데이터에 대한 슬라이스를 생성한 것이기 때문에, 새로운 astr는 안전하게 해당 데이터를 참조할 수 있는 거예요.


        Q. 그럼 내가 준 코드에서 원래의 String이 어떤 값을 가져?

        A. 원래의 String은 파일 전체의 내용을 그대로 담고 있어요.
        즉,
	    •	fs::read_to_string(afile).unwrap()가 반환한 String은 파일에 있는 모든 텍스트(공백 포함)를 저장합니다.
	    •	이후 astr.trim()을 호출하면, 이 원래 String의 데이터를 바탕으로 양쪽 끝의 불필요한 공백이 제거된 **슬라이스(&str)**를 생성합니다.
        이때 shadowing을 통해 새롭게 let astr = astr.trim();로 선언하면, 이전의 String은 이름은 가려지지만,
        새로 만들어진 슬라이스가 그 원본 데이터를 참조하기 때문에 원래의 String 값은 메모리에서 그대로 유지됩니다.
        따라서, 이후에 새로운 astr를 사용할 때는 공백이 제거된 슬라이스만 접근할 수 있고, 원래의 전체 텍스트에 직접 접근할 수는 없지만, 메모리 상에서는 원래의 String 데이터가 여전히 존재하는 거예요.


        Q. 그럼 새롭게 let astr로 덮어쓰고나면, 메모리 상에 있는 원래의 String 데이터에는 접근할 수 없는건가? 아니면 &참조로 접근할 수 있는거야? &참조로 접근할 수 있다면 어떻게 작성해야하지?
        
        A. shadowing을 통해 새롭게 같은 이름으로 바인딩하면, 원래의 String에 접근할 수 있는 이름은 사라지고 새롭게 &str 타입의 값이 그 이름에 할당돼요.
        즉,
        let astr = fs::read_to_string(afile).unwrap(); // astr: String
        let astr = astr.trim(); // 새롭게 선언된 astr: &str

        이렇게 하면, 이후 astr를 사용하면 이제 &str 슬라이스만 접근하게 되고, 원래의 String 데이터에 직접 접근할 수 없어요.
        그렇지만, 중요한 점은 trim()이 반환하는 슬라이스는 내부적으로 원래의 String 데이터(힙 메모리에 있는 텍스트)를 참조하고 있으므로, 그 데이터는 메모리 상에 여전히 존재해요. 
        다만, 접근하는 이름은 잃어버리게 되는 거죠. 

        만약 원래의 String과 슬라이스 둘 다 사용하고 싶다면, shadowing 대신 별도의 변수에 저장하면 돼요. 
        예를 들어:
        let original = fs::read_to_string(afile).unwrap(); // original: String
        let trimmed = original.trim(); // trimmed: &str, original 데이터를 참조함

        이렇게 하면 original 변수로 String의 모든 기능을 사용하고, trimmed 변수로 공백이 제거된 슬라이스에 접근할 수 있어요.


        Q. 그럼 내가 처음 줬던 코드는 let astr로 덮어씀으로써, 메모리상에 원래의 String 데이터는 남아있지만, 접근할 수 없다면, 메모리만 더 쓰고 비효율적인 코드 아냐?

        A. 그림자(Shadowing)로 인해 변수 이름이 재사용되어 원래의 String에 직접 접근할 수 없게 되는 건 맞아요. 하지만 이게 꼭 비효율적이라는 뜻은 아니에요.
            1.	메모리 재사용:
            •	astr.trim()이 반환하는 슬라이스(&str)는 원래 String 내부 데이터를 참조합니다.
            •	즉, 새로 바인딩한 astr는 원래의 String 데이터에 대한 포인터 역할을 하므로, 메모리에 중복 데이터가 생성되는 것은 아니에요.

            2.	메모리 해제:
            •	만약 원래의 String 데이터에 더 이상 접근할 필요가 없다면, 이 방식은 보통 문제가 되지 않아요.
            •	슬라이스의 유효성을 위해 원래의 String은 계속 메모리에 남아있어야 하므로, 이를 제거할 수는 없어요.

            3.	효율성:
            •	만약 실제로 원본 데이터가 필요 없고, trimmed된 결과만 필요하다면 String 자체를 새로 만들어서 사용(예: let trimmed = astr.trim().to_string();)하는 방법도 있지만, 
                이 경우에는 새로운 메모리 할당과 데이터 복사가 일어나게 됩니다.
            •	따라서 슬라이스를 사용하는 현재 방식이 메모리 사용 측면에서 오히려 효율적일 수 있어요.

        결론적으로, shadowing을 사용하여 &str 슬라이스를 만든 현재 코드는, 메모리상에 원래의 String이 남아있는 것이 필요조건이며, 비효율적이라고 보기는 어렵습니다.
        만약 원본 String의 기능이 전혀 필요 없고, 메모리 사용을 최소화하고 싶다면 별도의 변수로 관리하는 방법이나, 데이터를 복사해서 소유권을 가지는 방법을 고려할 수 있지만, 
        이 경우에는 오히려 메모리 사용량이 늘어날 수 있어요.
     */

    // 비교
    if astr == bstr {
        println!("OK");
    } else {
        println!("Different result");
    }
}

fn dictionary_ex2() {
    println!();
    println!("Dict 예제 2");

    use std::fs::File;
    use std::io::{BufRead, BufReader};

    // 사전 파일 지정
    let dictfile = "dict.txt";

    // 명령줄 인수를 벡터에 할당
    let args: Vec<String> = std::env::args().collect();

    // 인수 확인
    if args.len() < 2 {
        println!("[USAGE] ./dictionary word");
        return;
    }

    // 인수로 전달된 단어
    let word = &args[1];

    /* 
        Q. args[1]이 아닌 &args[1] 을 사용하는 이유?

        A. Rust의 소유권(ownership)과 빌림(borrowing) 개념 때문에 그렇습니다.
        •	인덱싱의 반환값:
                Vec<T>에서 인덱싱(args[1])을 사용하면 실제로는 소유권을 이동시키는 것이 아니라 해당 요소에 대한 참조(&T)가 반환됩니다. 즉, args[1]의 타입은 이미 &String입니다.
        •	왜 추가로 &를 붙일까?
                코드에서는 명시적으로 &args[1]라고 쓰는데, 이는 다음과 같은 이유 때문입니다:
        •	소유권 이동 방지:
                만약 소유권을 가져가고 싶다면 clone() 등을 사용해야 하는데, 그렇지 않고 단순히 빌리기(참조)를 원하는 경우, 명시적으로 &를 붙여 해당 요소를 빌린다는 의미를 강조합니다.
        •	자동 디레퍼런스 코어션:
                많은 함수들이 &str 타입을 인자로 받습니다. String은 Deref 트레이트를 통해 &str로 자동 변환되므로, &&String (즉, &args[1])을 사용해도 필요에 따라 &str로 변환됩니다.

        요약하면, &args[1]는 벡터 내의 값을 소유권을 이동시키지 않고 안전하게 빌려와서 사용하기 위함입니다. 이는 Rust의 메모리 안전성을 유지하는 중요한 패턴입니다.


        Q. 그럼 &args[1]이 아닌 args[1]이라고 고쳐써도 결과는 같아?

        A. Rust에서 벡터의 인덱싱 연산자는 이미 참조를 반환합니다. 즉,
        •	args[1]의 타입:
            Vec<String>에서 args[1]을 사용하면 실제로 반환되는 타입은 &String 입니다.
        •	&args[1]의 타입:
            args[1]이 이미 &String이므로, 추가로 &를 붙이면 &&String (이중 참조)가 됩니다.

        그러나 많은 경우, 특히 문자열 슬라이스(&str)로 자동 변환될 때, Rust의 디레퍼런스 코어션에 의해 &&String도 문제없이 &str로 변환되기 때문에 두 방식 모두 동일하게 작동합니다.

        즉, 함수가 &str 또는 &String을 요구하는 상황에서는
        •	args[1] 만 사용해도 충분하고,
        •	&args[1]을 사용해도 Rust의 자동 변환 덕분에 결과적으로는 같은 효과를 얻습니다.
    */


    // 사전 파일 열기
    let fp = File::open(dictfile).unwrap();

    // BufReader로 읽어들임
    let reader = BufReader::new(fp);

    for line in reader.lines() {
        // 한 줄씩 처리
        let line = line.unwrap();

        // 지정한 단어가 포함되는 줄인지 확인
        if line.find(word) == None { continue; }

        println!("{}", line);
    }
}

fn dictionary_ex3() {
    println!();
    println!("Dict 예제 3");

    use std::fs::{self, File};
    use std::io::{Write, BufWriter};

    // 출력할 파일 이름 지정
    let filename = "fizzbuzz_file_result.txt";

    // 파일로 저장할 부분을 블록으로 지정 (Scope 지정)
    {
        // 파일 생성 및 (쓰기 위해) 열기
        let fp = File::create(filename).unwrap();
        let mut writer = BufWriter::new(fp);        // 아래에서 For 문을 이용해 한 줄씩 기록하는 형태이므로, BufWriter를 이용해 한 줄씩 저장한다.

        // FizzBuzz를 100까지 구하기
        for i in 1..=100 {
            let mut line = format!("{}\n", i);

            if (i % 3 == 0) && (i % 5 == 0) {
                line = String::from("FizzBuzz\n");
            } else if i % 3 == 0 {
                line = String::from("Fizz\n");
            } else if i % 5 == 0 {
                line = String::from("Buzz\n");
            }

            // 파일에 쓰기
            let bytes = line.as_bytes();        // 문자열 변수 line에 as_bytes 메서드를 이용해 데이터를 "바이너리 데이터"로 변환한다.
            writer.write(bytes).unwrap();       // 그리고 이 바이너리 데이터를 BufWriter의 write 메서드를 이용해 실제로 파일에 쓴다.
        }
    }   // <- 파일은 여기서 자동으로 닫힌다. (지정한 영역 내에서 사용된 파일 생성 변수/기능이 끝났으므로.)
        // 러스트에서는 블록 안에서 생성한 변수는 해당 블록 안에서만 이용할 수 있다. 이는 러스트의 메모리 안정성을 위한 라이프타임(Lifetime)과 관련이 있다.
        // 블록 안에서 선언한 변수는 블록을 벗어나면 자동으로 소멸된다.

    // 저장한 파일의 내용을 읽어들여 출력
    let s = fs::read_to_string(filename).unwrap();
    println!("{}", s);
}