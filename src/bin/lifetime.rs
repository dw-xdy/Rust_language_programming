// #[derive(Debug)]
// struct City {
//     name: &'static str, // ⚠️
//     date_founded: u32,
// }
//
// fn main() {
//     let my_city = City {
//         name: "Ichinomiya",
//         date_founded: 1921,
//     };
//
//     println!("{:?}", my_city);
//     println!("{:?}", my_city.name);
//     println!("{:?}", my_city.date_founded);
// }

// #[derive(Debug)]
// struct City {
//     name: &'static str, // must live for the whole program
//     date_founded: u32,
// }
//
// fn main() {
//     let city_names = vec!["Ichinomiya".to_string(), "Kurume".to_string()]; // city_names does not live for the whole program
//
//     let my_city = City {
//         name: &city_names[0], // ⚠️ This is a &str, but not a &'static str. It is a reference to a value inside city_names
//         date_founded: 1921,
//     };
//
//     println!("{} was founded in {}", my_city.name, my_city.date_founded);
// }

#[derive(Debug)]
struct City<'a> { // City has lifetime 'a,
    // we can use anything to replace a, just like generics we usually use <T> and <U>
    name: &'a str, // and name also has lifetime 'a.
    date_founded: u32,
}

fn main() {
    let city_names = vec!["Ichinomiya".to_string(), "Kurume".to_string()];

    let my_city = City {
        name: &city_names[0],
        date_founded: 1921,
    };

    println!("{} was founded in {}", my_city.name, my_city.date_founded);
}
