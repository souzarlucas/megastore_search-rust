# Sistema de Busca Otimizado para Catálogo de Produtos - MegaStore

## 📝 Descrição do Projeto
Este projeto consiste no desenvolvimento de um motor de busca de alta performance para a "MegaStore", uma gigante do varejo online. A solução visa substituir métodos de busca tradicionais (lentos e imprecisos) por uma implementação em Rust que utiliza estruturas de dados avançadas para garantir buscas instantâneas e precisas em catálogos com milhões de itens.

O foco principal é a **eficiência na indexação** e a **velocidade de resposta**, fundamentais para a experiência do usuário e a conversão de vendas em grandes e-commerces.

## 🛠️ Tecnologias Utilizadas
* **Linguagem de Programação:** Rust (Edição 2021)
* **Gerenciador de Pacotes e Build:** Cargo
* **Estruturas de Dados:** `std::collections::HashMap` (Tabela Hash)
* **Ferramentas de Teste:** Rust Test Runner (Integrado)

## 🏗️ Arquitetura do Sistema
O sistema foi desenvolvido seguindo uma abordagem modular para facilitar a manutenção e escalabilidade:

* **`src/models.rs`**: Contém a definição da estrutura `Produto`.
* **`src/engine.rs`**: Implementa o motor de busca e a lógica do `Catalogo`.
* **`src/lib.rs`**: Atua como a biblioteca do projeto, expondo os módulos para o executável e para os testes.
* **`src/main.rs`**: Ponto de entrada da aplicação que demonstra o funcionamento do sistema.
* **`tests/`**: Pasta dedicada a testes de integração que validam o comportamento do sistema.



## 🧠 Algoritmos e Estruturas de Dados
O núcleo desta solução utiliza **Tabelas Hash (Hash Maps)**.
* **Funcionamento:** Cada produto é indexado utilizando o seu nome como chave (normalizado para minúsculas). Isso permite que, em vez de percorrer toda a lista de produtos ($O(n)$), o sistema realize um cálculo matemático (hashing) para encontrar o endereço exato do produto na memória.
* **Complexidade:** A busca opera em **Tempo Constante $O(1)$**, o que significa que o tempo de resposta permanece praticamente o mesmo, quer o catálogo tenha 10 ou 10 milhões de itens.

## 🚀 Como Executar o Sistema de Busca
1. Certifique-se de ter o ambiente Rust instalado ([rustup.rs](https://rustup.rs/)).
2. Navegue até a pasta do projeto no terminal.
3. Para compilar e rodar a demonstração, utilize o comando:
   ```bash
   cargo run
🧪 Como Executar os Testes
Para garantir a qualidade e a confiabilidade da indexação e da busca, execute os testes unitários e de integração com o comando:

Bash
cargo test
Este comando validará se a busca encontra produtos existentes e se lida corretamente com consultas de itens inexistentes.

💻 Exemplos de Uso
O sistema permite realizar buscas rápidas através da instância do Catalogo:

Rust
// Criando um catálogo e adicionando um produto
let mut catalogo = Catalogo::new();
catalogo.adicionar_produto(Produto {
    id: 1,
    nome: "Smartphone Pro".to_string(),
    marca: "TechCo".to_string(),
    categoria: "Eletrónicos".to_string(),
    preco: 2999.99,
});

// Realizando uma busca (insensível a maiúsculas)
if let Some(produto) = catalogo.buscar_por_nome("smartphone pro") {
    println!("Produto encontrado: {} - R${}", produto.nome, produto.preco);
}
📈 Desempenho e Escalabilidade
Diferente de métodos tradicionais, a implementação em Rust com HashMap oferece:

Latência Mínima: Resultados de testes mostram execuções de busca em frações de milissegundo.

Uso Eficiente de Memória: O Rust garante gerenciamento de memória seguro sem a necessidade de um Garbage Collector, ideal para lidar com grandes volumes de dados (Big Data) em tempo real.

Escalabilidade Horizontal: A lógica de indexação permite que o sistema cresça sem perda de performance linear.

🤝 Contribuições
Este projeto foi desenvolvido como um desafio acadêmico para a UniFECAF. Pull Requests com melhorias em busca fonética ou filtros por múltiplas categorias são bem-vindos.

📄 Licença
Este projeto está sob a [licença MIT](LICENSE).