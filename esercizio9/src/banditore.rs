
 pub mod banditore {

  use std::sync::{Arc, mpsc, Mutex};
  use std::sync::mpsc::{Receiver, Sender};
  use std::thread;
  use std::time::Duration;
  use rand::Rng;
  use crate::banditore::banditore::Partecipazione::Partecipa;
  use crate::partecipante::partecipante::Partecipante;

     /// Struttura che rappresenta il banditore dell'asta.
     pub struct Banditore {
         /// Canale di invio dei messaggi al banditore.
         tx_banditore: Sender<FromTx>,
         /// Canale di ricezione dei messaggi dai partecipanti.
         rx_banditore: Receiver<FromTx>,
         /// Vettore contenente i canali di invio dei messaggi ai partecipanti.
         tx_partecipanti: Vec<Sender<FromRx>>,
         /// Numero totale dei partecipanti all'asta.
         numero_partecipanti: usize,
         /// Descrizione del prodotto all'asta.
         descrizione_prodotto: String,
         /// Prezzo minimo richiesto per l'asta.
         prezzo_minimo: f64,
         /// Prezzo di riserva per l'asta.
         prezzo_riserva: f64,
         /// Memoria condivisa tra il banditore e i partecipanti.
         memoria_condivisa: Arc<Mutex<FromTx>>,
     }
     /// Struttura dei messaggi ricevuti dai partecipanti.
     pub struct FromTx {
         /// Identificatore univoco del partecipante.
         id: usize,
         /// Tipo di partecipazione del partecipante all'asta.
         partecipazione: Partecipazione,
         /// Nome del prodotto all'asta.
         nome_prodotto: String,
     }

     /// Struttura dei messaggi inviati ai partecipanti.
     pub struct FromRx {
         /// Nome del prodotto all'asta.
         nome_prodotto: String,
         /// Prezzo associato al prodotto.
         prezzo: f64,
     }
     /// Enumerator per il tipo di partecipazione dei partecipanti all'asta.
     pub enum Partecipazione {
         /// Indica un errore nella partecipazione all'asta.
         Err,
         /// Indica la partecipazione all'asta con un determinato valore.
         Partecipa(f64),
     }
///////////////////////implementazioni\\\\\\\\\\\\\\\\\\\\\\\\\\
impl FromTx {
    /// Crea una nuova istanza di `FromTx`.
    ///
    /// # Parametri
    ///
    /// - `id`: L'identificatore associato al partecipante.
    /// - `partecipazione`: La partecipazione all'asta.
    /// - `nome_prodotto`: Il nome del prodotto associato all'asta.
    ///
    /// # Ritorno
    ///
    /// Restituisce una nuova istanza di `FromTx`.
    pub fn new(id: usize, partecipazione: Partecipazione, nome_prodotto: String) -> Self {
        FromTx {
            id,
            partecipazione,
            nome_prodotto,
        }
    }

    /// Ottiene l'identificatore associato al partecipante.
    ///
    /// # Ritorno
    ///
    /// Restituisce l'identificatore associato al partecipante.
    pub fn get_id(&self) -> usize {
        self.id
    }

    /// Ottiene il riferimento alla partecipazione all'asta.
    ///
    /// # Ritorno
    ///
    /// Restituisce un riferimento alla partecipazione all'asta.
    pub fn get_partecipazione(&self) -> &Partecipazione {
        &self.partecipazione
    }

    /// Ottiene il riferimento al nome del prodotto associato all'asta.
    ///
    /// # Ritorno
    ///
    /// Restituisce un riferimento al nome del prodotto associato all'asta.
    pub fn get_nome_prodotto(&self) -> &String {
        &self.nome_prodotto
    }
}

     ///////////////////////
     impl Partecipazione {
         /// Ottiene il valore della partecipazione all'asta.
         ///
         /// Se la partecipazione è del tipo `Partecipa`, restituisce il valore associato.
         /// Se la partecipazione è del tipo `Err`, restituisce -1.
         ///
         /// # Ritorno
         ///
         /// Restituisce il valore della partecipazione.
         pub fn ottieni_valore(&self) -> f64 {
             match self {
                 Partecipa(valore) => *valore,
                 Partecipazione::Err => -1.0,
             }
         }

         /// Imposta il valore della partecipazione all'asta.
         ///
         /// Se la partecipazione è del tipo `Partecipa`, imposta il nuovo valore specificato.
         ///
         /// # Parametri
         ///
         /// - `nuovo_valore`: Il nuovo valore da impostare.
         pub fn imposta_valore(&mut self, nuovo_valore: f64) {
             if let Partecipa(ref mut valore) = self {
                 *valore = nuovo_valore;
             }
         }
     }

     ///////////////////////
     impl FromRx {
         /// Crea una nuova istanza di `FromRx` con il prezzo e il nome del prodotto specificati.
         ///
         /// # Parametri
         ///
         /// - `prezzo`: Il prezzo associato all'oggetto.
         /// - `nome_prodotto`: Il nome del prodotto.
         ///
         /// # Ritorno
         ///
         /// Restituisce una nuova istanza di `FromRx`.
         pub fn new(prezzo: f64, nome_prodotto: String) -> Self {
             FromRx { prezzo, nome_prodotto }
         }

         /// Ottiene il prezzo dell'oggetto.
         ///
         /// # Ritorno
         ///
         /// Restituisce il prezzo associato all'oggetto.
         pub fn get_prezzo(&self) -> f64 {
             self.prezzo
         }

         /// Ottiene il nome del prodotto.
         ///
         /// # Ritorno
         ///
         /// Restituisce il nome del prodotto associato all'oggetto.
         pub fn get_nome_prodotto(&self) -> &String {
             &self.nome_prodotto
         }
     }


     impl Banditore {
         /// Crea una nuova istanza di `Banditore` con i parametri specificati.
         ///
         /// # Parametri
         ///
         /// - `numero_partecipanti`: Il numero di partecipanti all'asta.
         /// - `descrizione_prodotto`: La descrizione del prodotto all'asta.
         /// - `prezzo_minimo`: Il prezzo minimo di partenza dell'oggetto all'asta.
         /// - `prezzo_riserva`: Il prezzo di riserva dell'oggetto all'asta.
         ///
         /// # Ritorno
         ///
         /// Restituisce una nuova istanza di `Banditore`.
         pub fn new(
             numero_partecipanti: usize,
             descrizione_prodotto: String,
             prezzo_minimo: f64,
             prezzo_riserva: f64,
         ) -> Self {
             let ch = mpsc::channel::<FromTx>();
             let mutex = Arc::new(Mutex::new(FromTx::new(0, Partecipa(prezzo_minimo.clone()), descrizione_prodotto.clone())));
             Banditore {
                 numero_partecipanti,
                 descrizione_prodotto,
                 prezzo_minimo,
                 prezzo_riserva,
                 memoria_condivisa: mutex,
                 tx_banditore: ch.0,
                 rx_banditore: ch.1,
                 tx_partecipanti: Vec::new(),
             }
         }

         /// Ottiene il canale di invio dei messaggi al banditore.
         ///
         /// # Ritorno
         ///
         /// Restituisce un riferimento al canale di invio dei messaggi al banditore.
         pub fn get_tx_banditore(&self) -> &Sender<FromTx> {
             &self.tx_banditore
         }

         /// Ottiene il canale di ricezione dei messaggi dal banditore.
         ///
         /// # Ritorno
         ///
         /// Restituisce un riferimento al canale di ricezione dei messaggi dal banditore.
         pub fn get_rx_banditore(&self) -> &Receiver<FromTx> {
             &self.rx_banditore
         }

         /// Ottiene il vettore dei canali di invio dei messaggi ai partecipanti.
         ///
         /// # Ritorno
         ///
         /// Restituisce un riferimento al vettore dei canali di invio dei messaggi ai partecipanti.
         pub fn get_tx_partecipanti(&self) -> &Vec<Sender<FromRx>> {
             &self.tx_partecipanti
         }

         /// Ottiene il numero di partecipanti all'asta.
         ///
         /// # Ritorno
         ///
         /// Restituisce il numero di partecipanti all'asta.
         pub fn get_numero_partecipanti(&self) -> usize {
             self.numero_partecipanti
         }

         /// Ottiene la descrizione del prodotto all'asta.
         ///
         /// # Ritorno
         ///
         /// Restituisce la descrizione del prodotto all'asta.
         pub fn get_descrizione_prodotto(&self) -> &String {
             &self.descrizione_prodotto
         }

         /// Ottiene il prezzo minimo di partenza dell'oggetto all'asta.
         ///
         /// # Ritorno
         ///
         /// Restituisce il prezzo minimo di partenza dell'oggetto all'asta.
         pub fn get_prezzo_minimo(&self) -> f64 {
             self.prezzo_minimo
         }
   /////////////////////////////////////////////////////////////////////////////
   ///Avvia l'asta e di conseguenza anche il numero di thread di partecipanti previsti per l'asta
   /// Implementa quindi tutto il protocollo dell'asta comunicando con i partecipanti, infine scrive nel campo memoria_condivisa del banditore il risultato dell'asta
   pub fn avvia_asta(mut self) ->(){
       println!("Avvio asta per articolo {}", self.get_descrizione_prodotto());
    let mut handles = Vec::new();
    let mut prezzo_attuale = self.get_prezzo_minimo().clone();
       let mut id_attuale:usize=0;
    let mut count =0;

    for n in 0..self.get_numero_partecipanti() {
     let tx_banditore_clone = self.get_tx_banditore().clone();

     let memoria_condivisa_clone = self.memoria_condivisa.clone();
     let ch = mpsc::channel::<FromRx>();
    //Salvo il tx di ogni partecipante
     self.tx_partecipanti.push(ch.0);
     //Avvio asta
     let handle = thread::spawn(move || {
      let p=Partecipante::new(tx_banditore_clone, ch.1, n, memoria_condivisa_clone);
      p.avvio();
     });
     handles.push(handle);
    }


    //Attendo che n partecipanti comunichino la loro presenza
    while count < self.get_numero_partecipanti().clone(){
        if self.get_rx_banditore().recv().unwrap().partecipazione.ottieni_valore() == 0.0{
            count+=1;
        }
    }
       println!("Tutti presenti,iniziamo!");
       //Invio a ogni partecipante il nome del prodotto e il suo prezzo di partenza

       for tx_partecipante in self.get_tx_partecipanti().clone(){
           let messaggio_da_inviare = FromRx::new(prezzo_attuale.clone(), self.get_descrizione_prodotto().clone());
           if let Err(_) = tx_partecipante.send(messaggio_da_inviare) {
               break;
           }
       }
    loop{
        let mut da_rimuovere= Vec::new();
        for _ in 0..self.numero_partecipanti.clone() {
            if let Ok(messaggio) = self.get_rx_banditore().recv() {
                let partecipazione = messaggio.get_partecipazione();
                let valore_partecipazione = partecipazione.ottieni_valore();

                if valore_partecipazione != -1.0 {
                    if prezzo_attuale < valore_partecipazione {
                        prezzo_attuale = valore_partecipazione;
                        id_attuale = messaggio.get_id().clone();
                    }
                } else {
                    // Aggiungi l'indice alla lista dei partecipanti da rimuovere
                       da_rimuovere.push(messaggio.get_id().clone());
                }
            }
        }
        self.numero_partecipanti=self.numero_partecipanti.clone()-da_rimuovere.len();

        //Se dopo aver aggiornato le offerte il vettore dei partecipanti resta vuoto allora l'asta è finita
        //per mancanza di partecipanti, quindi esco dal loop
        if self.numero_partecipanti == 0 {
            break;
        }else{
            //invio a tutti i partecipanti il nuovo prezzo attuale
            let mut rng = rand::thread_rng();
            let random_float: f64 = rng.gen_range(1.0..=50.0);
            prezzo_attuale+=random_float;
            println!("Il prezzo attuale è {} e il miglior offerente è {}", prezzo_attuale.clone(), id_attuale.clone());
            thread::sleep(Duration::from_secs(1));
            for tx in self.get_tx_partecipanti().iter() {
                if let Err(err) = tx.send(FromRx::new(prezzo_attuale.clone(), self.descrizione_prodotto.clone())) {
                    println!("Errore nell'invio del messaggio: {:?}", err);
                    thread::sleep(Duration::from_secs(1));
                }
            }
        }
    }
   let mut true_winner =true;
   if prezzo_attuale<self.prezzo_riserva.clone()
   {
    prezzo_attuale=self.prezzo_riserva.clone();
    id_attuale=self.numero_partecipanti.clone();
    true_winner=false;
   }
       self.memoria_condivisa.lock().unwrap().partecipazione.imposta_valore(prezzo_attuale.clone());
       self.memoria_condivisa.lock().unwrap().nome_prodotto=self.descrizione_prodotto.clone();
       self.memoria_condivisa.lock().unwrap().id=id_attuale;

   for tx in self.get_tx_partecipanti().clone().iter(){
       if true_winner{
           if let Err(err) = tx.send(FromRx::new(prezzo_attuale.clone(), String::from("Fine"))) {
               println!("Errore nell'invio del messaggio: {:?}", err);
           }
       }else{
           if let Err(err) = tx.send(FromRx::new(prezzo_attuale.clone(), String::from("Nessuno"))) {
               println!("Errore nell'invio del messaggio: {:?}", err);
           }
           break;
       }
       thread::sleep(Duration::from_millis(300));
   }
   }
  }
 }




