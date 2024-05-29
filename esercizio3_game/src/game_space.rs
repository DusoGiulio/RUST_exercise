pub mod game_space {
    use std::fmt;
    use rand::{random, Rng};

    /// Rappresenta una posizione nel campo di gioco.
    #[derive(Debug, Clone, Copy)]
    pub struct Posizione {
        /// Coordinata della riga.
        pub riga: usize,
        /// Coordinata della colonna.
        pub colonna: usize,
    }

    /// Rappresenta il contenuto di una cella nel campo di gioco.
    #[derive(Debug, Clone, Copy, PartialEq)]
    pub enum Cella {
        /// La cella è vuota.
        Vuota,
        /// La cella contiene cibo con una quantità specifica.
        Cibo(u32),
        /// La cella contiene veleno con una quantità specifica.
        Veleno(u32),
    }

    /// Rappresenta il campo di gioco con la sua dimensione, le celle e la posizione del giocatore.
    #[derive(Debug)]
    pub struct CampoGioco {
        /// La dimensione del campo di gioco.
        pub dimensione: usize,
        /// La matrice delle celle.
        pub celle: Vec<Vec<Cella>>,
        /// La posizione del giocatore.
        pub player_position: Posizione,
    }

    impl CampoGioco {
        /// Crea e restituisce un nuovo campo di gioco con la dimensione specificata e posiziona casualmente cibo e veleno nel campo.
        /// # Argomenti
        ///
        /// * `dimensione`: Dimensione del campo di gioco.
        /// * `m`: Numero di celle veleno e cibo in campo.
        pub fn nuovo(dimensione: usize, m: usize) -> CampoGioco {
            // Inizializza tutte le celle come vuote
            let mut celle = vec![vec![Cella::Vuota; dimensione]; dimensione];
            let mut x_prec=0;
            let mut y_prec=0;
            let mut p_pos_x=0;
            let mut p_pos_y=0;
            // Posizione casuale del giocatore
            let (x, y) = CampoGioco::pos_casuale(dimensione, (x_prec, y_prec), (p_pos_x,p_pos_y));
            celle[x][y] = Cella::Vuota; // Rimuovi qualsiasi oggetto presente nella cella del giocatore
            x_prec= x;
            y_prec=y;
            p_pos_x=x;
            p_pos_y=y;
            // Posiziona m cibo e m veleno in posizioni casuali
            for _ in 0..m {
                let (x, y) = CampoGioco::pos_casuale(dimensione, (x_prec, y_prec),(p_pos_x,p_pos_y));
                x_prec= x;
                y_prec=y;
                if random::<bool>() {
                    celle[x_prec][y_prec] = Cella::Cibo(10)
                } else {
                    celle[x_prec][y_prec] = Cella::Veleno(10)
                };
            }

            let player_position = Posizione {
                riga: x,
                colonna: y,
            };
            CampoGioco {
                dimensione,
                celle,
                player_position,
            }
        }

        /// Genera casualmente una posizione nel campo di gioco, evitando la posizione precedente e la posizione del giocatore.
        /// #Argomenti
        ///
        /// * 'dimensione': Dimensione del campo di gioco.
        /// * 'precedente': Posizione precedente generata
        /// * 'p_pos': Posizione del giocatore
        fn pos_casuale(dimensione: usize, precedente: (usize, usize), p_pos:(usize,usize)) -> (usize, usize) {
            let mut rng = rand::thread_rng();
            loop {
                let riga = rng.gen_range(0..dimensione);
                let colonna = rng.gen_range(0..dimensione);
                if (riga, colonna) != precedente && (riga, colonna)!= p_pos {
                    return (riga, colonna);
                }
            }
        }
    }

    impl fmt::Display for CampoGioco {
        /// Implementazione della formattazione per visualizzare il campo di gioco.
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            // Costruisci una rappresentazione del campo di gioco
            let mut campo_string = String::new();

            for (i, riga) in self.celle.iter().enumerate() {
                for (j, cella) in riga.iter().enumerate() {
                    let simbolo = match cella {
                        Cella::Vuota => "O",
                        Cella::Cibo(_) => "$",
                        Cella::Veleno(_) => "-",
                    };

                    // Se la posizione corrente è la stessa del giocatore, rappresenta il giocatore come 'P'
                    if i == self.player_position.riga && j == self.player_position.colonna {
                        campo_string.push('P');
                    } else {
                        campo_string.push_str(simbolo);
                    }
                    campo_string.push(' '); // Spazio tra le celle per una migliore leggibilità
                }
                campo_string.push('\n'); // Nuova riga alla fine di ogni riga del campo
            }
            write!(f, "{}", campo_string)
        }
    }

    impl fmt::Display for Posizione {
        /// Implementazione della formattazione per visualizzare una posizione nel campo di gioco.
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "({}, {})", self.riga, self.colonna)
        }
    }
}
