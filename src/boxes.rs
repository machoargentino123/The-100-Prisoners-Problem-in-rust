use std::collections::{BTreeMap};
use rand::Rng;
use std::time::{Instant};
use shuffle::shuffler::Shuffler;
use shuffle::irs::Irs;
use rand::rngs::mock::StepRng;



#[allow(dead_code)]
pub fn create_random_list(size:i32) -> Vec<i32>{


    let mut vec:Vec<i32> = Vec::new();
    let mut counter:i32 = 0;

    for _i in 0..size {
        counter = counter +1;
        vec.push(counter);
    }

    let mut rng = StepRng::new(2, 13);
    let mut irs = Irs::default();
    
    #[allow(unused_must_use)]
    let _unused = irs.shuffle(&mut vec, &mut rng);
    
    return vec;

}

#[allow(dead_code)]
pub fn create_index_ordered_hash(size:i32) -> BTreeMap<i32,i32>{


    let list:Vec<i32> = create_random_list(size);

    let mut orders: BTreeMap<i32,i32> = BTreeMap::new();
    let mut order:i32 = 0;

    for i in list{
        order = order+1;
        orders.insert(order,i);
    }
    
    return orders;


}


#[allow(dead_code)]
fn old_hash_generator(size:i32) -> BTreeMap<i32,i32> {
    let start = Instant::now();

    //vector primario que me permite generar el hash de tamaño de la variable size
    let vec = vec![0;size.try_into().unwrap()];
    let mut orders: BTreeMap<i32,i32> = BTreeMap::new();
    let mut order = 0;
    let mut valor:i32=0;
    let mut condition:bool;

    for _i in vec{
        order += 1;
        condition = false;
     
        while condition == false{
            valor = old_aleatorio(size);
            condition = old_verifier(valor,&orders);
        }
        orders.insert(order,valor);
        
        if order%2 == 0{
            println!("loop de orden: {}",order);
        }

        };
    

    let duration = start.elapsed();

    println!("Duracion de ciclo de programa: {:?}", duration);
    return orders;


}


#[allow(dead_code)]
fn old_aleatorio(nums:i32) -> i32 {
    let mut rng = rand::thread_rng();
    let output = rng.gen_range(1..nums+1);
    return output;
}

//verify if in the hashmapzz, exist the value valor
#[allow(dead_code)]
fn old_verifier(valor:i32,orders:&BTreeMap<i32,i32>) -> bool{
    let valor2 = valor.clone();
    let orders2 = orders.clone();
    let mut output:bool = false;
    let values = orders2.len();
    
    //If it is empty the btreemap, dont process.
    if orders2.is_empty() == true{
        output = true;
    //if len of the btreemap is 1 not, advance.
    }else if values == 1{
        output = true;

    }else{
        //devuelve un vector con los valores de cada campo
       let values2 = orders2.into_values();
       for i in values2{
            if i == valor2{
                output = false;
                break;
            }else{
                output = true;
            }
       };
    };
    return output;
}

