


enum TipoPessoa{
    Fisica,
    Juridica
}

struct Pessoa{
    id: u32,
    nome: String,
    documento: String,
    tipo: TipoPessoa
}

fn mostra_pessoa(pessoa: Pessoa){
    println!("Id: {}", pessoa.id);
    println!("Nome: {}", pessoa.nome);
    println!("Documento: {}", pessoa.documento);
    match pessoa.tipo {
        TipoPessoa::Fisica => {
            println!("Tipo: Fisica");
        },
        TipoPessoa::Juridica => {
            println!("Tipo: Juridica");
        }
    }
}

fn main(){
    let pessoa_fisica = Pessoa {
        id: 1,
        nome: "Danilo".to_string(),
        documento: "222.222.222-11".to_string(),
        tipo: TipoPessoa::Fisica
    };

    let pessoa_juridica = Pessoa {
        id: 1,
        nome: "C&C".to_string(),
        documento: "22.222.666/0001-22".to_string(),
        tipo: TipoPessoa::Juridica
    };

    mostra_pessoa(pessoa_fisica);
    mostra_pessoa(pessoa_juridica);
}