use std::collections::HashMap;
use crate::models::Produto;

pub struct Catalogo {
    // Indexamos por nome para busca O(1)
    pub index_nome: HashMap<String, Produto>,
}

impl Catalogo {
    pub fn new() -> Self {
        Catalogo { index_nome: HashMap::new() }
    }

    pub fn adicionar_produto(&mut self, p: Produto) {
        self.index_nome.insert(p.nome.to_lowercase(), p);
    }

    pub fn buscar_por_nome(&self, nome: &str) -> Option<&Produto> {
        self.index_nome.get(&nome.to_lowercase())
    }
}