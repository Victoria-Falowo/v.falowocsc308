use std::io;

fn main() {
    let mut input1 = String::new();

    println!("- - - - - - \nCHECK OUT! \n- - - - - -");
    
    println!("How much did you spend? (Eg. 2000.0, 3200)");
    io::stdin().read_line(&mut input1).expect("Not a valid string");
    let pinitial:f32 = input1.trim().parse().expect("Not a valid number");

    if (pinitial > 5000.0) & (pinitial < 10001.0){
        let pfinal = pinitial - (pinitial * 0.1);
        println!("\n\nInitial Bill: ₦{:.2} \nDiscount applied: 10% \nFinal Bill: ₦{:.2}", pinitial, pfinal);
        println!("\n\nGOODBYE! \n- - - - -");
    }
    else if pinitial > 10000.0{
        let pfinal = pinitial - (pinitial * 0.15);
        println!("\n\nInitial Bill: ₦{:.2} \nDiscount applied: 15% \nFinal Bill: ₦{:.2}", pinitial, pfinal);
        println!("\n\nGOODBYE! \n- - - - -");
    }
    else{
        let pfinal = pinitial;
        println!("Initial Bill: ₦{:.2} \nDiscount applied: 0% \nFinal Bill: ₦{:.2}", pinitial, pfinal);
        println!("\n\nGOODBYE! \n- - - - -");
    }
}