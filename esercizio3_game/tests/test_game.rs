
use esercizio3_game::game_space::game_space::{CampoGioco, Cella, Posizione};
use esercizio3_game::player::Player;

#[test]
fn test_nuovo_campo_gioco() {
    let dimensione = 5;
    let m = 3;
    let campo = CampoGioco::nuovo(dimensione, m);

    // Verifica che la dimensione del campo di gioco sia corretta
    assert_eq!(campo.dimensione, dimensione);

    // Verifica che ci siano esattamente m cibo e m veleno nel campo di gioco
    let mut cibo_count = 0;
    let mut veleno_count = 0;

    for riga in campo.celle.iter() {
        for cella in riga.iter() {
            match cella {
                Cella::Cibo(_) => cibo_count += 1,
                Cella::Veleno(_) => veleno_count += 1,
                _ => (),
            }
        }
    }

    assert_eq!(cibo_count+veleno_count, m);
}

#[test]
fn test_display_campo_gioco() {
    let dimensione = 5;
    let m = 3;
    let mut campo = CampoGioco::nuovo(dimensione, m);

    // Modifica la posizione del giocatore per il tests
    campo.player_position = Posizione { riga: 1, colonna: 1 };

    // Genera la rappresentazione del campo di gioco
    let display = format!("{}", campo);

    // Verifica che la rappresentazione contenga il giocatore e la dimensione corretti
    assert!(display.contains("P"));
    assert!(display.contains("O")); // Vuota
    assert!(display.contains("$")); // Cibo
    assert!(display.contains("-")); // Veleno
    assert_eq!(display.lines().count(), dimensione); // Verifica la dimensione corretta del campo
}
#[test]
fn test_nuovo_player() {
    // Crea un campo di gioco
    let campo = CampoGioco::nuovo(5, 3);

    // Crea un nuovo giocatore
    let player = Player::nuovo(&campo);

    // Assicurati che il giocatore sia stato creato correttamente
    assert_eq!(player.mosse, 25); // dimensione * dimensione
    assert!(player.forza >= 1 && player.forza <= 100); // Forza compresa tra 1 e 100
    assert!(player.posizione.riga < campo.dimensione && player.posizione.colonna < campo.dimensione); // Posizione all'interno del campo
}