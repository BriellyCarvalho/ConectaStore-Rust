mod grafo;
mod produto;

use grafo::Grafo;
use produto::Produto;
use std::io::{self, Write};
use std::time::Instant;

fn main() {
    let mut grafo = Grafo::novo();

    exemplo_inicial(&mut grafo);

    loop {
        println!("\n========== CONECTASTORE ==========");
        println!("1 - Cadastrar produto");
        println!("2 - Consultar produto");
        println!("3 - Conectar produtos");
        println!("4 - Registrar compra");
        println!("5 - Recomendar produtos");
        println!("6 - Listar produtos");
        println!("7 - Teste de desempenho");
        println!("0 - Sair");
        println!("==================================");

        let opcao = ler_u32("Escolha uma opção: ");

        match opcao {
            1 => cadastrar_produto(&mut grafo),
            2 => consultar_produto(&grafo),
            3 => conectar_produtos(&mut grafo),
            4 => registrar_compra(&mut grafo),
            5 => recomendar_produtos(&grafo),
            6 => listar_produtos(&grafo),
            7 => teste_desempenho(),
            0 => {
                println!("Encerrando o ConectaStore. Até mais!");
                break;
            }
            _ => println!("Opção inválida. Tente novamente."),
        }
    }
}

fn cadastrar_produto(grafo: &mut Grafo) {
    let id = ler_u32("ID do produto: ");

    if grafo.consultar_produto(id).is_some() {
        println!("Já existe um produto com esse ID.");
        return;
    }

    let nome = ler_texto("Nome do produto: ");
    let categoria = ler_texto("Categoria: ");
    let preco = ler_f64("Preço do produto: ");

    let produto = Produto::novo(id, &nome, &categoria, preco);
    grafo.cadastrar_produto(produto);

    println!("Produto cadastrado com sucesso!");
}

fn consultar_produto(grafo: &Grafo) {
    let id = ler_u32("ID do produto que deseja consultar: ");

    match grafo.consultar_produto(id) {
        Some(produto) => {
            println!("\n--- Produto encontrado ---");
            println!("ID: {}", produto.id);
            println!("Nome: {}", produto.nome);
            println!("Categoria: {}", produto.categoria);
            println!("Preço: R$ {:.2}", produto.preco);
        }
        None => println!("Produto não encontrado."),
    }
}

fn conectar_produtos(grafo: &mut Grafo) {
    let produto1 = ler_u32("ID do primeiro produto: ");
    let produto2 = ler_u32("ID do segundo produto: ");

    if grafo.consultar_produto(produto1).is_none() || grafo.consultar_produto(produto2).is_none() {
        println!("Um dos produtos não existe.");
        return;
    }

    if produto1 == produto2 {
        println!("Não é possível conectar um produto a ele mesmo.");
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

    if grafo.registrar_compra(cliente, produto) {
        println!("Compra registrada com sucesso!");
    } else {
        println!("Não foi possível registrar a compra. Verifique o produto.");
    }
}

fn recomendar_produtos(grafo: &Grafo) {
    println!("\n--- Recomendações ---");
    println!("1 - Recomendar a partir de um produto");
    println!("2 - Recomendar para um cliente");

    let opcao = ler_u32("Escolha uma opção: ");

    match opcao {
        1 => {
            let id = ler_u32("ID do produto de referência: ");

            if grafo.consultar_produto(id).is_none() {
                println!("Produto não encontrado.");
                return;
            }

            let recomendacoes = grafo.recomendar_por_produto(id);

            if recomendacoes.is_empty() {
                println!("Nenhuma recomendação encontrada.");
            } else {
                println!("\nProdutos recomendados:");

                for produto in recomendacoes {
                    println!(
                        "ID: {} | Nome: {} | Categoria: {} | Preço: R$ {:.2}",
                        produto.id, produto.nome, produto.categoria, produto.preco
                    );
                }
            }
        }
        2 => {
            let cliente = ler_u32("ID do cliente: ");
            let recomendacoes = grafo.recomendar_por_cliente(cliente);

            if recomendacoes.is_empty() {
                println!("Nenhuma recomendação encontrada para esse cliente.");
            } else {
                println!("\nProdutos recomendados:");

                for produto in recomendacoes {
                    println!(
                        "ID: {} | Nome: {} | Categoria: {} | Preço: R$ {:.2}",
                        produto.id, produto.nome, produto.categoria, produto.preco
                    );
                }
            }
        }
        _ => println!("Opção inválida."),
    }
}

fn listar_produtos(grafo: &Grafo) {
    if grafo.produtos.is_empty() {
        println!("Nenhum produto cadastrado.");
        return;
    }

    println!("\n--- Produtos cadastrados ---");

    let mut produtos: Vec<_> = grafo.produtos.values().collect();
    produtos.sort_by_key(|produto| produto.id);

    for produto in produtos {
        println!(
            "ID: {} | Nome: {} | Categoria: {} | Preço: R$ {:.2}",
            produto.id, produto.nome, produto.categoria, produto.preco
        );
    }
}

fn exemplo_inicial(grafo: &mut Grafo) {
    let produtos = vec![
        Produto::novo(1, "Notebook", "Eletronicos", 3500.00),
        Produto::novo(2, "Mouse", "Eletronicos", 80.00),
        Produto::novo(3, "Teclado", "Eletronicos", 150.00),
        Produto::novo(4, "Headset", "Eletronicos", 250.00),
        Produto::novo(5, "Monitor", "Eletronicos", 1200.00),
    ];

    for produto in produtos {
        grafo.cadastrar_produto(produto);
    }

    grafo.conectar_produtos(1, 2);
    grafo.conectar_produtos(1, 3);
    grafo.conectar_produtos(2, 3);
    grafo.conectar_produtos(2, 4);
    grafo.conectar_produtos(3, 4);
    grafo.conectar_produtos(1, 5);

    grafo.registrar_compra(100, 1);
    grafo.registrar_compra(100, 2);

    println!("Produtos de exemplo carregados!");
}

fn teste_desempenho() {
    println!("\n--- Teste de desempenho ---");

    let quantidades = [100, 500, 1000];

    for quantidade in quantidades {
        let inicio = Instant::now();
        let mut grafo = Grafo::novo();

        for id in 1..=quantidade {
            let nome = format!("Produto {}", id);
            let produto = Produto::novo(id, &nome, "Teste", 100.0);

            grafo.cadastrar_produto(produto);
        }

        for id in 1..quantidade {
            grafo.conectar_produtos(id, id + 1);
        }

        let duracao = inicio.elapsed();

        println!("{} produtos: {:?}", quantidade, duracao);
    }
}

fn ler_texto(mensagem: &str) -> String {
    loop {
        print!("{}", mensagem);
        io::stdout().flush().expect("Erro ao exibir mensagem.");

        let mut entrada = String::new();

        match io::stdin().read_line(&mut entrada) {
            Ok(_) => {
                let entrada = entrada.trim().to_string();

                if !entrada.is_empty() {
                    return entrada;
                }

                println!("A entrada não pode ficar vazia.");
            }
            Err(_) => println!("Erro ao ler a entrada. Tente novamente."),
        }
    }
}

fn ler_u32(mensagem: &str) -> u32 {
    loop {
        let entrada = ler_texto(mensagem);

        match entrada.parse::<u32>() {
            Ok(valor) => return valor,
            Err(_) => println!("Digite um número inteiro válido e não negativo."),
        }
    }
}

fn ler_f64(mensagem: &str) -> f64 {
    loop {
        let entrada = ler_texto(mensagem);

        match entrada.parse::<f64>() {
            Ok(valor) if valor.is_finite() && valor >= 0.0 => return valor,
            _ => println!("Digite um preço válido, maior ou igual a zero."),
        }
    }
}
