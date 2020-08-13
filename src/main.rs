use text_io::read;

fn main() {
    menu();
}

// Possibly make struct for creating new shapes

fn menu() {
    println!("Type what shape you want(cb: cube | sp: sphere)");
    let user_menu_input: String = read!();
    if user_menu_input == "cb"{
        cube();
    } else if user_menu_input == "sp" {
        sphere();
    }
}

fn cube() {
    println!("Enter dimensions:");
    let user_input: i32 = read!();
    let calc_vol: i32 = user_input*3;
    println!("The volume of the cube is: {}", calc_vol );
}

fn sphere() {
    println!("Enter dimensions:");
    let user_input: f64 = read!();
    let calc_vol: f64 = 4.0/3.0*3.14*user_input.powf(3.0);
    println!("The volume of the sphere is: {}", calc_vol );
}