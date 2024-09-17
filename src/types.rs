pub fn typ(){

    let mut x = std::u32::MAX;

    let mut y = std::u32::MAX;

    let mut z = std::u32::MAX;



    println!("max u32={0},\nmax u32={1},\nmax u32={2},\n",x,y,z);

    //bool
    


    x=3;
    y=4;

    z=x+y;

    let is_equal: bool = z==9;

    let face ='\u{1F639}';

    println!("{:?}{:?}",is_equal,face);
}
