/// Modulo `conto_bancario` per la gestione dei conti bancari.
pub mod conto_bancario {
    use crate::stati::stati::*;

    /// Rappresenta un conto bancario con le sue caratteristiche.
    pub struct ContoBancario {
        /// Nome del cliente associato al conto.
        pub nome_cliente: String,
        /// Saldo attuale del conto.
        pub saldo: f64,
        /// Limite inferiore del saldo del conto.
        pub limite_inferiore: f64,
        /// Limite superiore del saldo del conto.
        pub limite_superiore: f64,
        /// Tasso di interesse applicato al conto.
        pub interesse: f64,
        /// Stato attuale del conto (implementa il pattern State).
        pub stato: Box<dyn State>,
    }

    impl ContoBancario {
        /// Crea un nuovo conto bancario con i parametri specificati.
        ///
        /// # Argomenti
        ///
        /// * `nome_cliente` - Nome del cliente associato al conto.
        /// * `saldo` - Saldo iniziale del conto.
        /// * `limite_inferiore` - Limite inferiore del saldo del conto.
        /// * `limite_superiore` - Limite superiore del saldo del conto.
        /// * `interesse` - Tasso di interesse applicato al conto.
        ///
        /// # Panico
        ///
        /// Questo metodo panica se il saldo è inferiore al limite inferiore specificato.
        ///
        /// # Esempio
        ///
        /// ```
        /// use conto_bancario::ContoBancario;
        ///
        /// let conto = ContoBancario::new(String::from("Mario Rossi"), 1000.0, 0.0, 5000.0, 0.03);
        /// ```
        pub fn new(nome_cliente: String, saldo: f64, limite_inferiore: f64, limite_superiore: f64, interesse: f64) -> Self {

            ContoBancario {
                nome_cliente,
                saldo,
                limite_inferiore,
                limite_superiore,
                interesse,
                stato: {
                    if saldo < limite_inferiore {
                        Box::new(StatoRosso)
                    } else if saldo < limite_superiore {
                        Box::new(StatoArgento)
                    } else {
                        Box::new(StatoOro)
                    }
                }
            }
        }
    }
}