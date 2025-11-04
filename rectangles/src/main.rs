/// Documentation comment
struct Rectangle<T> {
    width: T,
    height: T,
}

impl<T: num::Num + Copy> Rectangle<T> {
    /// This function calculates the area of a [`Rectangle`] and returns the result as a u32.
    fn area(&self) -> T {
        self.width * self.height
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30.0,
        height: 60.0,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calculate_area() {
        let rect1 = Rectangle {
            width: 30,
            height: 60,
        };

        assert_eq!(rect1.area(), 1800);
    }
}
