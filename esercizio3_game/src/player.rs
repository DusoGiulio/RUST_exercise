use std::fmt;
use std::fmt::Formatter;
use rand::Rng;
use crate::game_space::game_space::CampoGioco;
use crate::game_space::game_space::Posizione;
use crate::game_space::game_space::Cella;

/// Rappresenta il giocatore con le sue caratteristiche.
pub struct Player {
    /// Numero di mosse disponibili.
    pub mosse: usize,
    /// Posizione attuale del giocatore.
    pub posizione: Posizione,
    /// Direzione attuale del giocatore.
    pub direzione: Direzione,
    /// Forza del giocatore.
    pub forza: i32,
}

/// Enumerazione delle direzioni possibili del giocatore.
#[derive(Clone, Copy)]
pub enum Direzione {
    Su,
    Giu,
    Destra,
    Sinistra,
}

impl Player {
    /// Crea e restituisce un nuovo Player in una posizione casuale all'interno del campo di gioco specificato.
    /// #Argomenti
    /// * 'campo': Un puntatore ad una struttura CampoGioco
    pub fn nuovo(campo: &CampoGioco) -> Player {
        let mut rng = rand::thread_rng();
        let posizione = campo.player_position;
        let mosse = campo.dimensione * campo.dimensione;
        let direzione = match rng.gen_range(0..4) {
            0 => Direzione::Su,
            1 => Direzione::Giu,
            2 => Direzione::Destra,
            _ => Direzione::Sinistra,
        };
        let forza = rng.gen_range(1..=100); // Forza casuale da 1 a 100

        Player {
            posizione,
            direzione,
            forza,
            mosse
        }
    }

    /// Muove il giocatore nella direzione specificata all'interno del campo di gioco.
   /// #Argomenti
   /// * 'gamespace': Un puntatore ad una struttura CampoGioco
    /// * 'direction': Direzione in cui ci si vuole spostare
    pub fn muovi(&mut self, gamespace: &mut CampoGioco, mut direction: Direzione) {
        let mut rng = rand::thread_rng();
        //gestione numero di mosse
        if self.mosse == 0 && self.forza > 0 {
            print!("HAI VINTO!\n IL NUMERO DI MOSSE è ARRIVATO A 0\n");
            return;
        }
        self.mosse -= 1;
        let mut row = self.posizione.riga;
        let mut column = self.posizione.colonna;
        //Succeso del movimento nella direzione voluta e riassegnazione se necessario
        if rng.gen_bool(0.5) {
            match rng.gen_range(0..4) {
                0 => direction = Direzione::Su,
                1 => direction = Direzione::Giu,
                2 => direction = Direzione::Destra,
                _ => direction = Direzione::Sinistra,
            };
            self.direzione = direction;
        } else {
            match direction {
                Direzione::Su => {
                    if self.posizione.riga > 0 {
                        row = self.posizione.riga - 1;
                        self.direzione = Direzione::Su;
                    } else {
                        self.direzione = wall(&self.direzione);
                    }
                }
                Direzione::Giu => {
                    if self.posizione.riga < gamespace.dimensione - 1 {
                        row = self.posizione.riga + 1;
                        self.direzione = Direzione::Giu;
                    } else {
                        self.direzione = wall(&self.direzione);
                    }
                }
                Direzione::Sinistra => {
                    if self.posizione.colonna > 0 {
                        column = self.posizione.colonna - 1;
                        self.direzione = Direzione::Sinistra;
                    } else {
                        self.direzione = wall(&self.direzione);
                    }
                }
                Direzione::Destra => {
                    if self.posizione.colonna < gamespace.dimensione - 1 {
                        column = self.posizione.colonna + 1;
                        self.direzione = Direzione::Destra;
                    } else {
                        self.direzione = wall(&self.direzione);
                    }
                }
            }
        }
        match gamespace.celle[row][column] {
            Cella::Cibo(10) => {
                gamespace.celle[row][column] = Cella::Vuota;
                self.forza += 10;
            },
            Cella::Veleno(10) => {
                gamespace.celle[row][column] = Cella::Vuota;
                self.forza -= 10
            },
            _ => gamespace.celle[row][column] = Cella::Vuota,
        };
        if self.forza <= 0 {
            print!("HAI PERSO!\n SEI STATO AVVELENATO\n");
            return;
        }
        self.posizione.riga = row;
        self.posizione.colonna = column;
        gamespace.player_position.riga = row;
        gamespace.player_position.colonna = column;
    }
}

/// Funzione per invertire la direzione se il giocatore incontra un muro.
/// #Argomenti
/// * 'dir': Direzione del giocatore attauale
fn wall(dir: &Direzione) -> Direzione {
    let direction;
    match dir {
        Direzione::Su => direction = Direzione::Giu,
        Direzione::Giu => direction = Direzione::Su,
        Direzione::Sinistra => direction = Direzione::Destra,
        Direzione::Destra => direction = Direzione::Sinistra,
    };
    direction
}

impl fmt::Display for Player {
    /// Implementazione della formattazione per visualizzare le informazioni del giocatore.
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        // Scrivi direttamente sul formatter anziché utilizzare una stringa temporanea
        write!(f, "Giocatore\nDirezione: {}\nForza: {}\nPosizione: {}\nMosse: {}\n",
               match self.direzione {
                   Direzione::Su => "Su",
                   Direzione::Destra => "Destra",
                   Direzione::Sinistra => "Sinistra",
                   Direzione::Giu => "Giu",
               },
               self.forza, self.posizione, self.mosse)
    }
}
