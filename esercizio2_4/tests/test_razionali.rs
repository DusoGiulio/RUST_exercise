#[cfg(test)]
mod test {
    use esercizio2_4::razionali::razionali::Razionali;
    //test esercizio2_4
    #[test]
    fn test_somma() {
        let raz1 = Razionali::new(1, 2);
        let raz2 = Razionali::new(1, 3);
        let risultato = raz1.somma(&raz2);
        assert_eq!(risultato.num(), 5);
        assert_eq!(risultato.denum(), 6);
    }

    #[test]
    fn test_prodotto() {
        let raz1 = Razionali::new(2, 3);
        let raz2 = Razionali::new(3, 4);
        let risultato = raz1.prodotto(&raz2);
        assert_eq!(risultato.num(), 1);
        assert_eq!(risultato.denum(), 2);
    }
    ////////////////////////////////////////////////////////////
    //test esercizio 4
    #[test]
    fn test_somma_razionali() {
        let a = Razionali::new(1, 2);
        let b = Razionali::new(1, 3);
        let r = a + b;
        assert_eq!(r, Razionali::new(5, 6));
    }

    #[test]
    fn test_moltiplicazione_razionali() {
        let a = Razionali::new(2, 3);
        let b = Razionali::new(3, 4);
        let r = a * b;
        assert_eq!(r, Razionali::new(1, 2));
    }

    #[test]
    fn test_moltiplicazione_i32_razionale() {
        let a = Razionali::new(2, 3);
        let b = 4;
        let r = a * b;
        assert_eq!(r, Razionali::new(8, 3));
    }

    #[test]
    fn test_somma_i32_razionale() {
        let a = Razionali::new(2, 3);
        let b = 4;
        let r = a + b;
        assert_eq!(r, Razionali::new(14, 3));
    }
    /////////////////////////////////////////////////////////////
}
