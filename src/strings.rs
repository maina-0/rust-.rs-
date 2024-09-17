//use std::string::String;

pub fn str(){

    let hello0 = "hello can change "; //a primitive string!
                           //
    let hello1 = String::from("hello can change the "); //a growable string!
                                       
     

    println!("{}\n{}", hello0,hello1);


    //string length
    
    let x = hello0.len();

    println!("the string length of \"{0}\"x is x={1}",hello1,x);

    
    let mut hey=String::from ('W');

    hey.push('o');

    hey.push_str("rld");

    println!("{}{}",hello1,hey);


    //capacity in bytes
    println!("number of bytes it can store is {}", hey.capacity());
    
    //check if string is empty
    //
    //
    println!("Is it empty ?{}",hey.is_empty());


    //replace  

    println!("{}",hello0.replace("can","cannot"));


    //declare a string the initialise


    let mut s = String::with_capacity(15);
    s.push('h'); 
    s.push('a');
    s.push('h');
    s.push('\u{1f602}');

        println!("{}",s);


    //comparing/assertion testing
    //assert will only give an output if it failed that if if what being compared is not equal
    assert_eq!("Boy","boy");

}
