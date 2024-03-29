use std::fs::File;
use std::io::{Error, ErrorKind, Read};

fn main() {
    data_types();
    variables();
    operators();
    project_part1();
    control_flow();
    project_part2();
    ownership_borrowing();
    example_functions();
    example_closure();
    handle_error();
    propigate_error();
    data_structures();
    example_trait();
    example_vectors();
}

enum NavigationAids {
    Ndb(u16),
    Vor(String, f32),
    Vordme(String, f32),
    Fix {
        name: String,
        latitude: f32,
        longitude: f32,
    },
}

struct Waypoint {
    name: String,
    latitude: f64,
    longitude: f64,
}

struct Segment {
    start: Waypoint,
    end: Waypoint,
}

fn operators() {
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
    let have_boarding_pass = true;
    let have_id = false;
    if have_boarding_pass && have_id {
        println!("Welcome aboard.");
    } else {
        println!("Sorry. Please return to your car.");
    }
    let first_value = 42;
    let second_value = 1776;
    if first_value >= second_value {
        println!("first_value is larger.");
    } else {
        println!("second_value is larger.");
    }
    let bitwise_and = 1776 & 42;
    let bitwise_or = 1776 | 42;
    let bitwise_exclusive_or = 1776 ^ 42;
    let shift_left = 1776 << 1;
    let shift_right = 1776 >> 1;
    println!("bitwise_and = {bitwise_and}");
    println!("bitwise_or = {bitwise_or}");
    println!("bitwise_exclusive_or = {bitwise_exclusive_or}");
    println!("shift_left = {shift_left}");
    println!("shift_right = {shift_right}");
}

fn variables() {
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
}

fn data_types() {
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
    println!("{location} {latitude} {longitude}");
    let city = "Chicago";
    let team = "Bears";
    let team_name = format!("{city} {team}");
    println!("{team_name}");
    let mut nickname = String::new();
    nickname.push_str("Monsters of the Midway");
    nickname.push_str(" - 1985 Super Bowl champs");
    println!("{nickname}");
}

fn project_part1() {
    const EARTH_RADIUS_IN_KILOMETERS: f64 = 6371.0;

    let kcle_latitude_degrees: f64 = 41.4075;
    let kcle_longitude_degrees: f64 = -81.851_111;

    let kslc_latitude_degrees: f64 = 40.7861;
    let kslc_longitude_degrees: f64 = -111.9822;

    let kcle_latitude_radians = kcle_latitude_degrees.to_radians();
    let kslc_latitude_radians = kslc_latitude_degrees.to_radians();

    let delta_latitude = (kcle_latitude_degrees - kslc_latitude_degrees).to_radians();
    let delta_longitude = (kcle_longitude_degrees - kslc_longitude_degrees).to_radians();

    let inner_central_angle = f64::powi((delta_latitude / 2.0).sin(), 2)
        + kcle_latitude_radians.cos()
            * kslc_latitude_radians.cos()
            * f64::powi((delta_longitude / 2.0).sin(), 2);
    let central_angle = 2.0 * inner_central_angle.sqrt().asin();

    let distance = EARTH_RADIUS_IN_KILOMETERS * central_angle;
    println!("The distance between the two points is {distance:.1} kilometers");
}

fn control_flow() {
    let name = "Kyle";
    if name == "Kyle" {
        println!("Hi Kyle!");
    } else if name == "Bubba" {
        println!("Hi Bubba!");
    } else {
        println!("Hi there.");
    }

    let available_aircraft = "SR71";
    let min_crew = 2;
    let available_crew = 1;
    if (available_aircraft == "SR71" || available_aircraft == "F117-A")
        && available_crew >= min_crew
    {
        println!("Ready for flight!");
    } else {
        println!("Back to the drawing board...");
    }

    // println!("NBD: {}", NavigationAids::Ndb as usize);
    // println!("VOR: {}", NavigationAids::Vor as usize);
    // println!("VORDME: {}", NavigationAids::Vordme as usize);

    let airline = String::from("Southwest");
    let letter = airline.chars().nth(5);
    let value = match letter {
        Some(character) => character.to_string(),
        None => String::from("No value"),
    };
    println!("{value}");

    let animal = "Raptor";
    match animal {
        "Raptor" => println!("Run!"),
        "T-Rex" => println!("Run quietly!"),
        _ => println!("Chill."),
    }

    let nbd_freq = 384;
    match nbd_freq {
        200..=500 => {
            println!("Valid frequency");
        }
        _ => {
            println!("Invalid frequency");
        }
    }

    let ndb_uwl = NavigationAids::Ndb(385);
    let vor_dqn = NavigationAids::Vor(String::from("DQN"), 114.5);
    let vor_dme_sgh = NavigationAids::Vordme(String::from("SGH"), 113.2);
    let fix_tarry = NavigationAids::Fix {
        name: String::from("TARRY"),
        latitude: 40.05333,
        longitude: -83.91367,
    };

    print_nav_aid(&ndb_uwl);
    print_nav_aid(&vor_dqn);
    print_nav_aid(&vor_dme_sgh);
    print_nav_aid(&fix_tarry);

    let mut counter = 0;
    loop {
        counter += 1;
        if counter == 5 {
            continue;
        }
        if counter == 10 {
            break;
        }
        println!("{counter}");
    }

    let mut index = 1;
    while index <= 10 {
        println!("{index}");
        index += 1;
    }

    // exclusive range
    for i in 1..11 {
        println!("{i}");
    }

    // inclusive range
    for j in 1..=10 {
        println!("{j}");
    }

    let aircraft = ["Boeing 737", "Boeing 767", "Boeing 787"];
    for craft in aircraft {
        println!("{craft}");
    }
}

fn print_nav_aid(navaid: &NavigationAids) {
    match navaid {
        NavigationAids::Ndb(khz) => {
            println!("NDB frequency is {khz} kilohertz");
        }
        NavigationAids::Vor(id, freq) => {
            println!("VOR identifier is {id} and it's frequency is {freq} kilohertz");
        }
        NavigationAids::Vordme(id, freq) => {
            println!("VORDME identifier is {id} and it's frequency is {freq} kilohertz");
        }
        NavigationAids::Fix {
            name,
            latitude,
            longitude,
        } => {
            println!("FIX {name} is at {latitude} latitude and {longitude} longitude");
        }
    }
}

fn project_part2() {
    const EARTH_RADIUS_IN_KILOMETERS: f64 = 6371.0;

    let route = [
        ("KCLE", 41.4075, -81.851_111),
        ("LEYIR", 41.51030, -83.88080),
        ("PIONS", 41.65390, -84.48190),
        ("ZOSER", 41.72390, -84.78130),
        ("MODEM", 41.72800, -84.89730),
        ("BRYTO", 41.74170, -85.31320),
        ("SEWTO", 41.74780, -85.51130),
        ("GIJ", 41.76860, -86.31850),
        ("NEPTS", 41.96750, -87.05300),
        ("THORR", 42.12330, -87.60030),
        ("OBK", 42.22140, -87.95160),
        ("COTON", 42.31990, -89.31220),
        ("DBQ", 42.40150, -90.70910),
        ("VIGGR", 42.55520, -93.12410),
        ("FOD", 42.61110, -94.29480),
        ("ONL", 42.47050, -98.68690),
        ("BFF", 41.89420, -103.48200),
        ("OCS", 41.59020, -109.01500),
        ("PUDVY", 41.54270, -109.34200),
        ("WEGEM", 41.44560, -109.99000),
        ("KSLC", 40.7861, -111.9822),
    ];

    let mut total_distance = 0.0;
    let mut previous_waypoint: Option<(&str, f64, f64)> = None;

    for waypoint in route {
        match previous_waypoint {
            None => {
                previous_waypoint = Option::from(waypoint);
                continue;
            }
            Some(previous_waypoint_value) => {
                let previous_waypoint_radians = previous_waypoint_value.1.to_radians();
                let waypoint_radians = waypoint.1.to_radians();

                let delta_latitude = (previous_waypoint_value.1 - waypoint.1).to_radians();
                let delta_longitude = (previous_waypoint_value.2 - waypoint.2).to_radians();

                let inner_central_angle = f64::powi((delta_latitude / 2.0).sin(), 2)
                    + previous_waypoint_radians.cos()
                        * waypoint_radians.cos()
                        * f64::powi((delta_longitude / 2.0).sin(), 2);
                let central_angle = 2.0 * inner_central_angle.sqrt().asin();
                let distance = EARTH_RADIUS_IN_KILOMETERS * central_angle;
                total_distance += distance;
                previous_waypoint = Option::from(waypoint);

                println!(
                    "The distance between {} and {} is {:.1} kilometers",
                    previous_waypoint_value.0, waypoint.0, distance
                );
            }
        }
    }

    println!("\nThe total distance between the two points is {total_distance:.1} kilometers");
}

fn ownership_borrowing() {
    let mut original = String::from("I was here first.");
    println!("original = {original}");

    {
        let next = &mut original;
        *next = String::from("I was here second.");
        println!("next = {next}");
        println!("original = {original}");
    }

    println!("original = {original}");
}

fn example_functions() {
    greet_lawdogs();
    let greater1 = find_greater(42, 31);
    println!("greater1 = {greater1}");
    let greater2 = find_greater(22, 76);
    println!("greater2 = {greater2}");
    let mut original = String::from("original value");
    print_original(&original);
    change_original(&mut original);
    print_original(&original);
}

fn greet(name: &str) {
    println!("Hello {name}!");
}

fn greet_lawdogs() {
    let lawdogs = ["Wyatt", "Doc", "Virgil", "Morgan"];
    for lawdog in lawdogs {
        greet(lawdog);
    }
}

fn find_greater(first: u8, second: u8) -> u8 {
    if first > second {
        first
    } else {
        second
    }
}

fn print_original(original: &str) {
    println!("original: {original}");
}

fn change_original(original: &mut String) {
    let next = original;
    *next = String::from("next value");
}

fn example_closure() {
    let airline = "Southwest";

    let write_msg = |greeting: String| println!("{greeting} {airline}.");

    write_msg(String::from("Welcome to"));
}

fn handle_error() {
    let filename = "updates.json";
    match File::open(filename) {
        Ok(file) => {
            println!("{file:#?}");
        }
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create(filename) {
                Ok(file) => {
                    println!("File {file:#?} created");
                }
                Err(error) => {
                    println!("{error:#?}");
                }
            },
            _ => {
                println!("{error:#?}");
            }
        },
    }
}

fn read_file(filename: &str) -> Result<String, Error> {
    let mut file_handle = File::open(filename)?;
    let mut file_data = String::new();
    file_handle.read_to_string(&mut file_data)?;
    Ok(file_data)
}

fn propigate_error() {
    let filename = "customer.json";
    let file_data = read_file(filename);
    match file_data {
        Ok(data) => {
            println!("{data}");
        }
        Err(_) => {
            println!("Oops! Cannot read {filename}.");
        }
    }
}

fn data_structures() {
    impl Segment {
        fn new(start: Waypoint, end: Waypoint) -> Self {
            Self { start, end }
        }

        fn distance(&self) -> f64 {
            const EARTH_RADIUS_IN_KILOMETERS: f64 = 6371.0;
            let start_name = &self.start.name;
            let end_name = &self.end.name;
            println!("Calculating distance between {start_name} & {end_name}...");
            let start_radians = self.start.latitude.to_radians();
            let end_radians = self.end.latitude.to_radians();
            let delta_latitude = (self.start.latitude - self.end.latitude).to_radians();
            let delta_longitude = (self.start.longitude - self.end.longitude).to_radians();
            let inner_central_angle = f64::powi((delta_latitude / 2.0).sin(), 2)
                + start_radians.cos()
                    * end_radians.cos()
                    * f64::powi((delta_longitude / 2.0).sin(), 2);
            let central_angle = 2.0 * inner_central_angle.sqrt().asin();
            EARTH_RADIUS_IN_KILOMETERS * central_angle
        }
    }

    let kcle = Waypoint {
        name: "KCLE".to_string(),
        latitude: 41.4075,
        longitude: -81.851_111,
    };

    let kslc = Waypoint {
        name: "KSLC".to_string(),
        latitude: 40.7861,
        longitude: -111.9822,
    };

    let kcle_kslc = Segment::new(kcle, kslc);
    let distance = kcle_kslc.distance();
    println!("{distance:.1}");
}

fn example_trait() {
    struct Boeing {
        required_crew: u8,
        range: u16,
    }

    struct Airbus {
        required_crew: u8,
        range: u16,
    }

    trait Flight {
        fn is_legal(
            &self,
            required_crew: u8,
            available_crew: u8,
            range: u16,
            distance: u16,
        ) -> bool;
    }

    impl Flight for Boeing {
        fn is_legal(
            &self,
            required_crew: u8,
            available_crew: u8,
            range: u16,
            distance: u16,
        ) -> bool {
            (available_crew >= required_crew) && (range + 150 > distance)
        }
    }

    impl Flight for Airbus {
        fn is_legal(
            &self,
            required_crew: u8,
            available_crew: u8,
            range: u16,
            distance: u16,
        ) -> bool {
            (available_crew >= required_crew) && (range + 280 > distance)
        }
    }

    let boeing = Boeing {
        required_crew: 4,
        range: 2500,
    };

    let airbus = Airbus {
        required_crew: 7,
        range: 2200,
    };

    let boeing_is_legal = boeing.is_legal(boeing.required_crew, 18, boeing.range, 2385);

    let airbus_is_legal = airbus.is_legal(airbus.required_crew, 3, airbus.range, 1863);

    println!("{boeing_is_legal}, {airbus_is_legal}");
}

fn example_vectors() {
    let mut flights: Vec<&str> = vec!["STL to KC departs at 06:00", "STL to CHI departs at 06:30"];
    flights.insert(1, "STL to SFO departs at 6:15");
    flights.remove(0);
    for flight in flights {
        println!("{flight}");
    }
}
