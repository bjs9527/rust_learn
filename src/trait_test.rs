struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
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
    #[test]
    fn test2() {
        let p = Point { x: 5, y: 10 };
        assert_eq!(*p.x(), 5);
    }

    #[test]
    fn test3() {
        struct Point<X1, Y1> {
            x: X1,
            y: Y1,
        }

        impl<X1, Y1> Point<X1, Y1> {
            fn mixup<X2, Y2>(self, other: Point<X2, Y2>) -> Point<X1, Y2> {
                Point {
                    x: self.x,
                    y: other.y,
                }
            }
        }

        let p1 = Point { x: 5, y: 10.4 };
        let p2 = Point { x: "Hello", y: 'c' };

        let p3 = p1.mixup(p2);

        assert_eq!(p3.x, 5);
        assert_eq!(p3.y, 'c');
    }
    

}
