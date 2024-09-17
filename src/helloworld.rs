pub fn run () {
    //basic formating
    println!("hello from the helloworld.rs file");

    //positional arguments
    println!("{0} added with {1} results in {2}, but the diffrence is {3} AKA {4} because {0} is less than {1}", 2,3,5,-1, "negative one");


    //named arguments
    println!("{john} is an incredible {boy} but {joan} isn't that of a {girl} ", john="john", joan="joan", boy="boy", girl="girl");

    //"fomat specifier"
    println!("binary is {:b}, octal is {:o}, hex is{:x}", 199,199,199);
}
