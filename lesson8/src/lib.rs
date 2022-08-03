#[derive(Debug)]
struct Rectangle {
    width: i32,
    height: i32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn larger_can_hold_smaller () {
        let large = Rectangle {
            width: 100,
            height: 100,
        };

        let small = Rectangle {
            width: 10,
            height: 10,
        };
        
        assert!(large.can_hold(&small));
    }
}