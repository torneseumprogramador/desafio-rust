use std::collections::HashMap;
use std::io;
use std::str::FromStr;

#[derive(Debug)]
enum AcoesMenu {
    CadastrarAluno,
    AlterarAluno,
    ExcluirAluno,
    Sair
}

impl FromStr for AcoesMenu {
    type Err = (); // Pode ser substituído por um tipo de erro mais específico

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim() {
            "CadastrarAluno" => Ok(AcoesMenu::CadastrarAluno),
            "AlterarAluno" => Ok(AcoesMenu::AlterarAluno),
            "ExcluirAluno" => Ok(AcoesMenu::ExcluirAluno),
            "Sair" => Ok(AcoesMenu::Sair),
            _ => Err(()),
        }
    }
}

impl AcoesMenu {
    fn opcoes() -> Vec<HashMap<&'static str, &'static str>> {
        let mut opcoes: Vec<HashMap<&str, &str>> = Vec::new();

        let mut opcao = HashMap::new();
        opcao.insert("CadastrarAluno", "Cadastrar um aluno");
        opcoes.push(opcao);

        let mut opcao = HashMap::new();
        opcao.insert("AlterarAluno", "Alterar um aluno");
        opcoes.push(opcao);

        let mut opcao = HashMap::new();
        opcao.insert("ExcluirAluno", "Excluir um aluno");
        opcoes.push(opcao);

        let mut opcao = HashMap::new();
        opcao.insert("Sair", "Sair do programa");
        opcoes.push(opcao);
        
        return opcoes;
    }
}


fn main(){

    loop {
        print!("\n");
        println!("Digite uma das opção abaixo:");
        println!("-----------------------");

        for opcoes in AcoesMenu::opcoes().iter() {
            let key = opcoes.iter().next().map(|(k, _)| *k).unwrap();
            let value = opcoes[key];
            println!("{} - {:?}", key, value);
        }

        println!("-----------------------");

        let mut opcao = String::new();
        io::stdin().read_line(&mut opcao).expect("Falha ao ler entrada");

        let opcao: Option<AcoesMenu> = match opcao.parse::<AcoesMenu>() {
            Ok(acao) => Some(acao),
            Err(_) => None,
        };

        match opcao {
            Some(acoes) => {
                match acoes {
                    AcoesMenu::CadastrarAluno => {
                        println!("Cadastrar ...");
                    },
                    AcoesMenu::AlterarAluno => {
                        println!("Alterar ...");
                    },
                    AcoesMenu::ExcluirAluno => {
                        println!("Excluindo ...");
                    },
                    AcoesMenu::Sair => {
                        println!("Saindo do programa...");
                        break;
                    }
                }
            }
            _ => {
                println!("Opção inválida!");
            }
        }
    }
}

