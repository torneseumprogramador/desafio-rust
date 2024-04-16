use macro_derive_traits::MeuTrait;
use macro_derive::MeuTrait;

#[derive(MeuTrait)]
struct TipoPadrao {}

struct TipoCustomizado {}
impl MeuTrait for TipoCustomizado {
    fn minha_funcao(&self) -> String {
        format!("Aqui tem um método novo criado pelo codigo")
    }
}


fn imprime_mensagem<T: MeuTrait>(item: &T) {
    println!("Retorno: {}", item.minha_funcao());
}

fn main() {
    let tipo = TipoPadrao {};
    imprime_mensagem(&tipo);

    let tipo2 = TipoCustomizado {};
    imprime_mensagem(&tipo2);
}