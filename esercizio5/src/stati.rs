/// Modulo `stati` per la gestione degli stati dei conti bancari.
pub mod stati {
    /// Definisce il comportamento di uno stato del conto bancario.
    pub trait State {
        /// Deposita una certa quantità di denaro nel conto.
        fn deposita(&self, saldo: &mut f64, amount: f64);

        /// Preleva una certa quantità di denaro dal conto.
        fn preleva(&self, saldo: &mut f64, amount: f64);

        /// Calcola e aggiunge gli interessi al saldo del conto.
        fn paga_interessi(&self, saldo: &mut f64, interesse: f64);

        /// Restituisce lo stato attuale del conto.
        fn stato(&self) -> Stato;
    }

    /// Enumerazione degli stati possibili di un conto bancario.
    pub enum Stato {
        /// Stato "Rosso", che indica un saldo inferiore al limite inferiore.
        Rosso,

        /// Stato "Argento", che indica un saldo compreso tra il limite inferiore e il limite superiore.
        Argento,

        /// Stato "Oro", che indica un saldo superiore al limite superiore.
        Oro,
    }

    /// Implementazione dello stato "Rosso" del conto bancario.
    pub struct StatoRosso;

    impl State for StatoRosso {
        fn deposita(&self, saldo: &mut f64, amount: f64) {
            *saldo += amount;
        }

        fn preleva(&self, _: &mut f64, _: f64) {}

        fn paga_interessi(&self, _: &mut f64, _: f64) {}

        fn stato(&self) -> Stato {
            Stato::Rosso
        }
    }

    /// Implementazione dello stato "Argento" del conto bancario.
    pub struct StatoArgento;

    impl State for StatoArgento {
        fn deposita(&self, saldo: &mut f64, amount: f64) {
            *saldo += amount;
        }

        fn preleva(&self, saldo: &mut f64, amount: f64) {
            if *saldo - amount >= 0.0 {
                *saldo -= amount;
            }
        }

        fn paga_interessi(&self, _: &mut f64, _: f64) {}

        fn stato(&self) -> Stato {
            Stato::Argento
        }
    }

    /// Implementazione dello stato "Oro" del conto bancario.
    pub struct StatoOro;

    impl State for StatoOro {
        fn deposita(&self, saldo: &mut f64, amount: f64) {
            *saldo += amount;
        }

        fn preleva(&self, saldo: &mut f64, amount: f64) {
            if *saldo - amount >= 0.0 {
                *saldo -= amount;
            }
        }

        fn paga_interessi(&self, saldo: &mut f64, interesse: f64) {
            *saldo += *saldo * interesse;
        }

        fn stato(&self) -> Stato {
            Stato::Oro
        }
    }
}
