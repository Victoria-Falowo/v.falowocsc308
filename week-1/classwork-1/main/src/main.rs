use std::io;

fn main() {
    let mut input1 = String::new();
    let mut input2 = String::new();
    let mut input3 = String::new();
    
    println!("- - - - -  \nWelcome! \n- - - - - \n");

    //Choosing temperature
    loop{
        println!("Please choose what temperature you want to change to? \n- - - - - - - - - - - - - - - - - - - - - - - - - - - \n1. Celsius \n2. Fahrenheit\n- - - - - - - - - - - - - - - - - - - - - - - - - - -  \n\n");
        io::stdin().read_line(&mut input1).expect("Not a valid string");
        let choice:u32 = input1.trim().parse().expect("Not a valid number");

        if choice == 1 {
            println!("What is the temperature in Fahrenheit? E.g(34.0,75.4)");
            io::stdin().read_line(&mut input2).expect("Not a valid string");
            let fah1:f32 = input2.trim().parse().expect("Not a valid number");

            let cel1 = (fah1 - 32.0) * (0.6);

            println!("The temperature in celsius is {} ", cel1);
            break;
        }
        else if choice == 2 {
            println!("What is the temperature in Celsius? E.g(34.0,75.4)");
            io::stdin().read_line(&mut input3).expect("Not a valid string");
            let cel2:f32 = input3.trim().parse().expect("Not a valid number");

            let fah2 = (cel2 * 1.8) + 32.0;

            println!("The temperature in fahrenheit is {} ", fah2);
            break;
        }
        else{
            println!("Please pick between 1 and 2\n");
            continue;
        }
    }
}
