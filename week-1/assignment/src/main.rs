use std::io;

fn main() {
    let mut input1 = String::new();

    println!("- - - - - - - - - - - - \nENERGY BILLING SYSTEM \n- - - - - - - - - - - -");
    
    println!("How much energy did you use in kwh? (Eg. 200.0, 320)");
    io::stdin().read_line(&mut input1).expect("Not a valid string");
    let pinitial:f32 = input1.trim().parse().expect("Not a valid number");

    if (pinitial > 100.0) & (pinitial < 201.0){
        let pfinal = pinitial * 25.0;
        println!("KWH Used: {:.2} \nPrice per Unit: ₦25 \nFinal Bill: ₦{:.2}", pinitial, pfinal);
        println!("\n\nGOODBYE! \n- - - - -");
    }
    else if pinitial > 200.0{
        let pfinal = pinitial * 30.0;
        println!("KWH Used: {:.2} \nPrice per Unit: ₦30 \nFinal Bill: ₦{:.2}", pinitial, pfinal);
        println!("\n\nGOODBYE! \n- - - - -");
    }
    else{
        let pfinal = pinitial * 20.0;
        println!("KWH Used: {:.2} \nPrice per Unit: ₦20 \nFinal Bill: ₦{:.2}", pinitial, pfinal);
        println!("\n\nGOODBYE! \n- - - - -");
    }
}