use std::collections::{HashMap, HashSet, VecDeque};

use crate::produto::Produto;

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub enum Vertice {
    Produto(u32),
    Cliente(u32),
}

pub struct Grafo {
    pub produtos: HashMap<u32, Produto>,
    pub conexoes: HashMap<Vertice, Vec<Vertice>>,
    pub compras: HashMap<u32, HashSet<u32>>,
}

impl Grafo {
    pub fn novo() -> Grafo {
        Grafo {
            produtos: HashMap::new(),
            conexoes: HashMap::new(),
            compras: HashMap::new(),
        }
    }

    // Cadastra um produto no HashMap
    pub fn cadastrar_produto(&mut self, produto: Produto) {
        let id = produto.id;

        self.produtos.insert(id, produto);

        self.conexoes
            .entry(Vertice::Produto(id))
            .or_insert_with(Vec::new);
    }

    // Consulta um produto pelo ID
    pub fn consultar_produto(&self, id: u32) -> Option<&Produto> {
        self.produtos.get(&id)
    }

    // Cria uma conexão entre dois produtos
    pub fn conectar_produtos(&mut self, produto1: u32, produto2: u32) -> bool {
        let v1 = Vertice::Produto(produto1);
        let v2 = Vertice::Produto(produto2);

        self.conexoes.entry(v1.clone()).or_insert_with(Vec::new);

        self.conexoes.entry(v2.clone()).or_insert_with(Vec::new);

        // Verifica se a conexão já existe
        let ja_existe = self.conexoes.get(&v1).unwrap().contains(&v2);

        if !ja_existe {
            self.conexoes.get_mut(&v1).unwrap().push(v2.clone());
            self.conexoes.get_mut(&v2).unwrap().push(v1);

            true
        } else {
            false
        }
    }

    // Registra uma compra de um cliente
    pub fn registrar_compra(&mut self, cliente: u32, produto: u32) {
        let cliente_vertice = Vertice::Cliente(cliente);
        let produto_vertice = Vertice::Produto(produto);

        self.conexoes
            .entry(cliente_vertice.clone())
            .or_insert_with(Vec::new);

        self.conexoes
            .entry(produto_vertice.clone())
            .or_insert_with(Vec::new);

        // Conecta cliente ao produto
        if !self
            .conexoes
            .get(&cliente_vertice)
            .unwrap()
            .contains(&produto_vertice)
        {
            self.conexoes
                .get_mut(&cliente_vertice)
                .unwrap()
                .push(produto_vertice.clone());
        }

        // Conecta produto ao cliente
        if !self
            .conexoes
            .get(&produto_vertice)
            .unwrap()
            .contains(&cliente_vertice)
        {
            self.conexoes
                .get_mut(&produto_vertice)
                .unwrap()
                .push(cliente_vertice);
        }

        // Registra o produto comprado pelo cliente
        self.compras
            .entry(cliente)
            .or_insert_with(HashSet::new)
            .insert(produto);
    }

    // BFS para recomendar produtos a partir de um produto
    pub fn recomendar_por_produto(&self, produto_inicial: u32) -> Vec<Produto> {
        let inicio = Vertice::Produto(produto_inicial);

        let mut fila = VecDeque::new();
        let mut visitados = HashSet::new();
        let mut recomendacoes = Vec::new();

        fila.push_back(inicio.clone());
        visitados.insert(inicio);

        while let Some(atual) = fila.pop_front() {
            if let Some(vizinhos) = self.conexoes.get(&atual) {
                for vizinho in vizinhos {
                    if !visitados.contains(vizinho) {
                        visitados.insert(vizinho.clone());
                        fila.push_back(vizinho.clone());

                        if let Vertice::Produto(id) = vizinho {
                            if *id != produto_inicial {
                                if let Some(produto) = self.produtos.get(id) {
                                    if !recomendacoes.iter().any(|p: &Produto| p.id == produto.id) {
                                        recomendacoes.push(produto.clone());
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        recomendacoes
    }

    // BFS para recomendar produtos a partir de um cliente
    pub fn recomendar_por_cliente(&self, cliente: u32) -> Vec<Produto> {
        let inicio = Vertice::Cliente(cliente);

        let mut fila = VecDeque::new();
        let mut visitados = HashSet::new();
        let mut recomendacoes = Vec::new();

        fila.push_back(inicio.clone());
        visitados.insert(inicio);

        while let Some(atual) = fila.pop_front() {
            if let Some(vizinhos) = self.conexoes.get(&atual) {
                for vizinho in vizinhos {
                    if !visitados.contains(vizinho) {
                        visitados.insert(vizinho.clone());
                        fila.push_back(vizinho.clone());

                        if let Vertice::Produto(id) = vizinho {
                            let ja_comprou = self
                                .compras
                                .get(&cliente)
                                .map(|compras| compras.contains(id))
                                .unwrap_or(false);

                            if !ja_comprou {
                                if let Some(produto) = self.produtos.get(id) {
                                    if !recomendacoes.iter().any(|p: &Produto| p.id == produto.id) {
                                        recomendacoes.push(produto.clone());
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        recomendacoes
    }
}

// =====================================================
// TESTES UNITÁRIOS
// =====================================================

#[cfg(test)]
mod testes {
    use super::*;
    use crate::produto::Produto;

    fn produto_teste(id: u32, nome: &str) -> Produto {
        Produto::novo(id, nome, "Eletronicos", 100.0)
    }

    #[test]
    fn deve_criar_grafo_vazio() {
        let grafo = Grafo::novo();

        assert!(grafo.produtos.is_empty());
        assert!(grafo.conexoes.is_empty());
        assert!(grafo.compras.is_empty());
    }

    #[test]
    fn deve_cadastrar_produto() {
        let mut grafo = Grafo::novo();

        grafo.cadastrar_produto(produto_teste(1, "Notebook"));

        assert!(grafo.consultar_produto(1).is_some());
    }

    #[test]
    fn deve_consultar_produto() {
        let mut grafo = Grafo::novo();

        grafo.cadastrar_produto(produto_teste(1, "Notebook"));

        let produto = grafo.consultar_produto(1);

        assert_eq!(produto.unwrap().nome, "Notebook");
    }

    #[test]
    fn produto_inexistente_deve_retornar_none() {
        let grafo = Grafo::novo();

        assert!(grafo.consultar_produto(999).is_none());
    }

    #[test]
    fn deve_conectar_produtos() {
        let mut grafo = Grafo::novo();

        grafo.cadastrar_produto(produto_teste(1, "Notebook"));
        grafo.cadastrar_produto(produto_teste(2, "Mouse"));

        grafo.conectar_produtos(1, 2);

        let conexoes = grafo.conexoes.get(&Vertice::Produto(1)).unwrap();

        assert!(conexoes.contains(&Vertice::Produto(2)));
    }

    #[test]
    fn nao_deve_duplicar_conexao() {
        let mut grafo = Grafo::novo();

        grafo.cadastrar_produto(produto_teste(1, "Notebook"));
        grafo.cadastrar_produto(produto_teste(2, "Mouse"));

        grafo.conectar_produtos(1, 2);
        grafo.conectar_produtos(1, 2);

        let conexoes = grafo.conexoes.get(&Vertice::Produto(1)).unwrap();

        assert_eq!(conexoes.len(), 1);
    }

    #[test]
    fn deve_registrar_compra() {
        let mut grafo = Grafo::novo();

        grafo.cadastrar_produto(produto_teste(1, "Notebook"));

        grafo.registrar_compra(100, 1);

        assert!(grafo.compras.get(&100).unwrap().contains(&1));
    }

    #[test]
    fn deve_recomendar_produto_conectado() {
        let mut grafo = Grafo::novo();

        grafo.cadastrar_produto(produto_teste(1, "Notebook"));
        grafo.cadastrar_produto(produto_teste(2, "Mouse"));

        grafo.conectar_produtos(1, 2);

        let recomendacoes = grafo.recomendar_por_produto(1);

        assert_eq!(recomendacoes.len(), 1);
        assert_eq!(recomendacoes[0].id, 2);
    }

    #[test]
    fn cliente_nao_deve_receber_produto_ja_comprado() {
        let mut grafo = Grafo::novo();

        grafo.cadastrar_produto(produto_teste(1, "Notebook"));
        grafo.cadastrar_produto(produto_teste(2, "Mouse"));

        grafo.conectar_produtos(1, 2);

        grafo.registrar_compra(100, 1);
        grafo.registrar_compra(100, 2);

        let recomendacoes = grafo.recomendar_por_cliente(100);

        assert!(!recomendacoes.iter().any(|p| p.id == 1));
        assert!(!recomendacoes.iter().any(|p| p.id == 2));
    }
}
