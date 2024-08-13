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


    // for-loop 처리합니다.
    {
        // 기본 for-loop
        {

            let mut counter = 0;
            
            let result = 
            loop 
            {
                counter += 1;
                
                if counter == 10 
                {
                    break counter * 2;
                }
            };
        
            println!( "counter : {counter}, result : {result}" );
        }

        // 복수 for-loop에서 loop의 name을 지정할 수 있음 오오 고투네
        {
            let mut first_loop_count = 0;

            'first_loop: loop 
            {
                first_loop_count += 1;

                let mut second_loop_count = 0;

                /*'second_loop: 안쓸거면 주석처리해야된다. */loop 
                {
                    second_loop_count += 1;

                    first_loop_count += second_loop_count;

                    println!( 
                        " first_loop_count : {first_loop_count}, 
                    second_loop_count : {second_loop_count}" );

                    if first_loop_count > 100
                    {
                        break 'first_loop;
                    }
                }
            }
        }

        // while문 똑같다
        {
            let mut number = 3;

            while number != 0 
            {
                println!("{number}!");
        
                number -= 1;
            }
        }

        // for문은 범위 기반 반복문으로만 사용된다.
        {
            let a = [10, 20, 30, 40, 50];

            for element in a 
            {
                println!("the value is: {element}");
            }
        }

        // .......? 이건 뭐야
        for number in (1..4).rev() 
        {
            //// 선넘네;;
            println!(" number : {number}!");
        }
    }
}