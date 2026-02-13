macro_rules! get_sum {
    // 쉼표로 구분된 임의의 개수의 식을 입력으로 받음
    ($($x:expr), *) => {{
        // 식들을 벡터에 담기
        let args = vec![$($x), *];

        // 벡터를 반복하며 요소들의 합 구하기
        args.iter().sum::<i32>()
    }};
}

fn main() {
    println!("{}", get_sum!(1, 2));
    println!("{}", get_sum!(1, 2, 3));
}
