/// Questo modulo fornisce strutture per contare il numero di vocali in una stringa.
pub mod numero_vocali {
    use crate::numero_vocali::check_char;

    /// Rappresenta il numero di vocali conteggiate singolarmente.
    #[derive(Debug, PartialEq)]
    pub struct NumVocali {
        /// Numero di vocali 'a'.
        a: i32,
        /// Numero di vocali 'e'.
        e: i32,
        /// Numero di vocali 'i'.
        i: i32,
        /// Numero di vocali 'o'.
        o: i32,
        /// Numero di vocali 'u'.
        u: i32,
    }

    /// Rappresenta una tupla contenente il numero di vocali conteggiate singolarmente.
    #[derive(Debug, PartialEq)]
    pub struct TuplaVocali(i32, i32, i32, i32, i32);

    impl NumVocali {
        /// Crea una nuova istanza di `NumVocali`.
        ///
        /// # Parametri
        ///
        /// * `a`: Numero di vocali 'a'.
        /// * `e`: Numero di vocali 'e'.
        /// * `i`: Numero di vocali 'i'.
        /// * `o`: Numero di vocali 'o'.
        /// * `u`: Numero di vocali 'u'.
        ///
        /// # Ritorno
        ///
        /// Una nuova istanza di `NumVocali`.
        pub fn new(a: i32, e: i32, i: i32, o: i32, u: i32) -> Self {
            NumVocali { a, e, i, o, u }
        }

        /// Conta il numero di vocali nella stringa specificata e restituisce un'istanza di `NumVocali`.
        ///
        /// # Parametri
        ///
        /// * `s`: La stringa in cui contare le vocali.
        ///
        /// # Ritorno
        ///
        /// Una nuova istanza di `NumVocali` contenente il conteggio delle vocali nella stringa.
        pub fn num_vocali_struct(s: &String) -> NumVocali {
            let n_vocali = s.chars().fold((0, 0, 0, 0, 0), |acc, c| { check_char(&c, &acc) });
            Self::new(n_vocali.0, n_vocali.1, n_vocali.2, n_vocali.3, n_vocali.4)
        }
    }

    impl TuplaVocali {
        /// Crea una nuova istanza di `TuplaVocali`.
        ///
        /// # Parametri
        ///
        /// * `a`: Numero di vocali 'a'.
        /// * `e`: Numero di vocali 'e'.
        /// * `i`: Numero di vocali 'i'.
        /// * `o`: Numero di vocali 'o'.
        /// * `u`: Numero di vocali 'u'.
        ///
        /// # Ritorno
        ///
        /// Una nuova istanza di `TuplaVocali`.
        pub fn new(a: i32, e: i32, i: i32, o: i32, u: i32) -> Self {
            TuplaVocali(a, e, i, o, u)
        }

        /// Conta il numero di vocali nella stringa specificata e restituisce un'istanza di `TuplaVocali`.
        ///
        /// # Parametri
        ///
        /// * `s`: La stringa in cui contare le vocali.
        ///
        /// # Ritorno
        ///
        /// Una nuova istanza di `TuplaVocali` contenente il conteggio delle vocali nella stringa.
        pub fn num_vocali_tuple(s: &String) -> TuplaVocali {
            let n_vocali = s.chars().fold((0, 0, 0, 0, 0), |acc, c| { check_char(&c, &acc) });
            Self::new(n_vocali.0, n_vocali.1, n_vocali.2, n_vocali.3, n_vocali.4)
        }
    }
}

/// Controlla se il carattere specificato è una vocale e aggiorna il conteggio.
///
/// # Parametri
///
/// * `c`: Il carattere da controllare.
/// * `acc`: Il conteggio corrente delle vocali.
///
/// # Ritorno
///
/// Una tupla con il conteggio aggiornato delle vocali.
fn check_char(c: &char, acc: &(i32, i32, i32, i32, i32)) -> (i32, i32, i32, i32, i32) {
    match c.to_ascii_lowercase() {
        'a' => (acc.0 + 1, acc.1, acc.2, acc.3, acc.4),
        'e' => (acc.0, acc.1 + 1, acc.2, acc.3, acc.4),
        'i' => (acc.0, acc.1, acc.2 + 1, acc.3, acc.4),
        'o' => (acc.0, acc.1, acc.2, acc.3 + 1, acc.4),
        'u' => (acc.0, acc.1, acc.2, acc.3, acc.4 + 1),
        _ => *acc,
    }
}
