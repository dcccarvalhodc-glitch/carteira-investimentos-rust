use crate::models::Investimento;

pub fn listar_investimentos() -> Vec<Investimento> {

    vec![
        Investimento {
            id: 1,
            nome: "Petrobras".to_string(),
            codigo: "PETR4".to_string(),
            quantidade: 100.0,
            preco: 32.50,
        },

        Investimento {
            id: 2,
            nome: "Vale".to_string(),
            codigo: "VALE3".to_string(),
            quantidade: 50.0,
            preco: 68.20,
        },
    ]
}
