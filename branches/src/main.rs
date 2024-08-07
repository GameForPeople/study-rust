fn main() {


    // 기본적인 if문 분기
    {
        let number = 6;

        if number % 4 == 0 {
            println!("number is divisible by 4");
        } else if number % 3 == 0 {
            println!("number is divisible by 3");
        } else if number % 2 == 0 {
            println!("number is divisible by 2");
        } else {
            println!("number is not divisible by 4, 3, or 2");
        }

        // C++와 상이하게 int형이 bool형으로 안바뀌기 떄문에, if number 이런식으로는 안된다.
        // Rust에서는 "표현식이 두 개 이상이면 코드를 리팩터링하는 것이 좋습니다. 6장에서는 이런 경우에 적합한 match라는 러스트의 강력한 분기 구조"
    }

    // C++의 삼항연산자처럼, 변수의 값을 세팅할 떄, if문을 사용하여 처리함.
    {
        let condition = true;
        let _number = if condition { 5 } else { 6 };

        println!("number : {_number}");
    }
}