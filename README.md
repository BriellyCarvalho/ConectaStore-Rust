# ConectaStore — Estruturas de Dados em Rust

## 1. Objetivo e funcionamento

O **ConectaStore** é uma aplicação de linha de comando desenvolvida em Rust para representar produtos, relações entre produtos, clientes e compras.

O sistema utiliza um grafo para relacionar produtos e clientes, permitindo gerar recomendações com base nas conexões existentes.

A aplicação permite:

- Cadastrar e consultar produtos;
- Conectar produtos relacionados;
- Registrar compras de clientes;
- Recomendar produtos relacionados a um produto selecionado;
- Recomendar produtos para clientes, evitando sugerir itens que já compraram;
- Listar os produtos cadastrados;
- Executar um teste de desempenho com diferentes quantidades de produtos.

Ao iniciar a aplicação, cinco produtos de exemplo são carregados: Notebook, Mouse, Teclado, Headset e Monitor. Também são criadas conexões entre produtos e registradas compras de exemplo para o cliente de ID 100, permitindo testar as recomendações desde o início.

## 2. Tecnologias utilizadas

- **Rust:** linguagem utilizada no desenvolvimento da aplicação.
- **Cargo:** ferramenta de gerenciamento, compilação e execução do projeto.
- **Git e GitHub:** controle de versão e hospedagem do código.

## 3. Arquitetura da solução

A estrutura do projeto está organizada da seguinte forma:

```
ConectaStore-Rust/
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

### Responsabilidade dos arquivos

- **`main.rs`**: menu, entrada de dados e fluxo da aplicação.
- **`grafo.rs`**: estrutura do grafo, conexões, compras e recomendações.
- **`produto.rs`**: definição e criação da estrutura `Produto`.
- **`integracao_test.rs`**: testes de integração das funcionalidades do sistema.

## 4. Estruturas de dados utilizadas

O projeto utiliza as seguintes estruturas:

- **HashMap:** armazena produtos e suas conexões, permitindo consultas por identificador.
- **HashSet:** registra os produtos comprados por cada cliente e evita duplicidade nos registros de compras.
- **Vec:** armazena as listas de adjacência, representando as conexões entre os vértices.
- **VecDeque:** implementa a fila utilizada na busca em largura.
- **Grafo:** representa os relacionamentos entre produtos e clientes.

Os vértices são representados pelo tipo `Vertice`, que diferencia produtos e clientes.

A representação por lista de adjacência permite armazenar as relações existentes entre os elementos sem precisar criar uma matriz para todas as combinações possíveis.

## 5. Como executar o projeto

Com Rust e Cargo instalados, execute:

```
Compilar: cargo check
Executar: cargo run
Testar: cargo test
Formatar: cargo fmt
Verificar formatação: cargo fmt --check
```

## 6. Funcionalidades do menu

Ao executar a aplicação, o usuário encontra as seguintes opções:

```
1 - Cadastrar produto
2 - Consultar produto
3 - Conectar produtos
4 - Registrar compra
5 - Recomendar produtos
6 - Listar produtos
7 - Teste de desempenho
0 - Sair
```

Na opção **Recomendar produtos**, existem duas possibilidades:

1. Recomendar produtos a partir de um produto de referência.
2. Recomendar produtos para um cliente, considerando suas compras registradas.

### Exemplo de recomendação

Se o cliente já comprou um Mouse, esse produto não deve aparecer novamente como sugestão de compra. O sistema percorre as conexões do grafo para encontrar outros produtos relacionados.

O sistema também verifica situações como produtos inexistentes, conexões duplicadas e tentativas de conectar um produto a ele mesmo.

## 7. Algoritmo e complexidade

O algoritmo de recomendação utiliza a **busca em largura (BFS — Breadth-First Search)**, implementada com uma fila `VecDeque` e um conjunto de vértices visitados.

Considerando:

- **V:** quantidade de vértices do grafo;
- **E:** quantidade de arestas do grafo.

A travessia em largura apresenta complexidade de tempo **O(V + E)**. A representação do grafo por listas de adjacência utiliza **O(V + E)** de espaço para armazenar os vértices e as conexões.

As verificações adicionais para evitar recomendações duplicadas e excluir produtos já comprados também influenciam o custo total da operação.

## 8. Testes automatizados

## 8. Testes automatizados

O projeto possui testes unitários e testes de integração para verificar o funcionamento das principais funcionalidades.

**Resultados da execução:**

- 16 testes unitários aprovados;
- 20 testes executados no arquivo de integração, incluindo testes unitários repetidos;
- 0 falhas em ambas as execuções.

Os testes verificam cadastro e consulta de produtos, conexões, registro de compras, recomendações e prevenção de duplicidades.

## 10. Apresentação do projeto

Vídeo de apresentação:

https://youtu.be/OywvDrrIcP4
