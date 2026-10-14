use std::collections::{HashMap, HashSet, VecDeque};

use crate::produto::Produto;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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

    pub fn cadastrar_produto(&mut self, produto: Produto) {
        let id = produto.id;

        self.produtos.insert(id, produto);

        self.conexoes.entry(Vertice::Produto(id)).or_default();
    }

    pub fn consultar_produto(&self, id: u32) -> Option<&Produto> {
        self.produtos.get(&id)
    }

    pub fn conectar_produtos(&mut self, produto1: u32, produto2: u32) -> bool {
        if produto1 == produto2
            || !self.produtos.contains_key(&produto1)
            || !self.produtos.contains_key(&produto2)
        {
            return false;
        }

        let vertice1 = Vertice::Produto(produto1);
        let vertice2 = Vertice::Produto(produto2);

        let conexao_existe = self
            .conexoes
            .get(&vertice1)
            .is_some_and(|vizinhos| vizinhos.contains(&vertice2));

        if conexao_existe {
            return false;
        }

        self.conexoes.entry(vertice1).or_default().push(vertice2);

        self.conexoes.entry(vertice2).or_default().push(vertice1);

        true
    }

    pub fn registrar_compra(&mut self, cliente: u32, produto: u32) -> bool {
        if !self.produtos.contains_key(&produto) {
            return false;
        }

        let vertice_cliente = Vertice::Cliente(cliente);
        let vertice_produto = Vertice::Produto(produto);

        let conexao_existe = self
            .conexoes
            .get(&vertice_cliente)
            .is_some_and(|vizinhos| vizinhos.contains(&vertice_produto));

        if !conexao_existe {
            self.conexoes
                .entry(vertice_cliente)
                .or_default()
                .push(vertice_produto);

            self.conexoes
                .entry(vertice_produto)
                .or_default()
                .push(vertice_cliente);
        }

        self.compras.entry(cliente).or_default().insert(produto);

        true
    }

    pub fn recomendar_por_produto(&self, produto_id: u32) -> Vec<&Produto> {
        if !self.produtos.contains_key(&produto_id) {
            return Vec::new();
        }

        let inicio = Vertice::Produto(produto_id);
        let mut fila = VecDeque::new();
        let mut visitados = HashSet::new();
        let mut recomendacoes = Vec::new();

        fila.push_back(inicio);
        visitados.insert(inicio);

        while let Some(atual) = fila.pop_front() {
            if let Some(vizinhos) = self.conexoes.get(&atual) {
                for vizinho in vizinhos {
                    if visitados.insert(*vizinho) {
                        fila.push_back(*vizinho);

                        if let Vertice::Produto(id) = vizinho {
                            if *id != produto_id {
                                if let Some(produto) = self.produtos.get(id) {
                                    recomendacoes.push(produto);
                                }
                            }
                        }
                    }
                }
            }
        }

        recomendacoes
    }

    pub fn recomendar_por_cliente(&self, cliente: u32) -> Vec<&Produto> {
        let inicio = Vertice::Cliente(cliente);
        let mut fila = VecDeque::new();
        let mut visitados = HashSet::new();
        let mut recomendacoes = Vec::new();

        let comprados = self.compras.get(&cliente);

        fila.push_back(inicio);
        visitados.insert(inicio);

        while let Some(atual) = fila.pop_front() {
            if let Some(vizinhos) = self.conexoes.get(&atual) {
                for vizinho in vizinhos {
                    if visitados.insert(*vizinho) {
                        fila.push_back(*vizinho);

                        if let Vertice::Produto(id) = vizinho {
                            let ja_comprado =
                                comprados.is_some_and(|produtos| produtos.contains(id));

                            if !ja_comprado {
                                if let Some(produto) = self.produtos.get(id) {
                                    recomendacoes.push(produto);
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

#[cfg(test)]
mod testes {
    use super::*;

    fn criar_produto(id: u32, nome: &str) -> Produto {
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

        grafo.cadastrar_produto(criar_produto(1, "Notebook"));

        assert!(grafo.produtos.contains_key(&1));
        assert!(grafo.conexoes.contains_key(&Vertice::Produto(1)));
    }

    #[test]
    fn deve_consultar_produto() {
        let mut grafo = Grafo::novo();

        grafo.cadastrar_produto(criar_produto(1, "Notebook"));

        let produto = grafo.consultar_produto(1).unwrap();

        assert_eq!(produto.nome, "Notebook");
    }

    #[test]
    fn nao_deve_consultar_produto_inexistente() {
        let grafo = Grafo::novo();

        assert!(grafo.consultar_produto(999).is_none());
    }

    #[test]
    fn deve_conectar_produtos() {
        let mut grafo = Grafo::novo();

        grafo.cadastrar_produto(criar_produto(1, "Notebook"));
        grafo.cadastrar_produto(criar_produto(2, "Mouse"));

        assert!(grafo.conectar_produtos(1, 2));

        let conexoes = grafo.conexoes.get(&Vertice::Produto(1)).unwrap();

        assert!(conexoes.contains(&Vertice::Produto(2)));
    }

    #[test]
    fn nao_deve_criar_conexao_duplicada() {
        let mut grafo = Grafo::novo();

        grafo.cadastrar_produto(criar_produto(1, "Notebook"));
        grafo.cadastrar_produto(criar_produto(2, "Mouse"));

        assert!(grafo.conectar_produtos(1, 2));
        assert!(!grafo.conectar_produtos(1, 2));

        let conexoes = grafo.conexoes.get(&Vertice::Produto(1)).unwrap();

        assert_eq!(conexoes.len(), 1);
    }

    #[test]
    fn nao_deve_conectar_produtos_inexistentes() {
        let mut grafo = Grafo::novo();

        grafo.cadastrar_produto(criar_produto(1, "Notebook"));

        assert!(!grafo.conectar_produtos(1, 999));
    }

    #[test]
    fn nao_deve_conectar_produto_a_ele_mesmo() {
        let mut grafo = Grafo::novo();

        grafo.cadastrar_produto(criar_produto(1, "Notebook"));

        assert!(!grafo.conectar_produtos(1, 1));
    }

    #[test]
    fn deve_registrar_compra() {
        let mut grafo = Grafo::novo();

        grafo.cadastrar_produto(criar_produto(1, "Notebook"));

        assert!(grafo.registrar_compra(100, 1));
        assert!(grafo.compras.get(&100).unwrap().contains(&1));
    }

    #[test]
    fn nao_deve_registrar_compra_de_produto_inexistente() {
        let mut grafo = Grafo::novo();

        assert!(!grafo.registrar_compra(100, 999));
    }

    #[test]
    fn nao_deve_duplicar_compra() {
        let mut grafo = Grafo::novo();

        grafo.cadastrar_produto(criar_produto(1, "Notebook"));

        assert!(grafo.registrar_compra(100, 1));
        assert!(grafo.registrar_compra(100, 1));

        assert_eq!(grafo.compras.get(&100).unwrap().len(), 1);
    }

    #[test]
    fn deve_recomendar_produtos_conectados() {
        let mut grafo = Grafo::novo();

        grafo.cadastrar_produto(criar_produto(1, "Notebook"));
        grafo.cadastrar_produto(criar_produto(2, "Mouse"));

        grafo.conectar_produtos(1, 2);

        let recomendacoes = grafo.recomendar_por_produto(1);

        assert_eq!(recomendacoes.len(), 1);
        assert_eq!(recomendacoes[0].id, 2);
    }

    #[test]
    fn nao_deve_recomendar_produto_inexistente() {
        let grafo = Grafo::novo();

        assert!(grafo.recomendar_por_produto(999).is_empty());
    }

    #[test]
    fn nao_deve_recomendar_produtos_ja_comprados() {
        let mut grafo = Grafo::novo();

        grafo.cadastrar_produto(criar_produto(1, "Notebook"));
        grafo.cadastrar_produto(criar_produto(2, "Mouse"));
        grafo.cadastrar_produto(criar_produto(3, "Teclado"));

        grafo.conectar_produtos(1, 2);
        grafo.conectar_produtos(2, 3);

        grafo.registrar_compra(100, 1);
        grafo.registrar_compra(100, 2);

        let recomendacoes = grafo.recomendar_por_cliente(100);

        assert!(recomendacoes.iter().all(|p| p.id != 1 && p.id != 2));
    }
}
