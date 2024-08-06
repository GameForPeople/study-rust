fn main() 
{
    {

        const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
        
        let mut x = 5;
        println!("The value of x is: {x}");
        x = 6;
        println!("The value of x is: {x}");
        x = THREE_HOURS_IN_SECONDS;
        println!("The value of x is: {x}");

    }

    {
        let x = 5;
        
        let mut x = x + 1;
        
        {
            let x = x * 2;
            println!("The value of x in the inner scope is: {x}");
        }
        
        println!("The value of x is: {x}");
    }

    {
        // 덧셈
        let sum = 5 + 10;
    
        // 뺄셈
        let difference = 95.5 - 4.3;
    
        // 곱셈
        let product = 4 * 30;
    
        // 나눗셈
        let quotient = 56.7 / 32.2;
        let truncated = -5 / 3; // 결괏값은 -1입니다
    
        // 나머지 연산
        let remainder = 43 % 5;
    }

    {
        let t = true;
        let f: bool = false; // 명시적인 타입 어노테이션
    }

    {
        // 러스트에서의 char는 4바이트로 처리된다.

        let c = 'z';
        let z: char = 'ℤ'; // 명시적인 타입 어노테이션
        let heart_eyed_cat = '😻';
    }

    
}
