#[cfg(test)]
mod tests {
    use esercizio5::conto_bancario::conto_bancario::ContoBancario;
    use esercizio5::stati::stati::*;

    #[test]
    fn test_nuovo_conto_bancario_stato_rosso() {
        let conto = ContoBancario::new("Mario Rossi".to_string(), 100.0, 50.0, 150.0, 0.05);
        matches!(conto.stato.stato(), Stato::Rosso);
    }
    #[test]
    fn test_nuovo_conto_bancario_stato_argento() {
        let conto = ContoBancario::new("Giuseppe Verdi".to_string(), 80.0, 50.0, 150.0, 0.05);
        matches!(conto.stato.stato(), Stato::Argento);
    }
    #[test]
    fn test_nuovo_conto_bancario_stato_oro() {
        let conto = ContoBancario::new("Alessandro Volta".to_string(), 200.0, 50.0, 150.0, 0.05);
        matches!(conto.stato.stato(), Stato::Oro);
    }
    #[test]
    fn test_deposito_conto_bancario() {
        let mut conto = ContoBancario::new("Mario Rossi".to_string(), 100.0, 50.0, 150.0, 0.05);
        conto.stato.deposita(&mut conto.saldo, 50.0);
        assert_eq!(conto.saldo, 150.0);
    }
    #[test]
    fn test_preleva_conto_bancario() {
        let mut conto = ContoBancario::new("Giuseppe Verdi".to_string(), 100.0, 50.0, 150.0, 0.05);
        conto.stato.preleva(&mut conto.saldo, 50.0);
        assert_eq!(conto.saldo, 50.0);
    }
    #[test]
    fn test_paga_interessi_conto_bancario() {
        let mut conto = ContoBancario::new("Alessandro Volta".to_string(), 200.0, 50.0, 150.0, 0.05);
        conto.stato.paga_interessi(&mut conto.saldo, 0.05);
        assert_eq!(conto.saldo, 210.0);
    }
}
