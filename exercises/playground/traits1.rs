pub trait Detail {
    fn description(&self) -> String;
    fn years_since_launched(&self) -> i32;
}

struct Car {
    brand_name: String,
    color: String,
    launched_year: i32,
}

impl Detail for Car {
    fn description(&self) -> String {
        return format!("I have a {} which is {} in color.", self.brand_name, self.color);
    }

    fn years_since_launched(&self) -> i32 {
        return 2021 - self.launched_year;
    }
}

//// You can't do this because the traits have the same function names
// pub trait Detail2 {
//     fn description(&self) -> String;
//     fn years_since_launched(&self) -> i32;
// }
//
// impl Detail2 for Car {
//     fn description(&self) -> String {
//         return format!("I have a {} which is {} in color. Parte dos!", self.brand_name, self.color);
//     }
//
//     fn years_since_launched(&self) -> i32 {
//         return 2021 - self.launched_year;
//     }
// }

fn main() {
    let car = Car {
        brand_name: "WagonR".to_string(),
        color: "Red".to_string(),
        launched_year: 1992,
    };

    let car2 = Car {
        brand_name: "Venue".to_string(),
        color: "White".to_string(),
        launched_year: 1997,
    };

    println!("{}", car.description());
    println!("The car was released {} years ago.\n", car.years_since_launched());

    println!("{}", car.description());
    println!("The car was released {} years ago.", car.years_since_launched());
}
