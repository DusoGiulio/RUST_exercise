/// Modulo per la gestione dei numeri razionali.
pub mod razionali {
    use std::ops::{Add, Mul};

    /// Struttura per rappresentare numeri razionali al suo interno ci sono un numeratore e un denumeratore entrambi di tipo i32.
    #[derive(Debug, PartialEq)]
    pub struct Razionali {
        num: i32,   // Numeratore
        denum: i32, // Denominatore
    }

    impl Razionali {
        /// Crea un nuovo numero razionale con il numeratore e il denominatore specificati.
        ///
        /// # Argomenti
        ///
        /// * `num`: Il numeratore del numero razionale.
        /// * `denum`: Il denominatore del numero razionale.
        ///
        /// # Esempio
        ///
        /// ```
        /// let numero = Razionali::new(3, 4);
        /// ```
        pub fn new(num: i32, denum: i32) -> Self {
            Razionali { num, denum }
        }

        /// Restituisce il numeratore del numero razionale.
        ///
        /// # Esempio
        ///
        /// ```
        /// let numero = Razionali::new(3, 4);
        /// assert_eq!(numero.num(), 3);
        /// ```
        pub fn num(&self) -> i32 {
            self.num
        }

        /// Restituisce il denominatore del numero razionale.
        ///
        /// # Esempio
        ///
        /// ```
        /// let numero = Razionali::new(3, 4);
        /// assert_eq!(numero.denum(), 4);
        /// ```
        pub fn denum(&self) -> i32 {
            self.denum
        }

        /// Restituisce la somma di due numeri razionali.
        ///
        /// # Argomenti
        ///
        /// * `other`: Il secondo numero razionale da sommare.
        ///
        /// # Esempio
        ///
        /// ```
        ///
        /// let num1 = Razionali::new(1, 2);
        /// let num2 = Razionali::new(1, 3);
        /// let risultato = num1.somma(&num2);
        /// assert_eq!(risultato, Razionali::new(5, 6));
        /// ```
        pub fn somma(&self, other: &Razionali) -> Razionali {
            let nuovo_num = self.num * other.denum + other.num * self.denum;
            let nuovo_denum = self.denum * other.denum;
            Razionali::minterm(nuovo_num, nuovo_denum)
        }

        /// Restituisce il prodotto di due numeri razionali.
        ///
        /// # Argomenti
        ///
        /// * `other`: Il secondo numero razionale da moltiplicare.
        ///
        /// # Esempio
        ///
        /// ```
        /// let num1 = Razionali::new(1, 2);
        /// let num2 = Razionali::new(2, 3);
        /// let risultato = num1.prodotto(&num2);
        /// assert_eq!(risultato, Razionali::new(1, 3));
        /// ```
        pub fn prodotto(&self, other: &Razionali) -> Razionali {
            let nuovo_num = self.num * other.num;
            let nuovo_denum = self.denum * other.denum;
            Razionali::minterm(nuovo_num, nuovo_denum)
        }

        // Funzione per semplificare il numero razionale
        fn minterm(num: i32, denum: i32) -> Razionali {
            let mcd = Razionali::mcd(num.abs(), denum.abs());
            Razionali {
                num: num / mcd,
                denum: denum / mcd,
            }
        }

        // Funzione per calcolare il massimo comune divisore
        fn mcd(a: i32, b: i32) -> i32 {
            if b == 0 {
                a
            } else {
                Razionali::mcd(b, a % b)
            }
        }
    }

    impl Add for Razionali {
        type Output = Self;

        /// Restituisce la somma di due numeri razionali.
        ///
        /// # Argomenti
        ///
        /// * `rhs`: Il secondo numero razionale da sommare.
        ///
        /// # Esempio
        ///
        /// ```
        /// let num1 = Razionali::new(1, 2);
        /// let num2 = Razionali::new(1, 3);
        /// let risultato = num1 + num2;
        /// assert_eq!(risultato, Razionali::new(5, 6));
        /// ```
        fn add(self, rhs: Self) -> Self::Output {
            let mcd = self.denum * rhs.denum / Razionali::mcd(self.denum, rhs.denum);
            let num = self.num * (mcd / self.denum) + rhs.num * (mcd / rhs.denum);
            Razionali::minterm(num, mcd)
        }
    }

    impl Add<i32> for Razionali {
        type Output = Self;

        /// Restituisce la somma di un numero razionale e un i32.
        ///
        /// # Argomenti
        ///
        /// * `rhs`: L'intero da sommare al numeratore del numero razionale.
        ///
        /// # Esempio
        ///
        /// ```
        /// let num1 = Razionali::new(1, 2);
        /// let risultato = num1 + 3;
        /// assert_eq!(risultato, Razionali::new(7, 2));
        /// ```
        fn add(self, rhs: i32) -> Self::Output {
            Razionali::new(self.num + (rhs * self.denum), self.denum)
        }
    }

    impl Mul for Razionali {
        type Output = Self;

        /// Restituisce il prodotto di due numeri razionali.
        ///
        /// # Argomenti
        ///
        /// * `rhs`: Il secondo numero razionale da moltiplicare.
        ///
        /// # Esempio
        ///
        /// ```
        /// let num1 = Razionali::new(1, 2);
        /// let num2 = Razionali::new(2, 3);
        /// let risultato = num1 * num2;
        /// assert_eq!(risultato, Razionali::new(1, 3));
        /// ```
        fn mul(self, rhs: Self) -> Self::Output {
            let num = self.num * rhs.num;
            let denum = self.denum * rhs.denum;
            Razionali::minterm(num, denum)
        }
    }

    impl Mul<i32> for Razionali {
        type Output = Self;

        /// Restituisce il prodotto di un numero razionale e un i32.
        ///
        /// # Argomenti
        ///
        /// * `rhs`: L'intero da moltiplicare per il numeratore del numero razionale.
        ///
        /// # Esempio
        ///
        /// ```
        /// let num1 = Razionali::new(1, 2);
        /// let risultato = num1 * 3;
        /// assert_eq!(risultato, Razionali::new(3, 2));
        /// ```
        fn mul(self, rhs: i32) -> Self::Output {
            Razionali::new(self.num * rhs, self.denum)
        }
    }
}
