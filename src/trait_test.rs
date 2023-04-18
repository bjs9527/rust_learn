struct Point<T> {
    x: T,
    y: T,
}

#[cfg(test)]
pub mod tests {
    use super::Point;
    #[test]
    fn test() {
        let point1 = Point { x: 1, y: 2 };
        let point2 = Point { x: "1", y: "2" };

        assert_eq!(point1.x, 1);
        assert_eq!(point2.x, "1");
    }
}
