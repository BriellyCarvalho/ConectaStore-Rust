mod grafo;
mod produto;

use grafo::Grafo;
use produto::Produto;
use std::io::{self, Write};
use std::time::Instant;

fn ler_texto(mensagem: &str) -> String {
    print!("{}", mensagem);
    io::stdout().flush().unwrap();

    let mut entrada = String::new();

    io::stdin().read_line(&mut entrada).unwrap();

    entrada.trim().to_string()
}

fn ler_u32(mensagem: &str) -> u32 {
    loop {
        let entrada = ler_texto(mensagem);

        match entrada.parse::<u32>() {
            Ok(valor) => return valor,
            Err(_) => {
                println!("Digite um número válido.");
            }
        }
    }
}

fn ler_f64(mensagem: &str) -> f64 {
    loop {
        let entrada = ler_texto(mensagem);

        match entrada.parse::<f64>() {
            Ok(valor) => return valor,
            Err(_) => {
                println!("Digite um número válido.");
            }
        }
    }
}

fn cadastrar_produto(grafo: &mut Grafo) {
    let id = ler_u32("ID do produto: ");

    if grafo.consultar_produto(id).is_some() {
        println!("Já existe um produto com esse ID.");
        return;
    }

    let nome = ler_texto("Nome: ");

    let categoria = ler_texto("Categoria: ");

    let preco = ler_f64("Preço: ");

    let produto = Produto::novo(id, &nome, &categoria, preco);

    grafo.cadastrar_produto(produto);

    println!("Produto cadastrado com sucesso!");
}

fn consultar_produto(grafo: &Grafo) {
    let id = ler_u32("Digite o ID do produto: ");

    match grafo.consultar_produto(id) {
        Some(produto) => {
            println!("\nProduto encontrado:");

            println!("ID: {}", produto.id);

            println!("Nome: {}", produto.nome);

            println!("Categoria: {}", produto.categoria);

            println!("Preço: R$ {:.2}", produto.preco);
        }

        None => {
            println!("Produto não encontrado.");
        }
    }
}

fn conectar_produtos(grafo: &mut Grafo) {
    let produto1 = ler_u32("ID do primeiro produto: ");

    let produto2 = ler_u32("ID do segundo produto: ");

    if grafo.consultar_produto(produto1).is_none() || grafo.consultar_produto(produto2).is_none() {
        println!("Um dos produtos não existe.");

        return;
    }

    if grafo.conectar_produtos(produto1, produto2) {
        println!("Produtos conectados com sucesso!");
    } else {
        println!("Essa conexão já existe.");
    }
}

fn registrar_compra(grafo: &mut Grafo) {
    let cliente = ler_u32("ID do cliente: ");

    let produto = ler_u32("ID do produto comprado: ");

    if grafo.consultar_produto(produto).is_none() {
        println!("Produto não encontrado.");

        return;
    }

    grafo.registrar_compra(cliente, produto);

    println!("Compra registrada com sucesso!");
}

fn recomendar_produto(grafo: &Grafo) {
    let produto = ler_u32("ID do produto: ");

    if grafo.consultar_produto(produto).is_none() {
        println!("Produto não encontrado.");

        return;
    }

    let recomendacoes = grafo.recomendar_por_produto(produto);

    println!("\nRecomendações:");

    if recomendacoes.is_empty() {
        println!("Nenhuma recomendação encontrada.");

        return;
    }

    for produto in recomendacoes {
        println!(
            "{} - {} - R$ {:.2}",
            produto.id, produto.nome, produto.preco
        );
    }
}

fn recomendar_cliente(grafo: &Grafo) {
    let cliente = ler_u32("ID do cliente: ");

    let recomendacoes = grafo.recomendar_por_cliente(cliente);

    println!("\nRecomendações para o cliente:");

    if recomendacoes.is_empty() {
        println!("Nenhuma recomendação encontrada.");

        return;
    }

    for produto in recomendacoes {
        println!(
            "{} - {} - R$ {:.2}",
            produto.id, produto.nome, produto.preco
        );
    }
}

fn exemplo_inicial(grafo: &mut Grafo) {
    grafo.cadastrar_produto(Produto::novo(1, "Notebook", "Eletronicos", 3500.00));

    grafo.cadastrar_produto(Produto::novo(2, "Mouse", "Eletronicos", 80.00));

    grafo.cadastrar_produto(Produto::novo(3, "Teclado", "Eletronicos", 150.00));

    grafo.cadastrar_produto(Produto::novo(4, "Headset", "Eletronicos", 250.00));

    grafo.cadastrar_produto(Produto::novo(5, "Monitor", "Eletronicos", 1200.00));

    grafo.conectar_produtos(1, 2);
    grafo.conectar_produtos(1, 5);
    grafo.conectar_produtos(2, 3);
    grafo.conectar_produtos(2, 4);
    grafo.conectar_produtos(3, 4);

    grafo.registrar_compra(100, 1);

    grafo.registrar_compra(100, 2);

    println!("Dados de exemplo carregados.");
}

fn benchmark() {
    println!("\n=== TESTE DE DESEMPENHO ===");

    for quantidade in [100, 500, 1000] {
        let mut grafo = Grafo::novo();

        for i in 1..=quantidade {
            grafo.cadastrar_produto(Produto::novo(
                i,
                &format!("Produto {}", i),
                "Categoria",
                100.0,
            ));
        }

        for i in 1..quantidade {
            grafo.conectar_produtos(i, i + 1);
        }

        let inicio = Instant::now();

        let _ = grafo.recomendar_por_produto(1);

        let tempo = inicio.elapsed();

        println!("{} produtos -> {:?}", quantidade, tempo);
    }
}

fn main() {
    let mut grafo = Grafo::novo();

    exemplo_inicial(&mut grafo);

    loop {
        println!("\n==============================");

        println!("       CONECTASTORE");

        println!("==============================");

        println!("1 - Cadastrar produto");

        println!("2 - Consultar produto");

        println!("3 - Conectar produtos");

        println!("4 - Registrar compra");

        println!("5 - Recomendar por produto");

        println!("6 - Recomendar por cliente");

        println!("7 - Teste de desempenho");

        println!("0 - Sair");

        let opcao = ler_texto("Escolha uma opção: ");

        match opcao.as_str() {
            "1" => {
                cadastrar_produto(&mut grafo);
            }

            "2" => {
                consultar_produto(&grafo);
            }

            "3" => {
                conectar_produtos(&mut grafo);
            }

            "4" => {
                registrar_compra(&mut grafo);
            }

            "5" => {
                recomendar_produto(&grafo);
            }

            "6" => {
                recomendar_cliente(&grafo);
            }

            "7" => {
                benchmark();
            }

            "0" => {
                println!("Sistema encerrado.");

                break;
            }

            _ => {
                println!("Opção inválida.");
            }
        }
    }
}
