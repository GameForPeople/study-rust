fn main() 
{
    another_function( 77 );
    print_labeled_measurement(5, 'h');


    // 러스트에서는 표현식과 구문을 언어차원에서 명확하게 구분된다.
    // 표현식 expression : 결괏값을 평가
    // 구문 statement : 값을 반환하지 않음

    // let x = (let y = 6);
    //  6는 표현식( 6이라는 값을 반환 )
    //  (let y = 6)는 구문, 어떠한 값도 반환하지 않음, 따라서 x는 값을 받지 못하므로, 빌드에러

    {

        let y = 
        {
            let x = 3;
            x + 1
        };
        
        println!( "y : {y}" );
        
        // x는 3으로 세팅되고, 이후 이 x + 1은 3 + 1로 처리되는데, y를 4로 세팅하는 표현식임
        //  x + 1은 표현식
        // x + 1;은 구문임, 어떠한 값도 반환하지 않음.. 세미콜론 차이가 있다.
    }

    {
        // 함수는 다음과 같은 특성을 가짐
        //  #### 세미콜론이 없으면 표현식이다. 러스트의 함수는 마지막 표현식을 반환한다.
        // 아 러스트의 함수명은 스네이크 케이스로 만든다.
        fn get_five( x : i32 ) -> i32 
        {
            if x == 7
            {
                return 77777;
            }

            5
        }

        println!( "getfive 0 : {}", get_five(0) );
        println!( "getfive 7 : {}", get_five(7) );
    }


    {
        fn plus_one(x: i32) -> i32 
        {
            x + 1
        }

        fn plus_three(x: i32) -> i32 
        {
            // 리턴할떄는 세미콜론 붙이든 안붙이든 상관 없네?
            return x + 3;
        }

        println!( "plus_one 0 : {}", plus_one(0) );
        println!( "plus_three 0 : {}", plus_three(0) );
    }

    {
        // 러스트의 주석은 ffffff보다 AAAAAA가 좋단다. 난 시른데
        
        fn ffffff() 
        {
            let lucky_number = 7; // 오늘 운이 좋은 느낌이에요
        }

        fn AAAAAA() 
        {
             // 오늘 운이 좋은 느낌이에요
            let lucky_number = 7;
        }
    }
}

fn another_function( x : i32 ) 
{
    println!( "Another function. x : {x} " );
}

fn print_labeled_measurement(value: i32, unit_label: char) 
{
    println!("The measurement is: {value}{unit_label}");
}
