// Agora usamos o nome do projeto para importar
use megastore_search_rust::models::Produto;
use megastore_search_rust::engine::Catalogo;

fn main() {
    let mut catalogo = Catalogo::new();
    
    let p1 = Produto {
        id: 1,
        nome: "Smartphone Pro".to_string(),
        marca: "TechCo".to_string(),
        categoria: "Eletrónicos".to_string(),
        preco: 2999.99,
    };

    catalogo.adicionar_produto(p1);

    println!("--- MegaStore Search Engine Ativa ---");
    println!("Dica: Use 'cargo test' para validar a integridade do sistema.");
}