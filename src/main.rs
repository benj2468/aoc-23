use std::path::Path;

#[derive(Clone)]
pub struct AOCInput(pub String);

impl<T> From<T> for AOCInput
where
    T: AsRef<Path>,
{
    fn from(value: T) -> Self {
        Self(
            std::fs::read_to_string(value)
                .expect("The elves have provided you with a bad file path"),
        )
    }
}

impl AsRef<String> for AOCInput {
    fn as_ref(&self) -> &String {
        &self.0
    }
}

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;

fn main() {
    println!("Hello, world!");
}
