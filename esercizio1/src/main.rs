
pub mod razioinali {
    use std::collections::HashMap;

    /// Verifica se due stringhe sono anagrammi.
    ///
    /// Un anagramma è una parola o frase ottenuta riarrangiando le lettere di un'altra, utilizzando tutte le lettere originali esattamente una volta.
    ///
    /// # Argomenti
    ///
    /// * `str1`: La prima stringa da confrontare.
    /// * `str2`: La seconda stringa da confrontare.
    ///
    /// # Esempio
    ///
    /// ```
    /// let risultato = sono_anagrammi("listen", "silent");
    /// assert_eq!(risultato, true);
    /// ```
    ///
    /// # Ritorna
    ///doc
    /// Restituisce true se le stringhe sono anagrammi, altrimenti false.
    pub fn sono_anagrammi(str1: &str, str2: &str) -> bool {
        if str1.len() != str2.len() {
            return false; // Se le lunghezze delle due stringhe sono diverse, non possono essere anagrammi
        }

        let mut char_count = HashMap::new(); // Crea una nuova HashMap per contare i caratteri

        // Conta i caratteri nella prima stringa
        for c in str1.chars() {
            let _p = *char_count.entry(c).or_insert(0); // Si accede al valore con chiave c se la chiave non esiste allora la inserisce e segna il numero di utilizzi a 0
            if let Some(val) = char_count.get_mut(&c) {
                *val = *val + 1;
            }
        }

        // Sottrae i caratteri della seconda stringa
        for c in str2.chars() {
            if let Some(val) = char_count.get_mut(&c) { // Ottiene il riferimento al conteggio del carattere dalla HashMap
                *val -= 1; // Decrementa il conteggio del carattere
                if *val < 0 { // Se il conteggio diventa negativo, significa che ci sono più occorrenze del carattere nella seconda stringa rispetto alla prima
                    return false; // Quindi non sono anagrammi
                }
            } else {
                return false; // Se il carattere non esiste nella prima stringa, non possono essere anagrammi
            }
        }

        // Verifica che tutti i conteggi siano tornati a 0 , (internet dice che si fa così)
        char_count.values().all(|&count| count == 0) // Restituisce true se tutti i conteggi sono tornati a 0, altrimenti false
    }
}
fn main() {
}

#[cfg(test)]
mod tests {
    use crate::razioinali::sono_anagrammi;

    #[test]
    fn test_sono_anagrammi_anagrammi() {
        assert_eq!(sono_anagrammi("listen", "silent"), true);
        assert_eq!(sono_anagrammi("debit card", "bad credit"), true);
        assert_eq!(sono_anagrammi("rail safety", "fairy tales"), true);
    }

    #[test]
    fn test_sono_anagrammi_non_anagrammi() {
        assert_eq!(sono_anagrammi("hello", "world"), false);
        assert_eq!(sono_anagrammi("abc", "def"), false);
        assert_eq!(sono_anagrammi("rust", "trust"), false);
    }

    #[test]
    fn test_sono_anagrammi_stringhe_diverse_lunghezza() {
        assert_eq!(sono_anagrammi("hello", "helloworld"), false);
        assert_eq!(sono_anagrammi("abc", "abcd"), false);
    }
}