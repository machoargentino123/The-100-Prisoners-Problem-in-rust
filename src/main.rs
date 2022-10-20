#[allow(unused_imports)]
use std::collections::{BTreeMap};
use std::{time::{Instant}};
mod boxes;
mod prisioners;
mod trial;




fn main(){

for _i in 1..10{
    tries();
}

}


fn tries() {


    /*problems of olds funtions.
    
    1st funtion that return an integer.
    2nd funtion that compairs integer and hash and return a false if the 
        value exists in the hash.
    3rd function that reurn a hash y add a ranmdom value not repeated, if it is
    repeated not add and again reply for a new random number.
    
    Problems: when increase the hash size the reply of random and not repeated number,
     
    
    ---     ---                 ---     ---     ---     ---     ---  
    ---     ---                 ---     ---     ---     ---     ---
    --- ok  ---                 ---     ---     ---     ---     ---
            --- rep             --- rep ---ok   ---     ---     --- 
                                                --- rep --- rep ---ok


    */
    /* 
    Solution: Create a ordered hash, and shuffle.
    generate a list of n size len with no repeated random integers.
    using Fisher-Yates and Inverse Riffle Shuffle
    
    */



    let start = Instant::now();

    let mut positives:i32 = 0;
    let mut negatives:i32 = 0;

    //number of trys
    let tries:i32 = 100;
    
    for _i in 1..tries{
        
        //quantity of prisioners and boxes.
        let size:i32 = 100;
        
        let boxes:BTreeMap<i32,i32> = boxes::create_index_ordered_hash(size);
        
        let limit:i32 = size/2;
        let prisioners:Vec<i32> = prisioners::prisioners(size);
    
        let resultado = trial::trial(prisioners,boxes,limit);
    
        if resultado == true{
            positives = positives + 1;
        }else if resultado == false{
            negatives = negatives + 1; 
        }else{
            do_nothing();
        }
    }

    println!("Positives results: {positives}");
    println!("Negatives results: {negatives}");
    let porc_pos:f32 = (&positives*100) as f32 /(&positives+&negatives) as f32; 
    println!("Positives: {}%",porc_pos);
    let porc_pos:f32 = (&negatives*100) as f32 /(&positives+&negatives) as f32; 
    println!("Negatives: {}%",porc_pos);

    let duration = start.elapsed();
    println!("Duracion de ciclo de programa: {:?}", duration);

    
}    

fn do_nothing(){}


