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
    // end data types
}
