#[cfg(test)]
mod tests {
    use eserczio6::double_link::double_link::DoublyPointedList;
    #[derive(Debug,Copy,Clone,PartialEq)]
    struct Point {
        x: i32,
        y: i32
    }

    #[test]
    fn test_push_back_pop_front() {
        let mut list = DoublyPointedList::new();
        list.push_back(Point { x: 1, y: 2 });
        list.push_back(Point { x: 3, y: 4 });

        assert_eq!(list.pop_front(), Some(Point { x: 1, y: 2 }));
        assert_eq!(list.pop_front(), Some(Point { x: 3, y: 4 }));
        assert_eq!(list.pop_front(), None);
        assert_eq!(list.is_empty(), true);
    }

    #[test]
    fn test_push_front_pop_back() {
        let mut list = DoublyPointedList::new();
        list.push_front(Point { x: 1, y: 2 });
        list.push_front(Point { x: 3, y: 4 });

        assert_eq!(list.pop_back(), Some(Point { x: 1, y: 2 }));
        assert_eq!(list.pop_back(), Some(Point { x: 3, y: 4 }));
        assert_eq!(list.pop_back(), None);
        assert_eq!(list.is_empty(), true);
    }

    #[test]
    fn test_get() {
        let mut list = DoublyPointedList::new();
        list.push_back(Point { x: 1, y: 2 });
        list.push_back(Point { x: 3, y: 4 });
        list.push_back(Point { x: 5, y: 6 });

        assert_eq!(list.get(0), Some(Point { x: 1, y: 2 }));
        assert_eq!(list.get(-1), Some(Point { x: 5, y: 6 }));
        assert_eq!(list.get(1), Some(Point { x: 3, y: 4 }));
        assert_eq!(list.get(-2), Some(Point { x: 3, y: 4 }));
        assert_eq!(list.get(2), Some(Point { x: 5, y: 6 }));
        assert_eq!(list.get(-3), Some(Point { x: 1, y: 2 }));
        assert_eq!(list.get(3), None);
        assert_eq!(list.get(-4), None);
    }
}