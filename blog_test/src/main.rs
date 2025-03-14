use std::{env, fs};

fn main() {
    // 명령줄 인수 취득
    let args = env::args();
    let mut total: f64 = 0.0;

    // 모든 인수 처리
    for (i, fname) in args.enumerate() {
        if i == 0 { continue; }

        // 텍스트 파일 읽기
        let text = fs::read_to_string(fname).unwrap();

        // 한 줄씩 분리
        let lines = text.split('\n');

        // 반복해서 계산
        for line in lines {
            let n: f64 = match line.parse() {
                Ok(v) => v,
                Err(_) => 0.0,
            };
            total += n;
        }
    }

    println!("{}", total);
}