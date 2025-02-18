// 배열을 섞기 위한 rand 크레이트 이용 선언
use rand::seq::SliceRandom;

fn main() {
    // 1에서 75까지의 숫자로 이루어진 배열을 생성
    let mut nums = [0; 75];
    for i in 1..=75 {
        nums[i - 1] = i;
    }

    // 섞기
    let mut rng = rand::thread_rng();
    nums.shuffle(&mut rng);

    // let mut i;

    // 카드 표시
    for y in 0..5 {
        for x in 0..5 {
            let i = y * 5 + x;      // 반복문 내에서 계속 변수를 생성하는게 리소스를 더 잡아먹는게 아니냐 할 수 있지만, 러스트의 소유권/라이프타임 등 메모리 관리에 뛰어난 성능을 보이는 특징을 생각하면 오히려 활용해야할 것.
            // i = y * 5 + x;

            if i == 12 {
                // 와일드 카드
                print!("  *,");
            } else {
                print!("{:3},", nums[i]);
            }
        }

        println!("");
    }
}
