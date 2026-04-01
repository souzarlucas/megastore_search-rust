## Sistema de Busca Otimizado - MegaStore 🦀

Este projeto foi desenvolvido como parte do desafio prático de **Data Structure Strategy and Implementation** na UniFECAF. O objetivo é substituir sistemas de busca tradicionais (lentos e imprecisos) por uma implementação de alta performance em **Rust**.

## 🚀 O Problema de Negócio

A MegaStore enfrentava abandono de carrinhos devido a buscas lentas em um catálogo de milhões de itens. O sistema anterior falhava na escalabilidade, prejudicando a experiência do usuário.

## 🛠️ Minha Solução Técnica

Para garantir que a busca ocorra em tempo constante $O(1)$, utilizei a estrutura std::collections::HashMap.

Por que esta abordagem?
    
* Indexação Eficiente: Os produtos são mapeados por chaves (nomes), permitindo acesso direto sem percorrer toda a lista.
* Segurança de Memória: Rust elimina erros comuns de memória (segfaults) sem a necessidade de um Garbage Collector, ideal para sistemas de alta carga.
* Normalização: Implementei a busca case-insensitive (insensível a maiúsculas) para melhorar a precisão dos resultados.
    
### 🧪 Qualidade e Confiabilidade

O projeto segue as melhores práticas de desenvolvimento Rust:

* Testes Unitários: Cobertura de testes para inserção, busca bem-sucedida e tratamento de itens inexistentes.
* Gestão de Dependências: Uso do Cargo para um ambiente de compilação limpo e reprodutível.

## 📋 Como Testar

Clone o repositório: 

1. **git clone git@github.com:seu-usuario/megastore_search-rust.git**
2. Execute os testes de performance: **cargo test**
3. Execute a aplicação: **cargo run**