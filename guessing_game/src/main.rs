use std::io;
use std::cmp::Ordering;
use rand::Rng;
// extern crate rand; use rand::Rng; 대신에 쓸 수 있음

fn main(){
    println!( "숫자를 맞추세요!");

    let secret_number = rand::thread_rng().gen_range( 1, 101 );

    println!( "비밀 숫자는! {}", secret_number );

    loop 
    {
        println!( "너의 숫자를 입력하세요!" );
        
        let mut guess = String::new();
        
        io::stdin().read_line( &mut guess )
            .expect( "Failed To read lin" );
    
        let guess: u32 = match guess.trim().parse() 
            {
                Ok(num) => num,
                Err(_) => continue,
            };

        println!( "you guessed: {}", guess );

        match guess.cmp( &secret_number ) 
        {
            Ordering::Less    => println!("작다!"),
            Ordering::Greater => println!("크다!"),
            Ordering::Equal   => 
            {
                println!("맞췄다!");
                break;
            }
        }
    }
}