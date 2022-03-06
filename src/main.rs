fn main() {
    println!("Hello, world!");

    // start data types
    let first_year = 1939;
    let second_year = 2022;
    println!("{first_year}");
    println!("{second_year}");

    let is_winter = true;
    let is_snowing = false;
    println!("winter: {is_winter}");
    println!("snowing: {is_snowing}");

    let stl_location = ("STL", 38.741_650_2, -90.365_725_8);
    println!("{} {} {}", stl_location.0, stl_location.1, stl_location.2);

    let mci_location = ("MCI", 39.300_642_7, -94.712_593_7);
    let (location, latitude, longitude) = mci_location;
    println!("{} {} {}", location, latitude, longitude);

    let city = "Chicago";
    let team = "Bears";
    let team_name = format!("{} {}", city, team);
    println!("{team_name}");

    let mut nickname = String::new();
    nickname.push_str("Monsters of the Midway");
    nickname.push_str(" - 1985 Super Bowl champs");
    println!("{nickname}");
    // end data types

    // start variables
    let age: u8 = 21;
    let current_year = 2022;
    let current_tax = 0.75;
    let cost = 2.95;
    let items = 12;
    // cast items to f64 to match cost
    let bill = cost * f64::from(items);
    println!("age = {age}");
    println!("current_year = {current_year}");
    println!("current_tax = {current_tax}");
    println!("bill = {bill}");
    let scope = "outer";
    println!("scope = {scope}");
    {
        let scope = "inner";
        println!("scope = {scope}");
    }
    println!("scope = {scope}");
    // end variables

    // start operators
    let modulas = 18 % 7;
    println!("modules = {modulas}");
    let squared = i32::pow(8, 2);
    println!("8 squared = {squared}");
    let float_int = f32::powi(3.4, 3);
    let float_float = f32::powf(3.4, 3.4);
    println!("float_int = {float_int}");
    println!("float_float = {float_float}");
    let order_ops = ((12 * 8 + 2) - 8 / 4) - 95;
    println!("order_ops = {order_ops}");
    let maybe_true = 1 == order_ops;
    let always_false = 1 == 2;
    let not_always_false = !always_false;
    println!("maybe_true = {maybe_true}");
    println!("always_false = {always_false}");
    println!("not_always_false = {not_always_false}");
    // end operators
}
