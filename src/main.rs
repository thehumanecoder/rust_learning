// this is here is a happy case use of code blocks 

fn main() {
    let x = 10;

    {
        let y = 5 ;

        println!("x:{}, y:{}",x,y);
    }
}

// below will throw an exception that y is not defined
// fn main() {
//     let x = 10;

//     {
//         let y = 5 ;

//     }
//     println!("x:{}, y:{}",x,y);

// }