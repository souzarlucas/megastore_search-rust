use std::collections::HashMap;

// Estrutura do Produto (Requisito: Indexar por nome, marca, categoria) [cite: 15]
#[derive(Debug, Clone)]
struct Produto {
    id: u32,
    nome: String,
    categoria: String,
    preco: f64,
}

struct Catalogo {
    // Tabela Hash para busca O(1) [cite: 44, 74]
    index_nome: HashMap<String, Produto>,
}

impl Catalogo {
    fn new() -> Self {
        Catalogo { index_nome: HashMap::new() }
    }

    fn adicionar_produto(&mut self, p: Produto) {
        self.index_nome.insert(p.nome.to_lowercase(), p);
    }

    fn buscar_por_nome(&self, nome: &str) -> Option<&Produto> {
        self.index_nome.get(&nome.to_lowercase())
    }
}

fn main() {
    println!("Sistema de Busca MegaStore Ativo!");
}

// --- TESTES UNITÁRIOS [cite: 56, 71, 85, 93] ---
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn teste_busca_eficiente() {
        let mut cat = Catalogo::new();
        cat.adicionar_produto(Produto {
            id: 1,
            nome: "Teclado".to_string(),
            categoria: "Tech".to_string(),
            preco: 100.0,
        });
        assert!(cat.buscar_por_nome("teclado").is_some());
    }
}