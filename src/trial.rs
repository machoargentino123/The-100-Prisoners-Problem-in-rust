
use std::collections::{BTreeMap};

pub fn trial(prisioners:Vec<i32>,boxes:BTreeMap<i32,i32>,limit:i32) -> bool{

    //path:i32 = 0;
    let flag:bool;
    let mut valor:i32 = 0;
    let mut cont:i32 = 0;
    let size:i32 = prisioners.len().try_into().unwrap();
    
        //all prisioners
        for i in prisioners{
            
            //have limited oportunities
            for a in 1..limit{
                //first time open the box ith his number.
                if a == 1{
                        valor = *boxes.get(&i).unwrap(); 
                        
                        if i == valor{
                            //if it is ok add 1 to the counter and finish loop.
                            cont = cont + 1;
                            break;
                        }else{
                            do_nothing();
                        }
                //if in the first box is not the prisioner number. go to number box, that 
                //get in the last box.
                }else{
                    valor = *boxes.get(&valor).unwrap();

                    //if it is ok add one to the counter.
                    if i == valor{
                        cont = cont + 1;
                        break;
                    }else{
                        do_nothing();
                    }

                }         
    
            };
        };

    //To survive all must be success. 
    if cont  == size {

        flag = true;
        return flag;
    }else{

        flag = false;
        return flag;
    }
}

fn do_nothing(){}