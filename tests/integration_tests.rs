// Importamos as funcionalidades da nossa própria biblioteca
use megastore_search_rust::models::Produto;
use megastore_search_rust::engine::Catalogo;

#[test]
fn teste_fluxo_completo_de_busca_no_catalogo() {
    // 1. Preparação (Setup)
    let mut catalogo = Catalogo::new();
    let produto_exemplo = Produto {
        id: 100,
        nome: "Monitor Gamer".to_string(),
        marca: "UltraVision".to_string(),
        categoria: "Periféricos".to_string(),
        preco: 1200.50,
    };

    // 2. Ação (Action)
    catalogo.adicionar_produto(produto_exemplo);
    let resultado = catalogo.buscar_por_nome("monitor gamer");

    // 3. Verificação (Assertion)
    assert!(resultado.is_some(), "O produto deveria ter sido encontrado");
    let p = resultado.unwrap();
    assert_eq!(p.marca, "UltraVision");
    assert_eq!(p.preco, 1200.50);
}

#[test]
fn teste_busca_por_produto_inexistente() {
    let catalogo = Catalogo::new();
    let resultado = catalogo.buscar_por_nome("Cadeira de Escritório");
    
    assert!(resultado.is_none(), "Não deveria encontrar um produto que não foi adicionado");
}