pub mod double_link {
    use std::{cell::RefCell, rc::Rc};
    /// Rappresenta un nodo della lista doppiamente linkata.
    #[derive(Default)]
    pub struct Node<T: Copy> {
        /// Elemento memorizzato nel nodo.
        item: T,
        /// Puntatore al nodo successivo.
        next: Pointer<T>,
        /// Puntatore al nodo precedente.
        prev: Pointer<T>,
    }
    /// Crea un nuovo nodo con l'elemento specificato.
    ///
    /// # Argomenti
    ///
    /// * `item`: Elemento da memorizzare nel nodo.
    impl<T: Copy> Node<T> {
        pub fn new(item: T) -> Self {
            Node { item, next: None, prev: None }
        }
    }
    /// Tipo alias per un puntatore a un nodo.
    type Pointer<T> = Option<Rc<RefCell<Node<T>>>>;
    /// Rappresenta una lista doppiamente linkata.
    #[derive(Default)]
    pub struct DoublyPointedList<T: Copy> {
        /// Puntatore alla testa della lista.
        head: Pointer<T>,
        /// Puntatore alla coda della lista.
        tail: Pointer<T>,
        /// Dimensione della lista
        size: usize,
    }

    impl<T: Copy> DoublyPointedList<T> {
        /// Crea una nuova lista doppiamente linkata vuota.
        pub fn new() -> Self {
            DoublyPointedList { head: None, tail: None, size: 0 }
        }
        /// Verifica se la lista è vuota.
        ///
        /// # Restituisce
        ///
        /// Restituisce `true` se la lista è vuota, altrimenti `false`.
        pub fn is_empty(&self) -> bool {
            self.size == 0
        }
        /// Restituisce la lunghezza della lista.
        ///
        /// # Restituisce
        ///
        /// Restituisce il numero di elementi presenti nella lista.
        pub fn len(&self) -> usize {
            self.size
        }
        /// Aggiunge un elemento alla fine della lista.
        ///
        /// # Argomenti
        ///
        /// * `item`: Elemento da aggiungere alla lista.
        pub fn push_back(&mut self, item: T) {
            let new_node = Rc::new(RefCell::new(Node::new(item)));
            if self.is_empty() {
                self.head = Some(Rc::clone(&new_node));
            } else {
                let old_tail = self.tail.take();
                if let Some(old_tail) = old_tail {
                    old_tail.borrow_mut().next = Some(Rc::clone(&new_node));
                    new_node.borrow_mut().prev = Some(Rc::clone(&old_tail));
                }
            }
            self.tail = Some(new_node);
            self.size += 1;
        }
        /// Aggiunge un elemento all'inizio della lista.
        ///
        /// # Argomenti
        ///
        /// * `item`: Elemento da aggiungere alla lista.
        pub fn push_front(&mut self, item: T) {
            let new_node = Rc::new(RefCell::new(Node::new(item)));
            if self.is_empty() {
                self.tail = Some(Rc::clone(&new_node));
            } else {
                let old_head = self.head.take();
                if let Some(old_head) = old_head {
                    old_head.borrow_mut().prev = Some(Rc::clone(&new_node));
                    new_node.borrow_mut().next = Some(Rc::clone(&old_head));
                }
            }
            self.head = Some(new_node);
            self.size += 1;
        }
        /// Rimuove e restituisce l'elemento alla fine della lista.
        ///
        /// # Restituisce
        ///
        /// Restituisce l'elemento rimosso, se presente.
        pub fn pop_back(&mut self) -> Option<T> {
            if self.is_empty() {
                return None;
            }

            let old_tail = self.tail.take()?;
            self.size -= 1;

            if let Some(precedente) = old_tail.borrow_mut().prev.take() {
                precedente.borrow_mut().next = None;
                self.tail = Some(precedente);
            } else {
                self.head = None;
            }
            Some(old_tail.clone().borrow_mut().item)
        }
        /// Rimuove e restituisce l'elemento all'inizio della lista.
        ///
        /// # Restituisce
        ///
        /// Restituisce l'elemento rimosso, se presente.
        pub fn pop_front(&mut self) -> Option<T> {
            if self.is_empty() {
                return None;
            }

            let old_head = self.head.take()?;
            self.size -= 1;

            if let Some(next) = old_head.borrow_mut().next.take() {
                next.borrow_mut().prev = None;
                self.head = Some(next);
            } else {
                self.tail = None;
            }

            Some(old_head.clone().borrow_mut().item)
        }
        /// Restituisce l'elemento in posizione `n` dalla testa della lista.
        ///
        /// `n` deve essere un numero positivo o negativo. Se `n` è positivo,
        /// l'elemento sarà `n` posizioni dopo la testa della lista. Se `n` è
        /// negativo, l'elemento sarà `n` posizioni prima della coda della lista.
        ///
        /// # Argomenti
        ///
        /// * `n`: Indice dell'elemento da recuperare dalla lista. Deve essere un numero intero.
        ///
        /// # Restituisce
        ///
        /// Restituisce l'elemento in posizione `n`, se presente.
        pub fn get(&self, n: i32) -> Option<T> {
            if n >= 0 {
                let mut ennesimo = self.head.clone();
                for _ in 0..n {
                    if let Some(node) = ennesimo {
                        ennesimo = node.borrow().next.clone();
                    } else {
                        return None;
                    }
                }
                ennesimo.map(|node| node.borrow().item)
            } else {
                let mut ennesimo = self.tail.clone();
                for _ in 0..(-n - 1) {
                    if let Some(node) = ennesimo {
                        ennesimo = node.borrow().prev.clone();
                    } else {
                        return None;
                    }
                }
                ennesimo.map(|node| node.borrow().item)
            }
        }
    }
}
