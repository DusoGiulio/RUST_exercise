#[cfg(test)]
mod tests {
    use esercizio7::double_link::double_link::DoublyPointedList;

    #[derive(Debug, PartialEq, Clone, Default)]
    struct Punto {
        x: i32,
        y: i32,
    }

    #[test]
    fn test_get() {
        let mut list: DoublyPointedList<Punto> = DoublyPointedList::new();

        list.push_back(Punto { x: 1, y: 2 });
        list.push_back(Punto { x: 3, y: 4 });
        list.push_back(Punto { x: 5, y: 6 });
        list.push_back(Punto { x: 7, y: 8 });

        // Verifica che l'elemento alla posizione 2 sia corretto
        matches!(list.get(2), Some(Punto { x: 5, y: 6 }));

        // Rimuove l'elemento alla posizione 2
        let removed = list.get(2);
        matches!(removed, Some(Punto { x: 5, y: 6 }));

        // Verifica che la lista sia stata correttamente modificata
        assert_eq!(list.len(), 3);
        matches!(list.get(2), Some(Punto { x: 7, y: 8 }));

        // Verifica il comportamento per un indice non valido
        assert_eq!(list.get(10), None);
    }
}
