pub fn prisioners(size:i32) -> Vec<i32>{
    let mut vec:Vec<i32> = Vec::new();
    let mut counter:i32 = 0;

    for _i in 0..size {
        counter = counter +1;
        vec.push(counter);
    }

    return vec;

}