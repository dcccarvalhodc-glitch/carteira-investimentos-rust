mod models;
mod handlers;

fn main() {

    println!("========================================");
    println!("CARTEIRA DE INVESTIMENTOS");
    println!("========================================");

    let investimentos = handlers::listar_investimentos();

    println!();

    println!("Investimentos cadastrados:");

    for investimento in investimentos {

        println!("--------------------------------");

        println!("ID: {}", investimento.id);

        println!("Nome: {}", investimento.nome);

        println!("Código: {}", investimento.codigo);

        println!("Quantidade: {}", investimento.quantidade);

        println!("Preço: R$ {:.2}", investimento.preco);

    }

}
