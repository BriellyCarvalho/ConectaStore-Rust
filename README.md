# ConectaStore — Estruturas de Dados em Rust

## 1. Objetivo e funcionamento

O **ConectaStore** é uma aplicação de linha de comando desenvolvida em Rust para representar produtos, relações entre produtos, clientes e compras.

O sistema permite:

* cadastrar e consultar produtos;
* conectar produtos;
* registrar compras;
* recomendar produtos relacionados;
* evitar recomendações de produtos já comprados pelo cliente;
* testar o desempenho da recomendação.

A recomendação utiliza o algoritmo **BFS (Busca em Largura)** para percorrer o grafo de produtos.

## 2. Tecnologias e estruturas utilizadas

- **Rust**: linguagem utilizada no desenvolvimento.
- **Cargo**: ferramenta de gerenciamento, compilação e testes.
- **HashMap**: armazenamento dos vértices e suas conexões.
- **HashSet**: controle de produtos já comprados e vértices visitados.
- **VecDeque**: fila utilizada pela BFS.
- **enum Vertice**: representação de produtos e clientes.
- **Testes unitários e testes de integração**: verificação do funcionamento do sistema.

## 3. Arquitetura da solução

```
megastore/
├── src/
│   ├── main.rs
│   ├── grafo.rs
│   └── produto.rs
├── tests/
│   └── integracao_test.rs
├── Cargo.toml
├── Cargo.lock
└── README.md
```

* **`main.rs`**: menu, entrada de dados e fluxo da aplicação.
* **`grafo.rs`**: estrutura do grafo, conexões, compras e recomendações.
* **`produto.rs`**: definição da estrutura `Produto`.
* **`integracao_test.rs`**: testes de integração.

O grafo utiliza **lista de adjacência**, representada por:

```
HashMap<Vertice, Vec<Vertice>>
```

As relações entre produtos são representadas como arestas não direcionadas e não ponderadas.

## 4. Como compilar e executar

Com Rust e Cargo instalados, execute:

```
Compilar: cargo check
Executar: cargo run
Testar: cargo test
Formatar: cargo fmt
Verificar formatação: cargo fmt --check
```

## 5. Exemplos de uso

Ao executar o programa, o menu apresenta:

```
1 - Cadastrar produto
2 - Consultar produto
3 - Conectar produtos
4 - Registrar compra
5 - Recomendar por produto
6 - Recomendar por cliente
7 - Teste de dese
```
Exemplo de consulta:

Selecionar a opção de consultar produto

Informar o ID do produto

O sistema apresenta os dados do produto correspondente, caso ele exista.

## 6. Complexidade e desempenho

A busca em largura (BFS) percorre o grafo para encontrar produtos relacionados.

Considerando V como o número de vértices e E como o número de arestas, a complexidade de tempo da BFS é O(V + E).

A lista de adjacência utiliza O(V + E) de espaço para representar os vértices e as arestas do grafo.

Os tempos de execução podem variar de acordo com o hardware, a quantidade de dados e as condições do ambiente de teste.

## 7. Testes automatizados
Os testes automatizados verificam funcionalidades como cadastro e consulta de produtos, conexões, compras e recomendações.

Resultados dos testes:

11 testes unitários aprovados

15 testes de integração aprovados

26 testes no total

0 falhas

## 8. Vídeo Pitch

Link do vídeo: https://youtu.be/OywvDrrIcP4