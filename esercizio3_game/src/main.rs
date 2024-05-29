use std::io;
use esercizio3_game::game_space::game_space::CampoGioco;
use esercizio3_game::player::{Direzione, Player};


fn main() {
    /// Richiede all'utente la dimensione della mappa e la quantità di cibo e veleno, quindi gestisce il gioco.
    ///
    /// Questa funzione avvia il gioco e gestisce l'interazione con l'utente.
    fn main() {
        // Richiedi all'utente di inserire la dimensione della mappa
        println!("Inserisci la dimensione della mappa:");
        let mut dimensione = String::new();
        io::stdin().read_line(&mut dimensione)
            .expect("Errore durante la lettura dell'input");
        let dimensione: usize = dimensione.trim()
            .parse()
            .expect("Inserisci un numero valido");


        // Richiedi all'utente di inserire la quantità di cibo e veleno
        println!("Inserisci la quantità di cibo e veleno:");
        let mut qty = String::new();
        io::stdin().read_line(&mut qty)
            .expect("Errore durante la lettura dell'input");
        let qty: usize = qty.trim()
            .parse()
            .expect("Inserisci un numero valido");

        // Crea il campo di gioco e il giocatore
        let mut gamespace = CampoGioco::nuovo(dimensione, qty);
        let mut player = Player::nuovo(&gamespace);

        loop {
            // Visualizza lo stato attuale del gioco
            println!("{}", gamespace);
            println!("{}", player);
            println!("Che mossa vuoi fare? (G/S/D/L):");

            // Leggi l'input dell'utente
            let mut input = String::new();
            io::stdin().read_line(&mut input)
                .expect("Errore durante la lettura dell'input");
            let input = input.trim().to_ascii_uppercase();

            // Esegui la mossa del giocatore in base all'input
            match input.as_str() {
                "G" => player.muovi(&mut gamespace, Direzione::Giu),
                "S" => player.muovi(&mut gamespace, Direzione::Su),
                "D" => player.muovi(&mut gamespace, Direzione::Destra),
                "L" => player.muovi(&mut gamespace, Direzione::Sinistra),
                _ => println!("Input non valido! Usa G, S, D o L per muoverti."),
            }
        }
    }
}
