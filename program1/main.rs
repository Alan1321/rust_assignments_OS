use std::io;

/*
This function below is the main function where the program initially begins.
It accepts inputs and calls other functions to calculate and output volume data.
*/
fn main(){

    println!("Welcome to the volume calculator. You may enter quit at the prompt at any time to exit the program.\n");

    loop {
        //These 3 lines accepts user input as String
        println!("Enter the radius of your sphere (in cm):");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("failed to read line");

        //If input is 'quit' then quit the program
        if input == "quit\n"{
            break;
        }

        //Here we convert String to float and also do error handling for strings
        //other than 'quit'
        let num: f64 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid Input :(\n");
                continue;
            }
        };

        //if the radius is 0 or less, the input is invalid
        if num <= 0.0 {
            println!("Invalid Input :(\n");
            continue;
        }

        //if everything works out, call the result function and print out volume
        result(num);   
    }
}

/*
This function prints out volume to the screen
*/
fn result(radius: f64){
    println!("For a sphere with a radius of {} cm, The volume is: {} cm-cubed, or {} inches-cubed\n", radius, calculate_volume(radius), calculate_volume(cm_to_inches(radius)));
}

/*
This function accepts radius as input, calculates volume and gives result back
*/
fn calculate_volume(radius: f64) -> f64{
    const PI: f64 = 3.1415926; //declare PI as constant
    return 4.0 * PI * radius * radius * radius / 3.0

}

/*
This function accepts radius in cm as input, converts to inches, and returns it back
*/
fn cm_to_inches(radius: f64) -> f64{
    return radius / 2.54;
}