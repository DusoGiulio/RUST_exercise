use std::thread;
use crate::banditore::banditore::Banditore;

mod banditore;
mod partecipante;



fn main() {
    let h= thread::spawn(||{ let s = String::from("Prova");
        let b= Banditore::new(5,s,10.0,1000.0);
        b.avvia_asta();});
   h.join().unwrap();

    println!("\n\n\n");


    let h1= thread::spawn(||{ let s = String::from("Prova1");
        let b= Banditore::new(6,s,50.0,100.0);
        b.avvia_asta();});
    h1.join().unwrap();
}