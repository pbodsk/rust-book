#[derive(Debug)]
pub enum Toast {
    Rye,
    Wheat
}
pub struct Breakfast {
    pub toast: Toast,
    seasonal_fruit: String
}

impl Breakfast {
    pub fn summer(toast: Toast) -> Breakfast {
        Breakfast {
            toast: toast,
            seasonal_fruit: String::from("peaches"),
        }
    }
}