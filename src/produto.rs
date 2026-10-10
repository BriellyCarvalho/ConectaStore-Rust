#[derive(Clone, Debug)]
pub struct Produto {
    pub id: u32,
    pub nome: String,
    pub categoria: String,
    pub preco: f64,
}

impl Produto {
    pub fn novo(id: u32, nome: &str, categoria: &str, preco: f64) -> Produto {
        Produto {
            id,
            nome: nome.to_string(),
            categoria: categoria.to_string(),
            preco,
        }
    }
}

#[cfg(test)]
mod testes {
    use super::*;

    #[test]
    fn deve_criar_produto_corretamente() {
        let produto = Produto::novo(1, "Notebook", "Eletronicos", 3500.00);

        assert_eq!(produto.id, 1);
        assert_eq!(produto.nome, "Notebook");
        assert_eq!(produto.categoria, "Eletronicos");
        assert_eq!(produto.preco, 3500.00);
    }

    #[test]
    fn deve_criar_produto_com_preco_diferente() {
        let produto = Produto::novo(2, "Mouse", "Eletronicos", 80.00);

        assert_eq!(produto.preco, 80.00);
    }
}
