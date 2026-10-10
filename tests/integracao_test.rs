#[path = "../src/produto.rs"]
mod produto;

#[path = "../src/grafo.rs"]
mod grafo;

use grafo::Grafo;
use produto::Produto;

#[test]
fn deve_cadastrar_e_consultar_produto() {
    let mut grafo = Grafo::novo();

    let produto = Produto::novo(1, "Notebook", "Eletronicos", 3500.00);

    grafo.cadastrar_produto(produto);

    let encontrado = grafo.consultar_produto(1);

    assert!(encontrado.is_some());

    assert_eq!(encontrado.unwrap().nome, "Notebook");
}

#[test]
fn deve_recomendar_produtos_conectados() {
    let mut grafo = Grafo::novo();

    grafo.cadastrar_produto(Produto::novo(1, "Notebook", "Eletronicos", 3500.0));

    grafo.cadastrar_produto(Produto::novo(2, "Mouse", "Eletronicos", 80.0));

    grafo.conectar_produtos(1, 2);

    let recomendacoes = grafo.recomendar_por_produto(1);

    assert_eq!(recomendacoes.len(), 1);

    assert_eq!(recomendacoes[0].id, 2);
}

#[test]
fn nao_deve_recomendar_produto_ja_comprado() {
    let mut grafo = Grafo::novo();

    grafo.cadastrar_produto(Produto::novo(1, "Notebook", "Eletronicos", 3500.0));

    grafo.cadastrar_produto(Produto::novo(2, "Mouse", "Eletronicos", 80.0));

    grafo.cadastrar_produto(Produto::novo(3, "Teclado", "Eletronicos", 150.0));

    grafo.conectar_produtos(1, 2);

    grafo.conectar_produtos(2, 3);

    grafo.registrar_compra(100, 1);

    grafo.registrar_compra(100, 2);

    let recomendacoes = grafo.recomendar_por_cliente(100);

    assert!(recomendacoes.iter().all(|p| p.id != 1 && p.id != 2));
}

#[test]
fn deve_evitar_conexoes_duplicadas() {
    let mut grafo = Grafo::novo();

    grafo.cadastrar_produto(Produto::novo(1, "Notebook", "Eletronicos", 3500.0));

    grafo.cadastrar_produto(Produto::novo(2, "Mouse", "Eletronicos", 80.0));

    grafo.conectar_produtos(1, 2);

    grafo.conectar_produtos(1, 2);

    let conexoes = grafo.conexoes.get(&grafo::Vertice::Produto(1)).unwrap();

    assert_eq!(conexoes.len(), 1);
}
