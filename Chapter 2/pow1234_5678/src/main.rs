// BigInt를 사용하기 위한 선언
use num_bigint::BigInt;

// use 사용법
/*
    use 크레이트명::모듈;
    use 크레이트명::모듈1::모듈1-1;
    use 크레이트명::{모듈1, 모듈2};
*/

fn main() {
    // BigInt 오브젝트를 만들어 값을 설정
    let v = BigInt::from(1234);
    println! {"{}", v.pow(5678)};
}
