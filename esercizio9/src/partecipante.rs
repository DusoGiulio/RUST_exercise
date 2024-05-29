/// Questo modulo fornisce una struttura `Partecipante` per gestire il comportamento di un partecipante all'asta.
pub mod partecipante {

    use std::sync::mpsc::{Receiver, Sender};
    use std::sync::{Arc, Mutex};
    use rand::Rng;
    use crate::banditore::banditore::*;
    use crate::banditore::banditore::Partecipazione::Partecipa;

    /// Struttura che rappresenta un partecipante all'asta.
    pub struct Partecipante {
        tx_banditore: Sender<FromTx>,      // invio al banditore i messaggi
        rx_partecipante: Receiver<FromRx>, // ricevo dal banditore il messaggio
        id: usize,                          // identificatore univoco del partecipante
        memoria_condivisa: Arc<Mutex<FromTx>>, // zona di memoria condivisa
    }

    impl Partecipante {
        /// Crea una nuova istanza di `Partecipante`.
        pub fn new(tx_banditore: Sender<FromTx>, rx_partecipante: Receiver<FromRx>, id: usize, memoria_condivisa: Arc<Mutex<FromTx>>) -> Self {
            Partecipante {
                tx_banditore,
                rx_partecipante,
                id,
                memoria_condivisa,
            }
        }

        /// Avvia il comportamento del partecipante.
        pub fn avvio(&self) {
            let init_name = String::new();
            let id = self.id.clone();

            // Comunicazione della presenza
            self.tx_banditore.clone().send(FromTx::new(id, Partecipa(0.0), init_name)).unwrap();
            let mut pausa = false;
            loop {
                if pausa {
                    attesa_fine(self);
                }
                let mex;
                loop {
                    if let Ok(messaggio) = self.rx_partecipante.recv() {
                        mex = messaggio;
                        break; // Esci dal loop una volta ricevuto il messaggio
                    }
                }
                let nome = mex.get_nome_prodotto();
                let mut prezzo = mex.get_prezzo();

                let mut rng = rand::thread_rng();
                let max = rng.gen_range(80.0..=120.0); // definisce l'aggressività del partecipante
                let random_float: f64 = rng.gen_range(1.0..=max);

                if random_float < 50.0 {
                    println!("\tIo partecipante numero {} non partecipo più", self.id.clone());
                    // Comunicazione di partecipazione errata
                    self.tx_banditore.clone().send(FromTx::new(id, Partecipazione::Err, nome.clone())).unwrap();
                    pausa = true;
                } else {
                    // Aggiornamento del prezzo e comunicazione di partecipazione
                    prezzo += random_float;
                    println!("\tIo partecipante numero {} offro {}", self.id.clone(), prezzo.clone());
                    self.tx_banditore.clone().send(FromTx::new(id, Partecipa(prezzo), nome.clone())).unwrap();
                }
            }
        }
    }

    /// Funzione ausiliaria per l'attesa del termine dell'asta.
    pub fn attesa_fine(x: &Partecipante) {
        let mut mex;
        let bind = x.memoria_condivisa.clone();

        loop {
            if let Ok(messaggio) = x.rx_partecipante.recv() {
                mex = messaggio;
                if mex.get_nome_prodotto().clone().eq(&String::from("Fine")) {
                    let binding = bind.lock().unwrap();
                    let name_product = binding.get_nome_prodotto();
                    let id_winner = binding.get_id();
                    let prezzo_winner = binding.get_partecipazione().ottieni_valore();
                    if id_winner == x.id.clone() {
                        println!("\tHo vinto l'oggetto {} pagandolo {} , il mio id è {}", name_product.clone(), prezzo_winner.clone(), id_winner.clone());
                        break;
                    }
                } else if mex.get_nome_prodotto().eq(&String::from("Nessuno")) {
                    let binding = bind.lock().unwrap();
                    let name_product = binding.get_nome_prodotto();
                    println!("L'asta per l'oggetto {} non ha superato il valore di riserva, nessuno ha vinto", name_product.clone());
                    break;
                }
            }
        }
    }
}
