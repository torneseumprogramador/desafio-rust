// use std::fs::File;

// fn main() {
//     let f = File::open("hello.txt");

//     let f = match f {
//         Ok(file) => file,
//         Err(error) => {
//             println!("====== Problema ao abrir o arquivo ======");
//             panic!("====== {:?} ======", error)
//         }
//     };

//     println!("{:?}", f)
// }






// fn main() {
//     panic!("Erro ao rodar o programa");
//     println!("Aula - tratamento de erro")
// }






// use std::{fs::File, io::Error};

// fn main() -> Result<(), Error> {
//     let f = File::open("hello.txt")?;

//     println!("{:?}", f);

//     Ok(())
// }








// pub struct Aluno {
//     pub nome: String,
//     pub matricula: String,
//     pub notas: Vec<f32>
// }

// fn validacao_presenca(aluno: &Aluno) -> Result<bool, String> {
//     if aluno.nome.trim().is_empty() {
//         return Err(String::from("Nome é obrigatório"));
//     }

//     Ok(true)
// }

// fn main() -> Result<(), String>{
//     let aluno = Aluno {
//         nome: "".to_string(),
//         matricula: "1".to_string(),
//         notas: vec![9.0,4.5]
//     };

//     let validado = validacao_presenca(&aluno)?;

//     if validado {
//         println!("A validação está correta")
//     };

//     Ok(())
// }







// pub struct Aluno {
//     pub nome: String,
//     pub matricula: String,
//     pub notas: Vec<f32>
// }

// fn validacao_presenca(aluno: &Aluno) -> Result<bool, String> {
//     if aluno.nome.trim().is_empty() {
//         return Err(String::from("Nome é obrigatório"));
//     }

//     Ok(true)
// }

// fn main() {
//     let aluno = Aluno {
//         nome: "".to_string(),
//         matricula: "1".to_string(),
//         notas: vec![9.0,4.5]
//     };

//     let validado = match validacao_presenca(&aluno) {
//         Ok(resultado) => resultado,
//         Err(erro_str) => {
//             // panic!("Erro ao retornar dados - {}", erro_str)
//             println!("Erro ao retornar dados - {}", erro_str);
//             false
//         }
//     };

//     if validado {
//         println!("A validação está correta")
//     };
// }




// struct Aluno {
//     pub nome: String,
//     pub matricula: String,
//     pub notas: Vec<f32>
// }

// fn validacao_presenca(aluno: &Aluno) -> bool {
//     if aluno.nome.trim().is_empty() {
//         println!("Nome é obrigatório");
//         return false;
//     }

//     true
// }

// fn main() {
//     let aluno = Aluno {
//         nome: "".to_string(),
//         matricula: "1".to_string(),
//         notas: vec![9.0,4.5]
//     };

//     let validado = validacao_presenca(&aluno);

//     if validado {
//         println!("A validação está correta")
//     };
// }







// use std::panic;

// struct Aluno {
//     pub nome: String,
//     pub matricula: String,
//     pub notas: Vec<f32>
// }

// fn validacao_presenca(aluno: &Aluno) -> bool {
//     if aluno.nome.trim().is_empty() {
//         panic!("Nome é obrigatório");
//     }

//     true
// }

// fn main() {
//     let aluno = Aluno {
//         nome: "".to_string(),
//         matricula: "1".to_string(),
//         notas: vec![9.0,4.5]
//     };

//     let validado = validacao_presenca(&aluno);

//     if validado {
//         println!("A validação está correta")
//     };
// }




// pub struct Aluno {
//     pub nome: String,
//     pub matricula: String,
//     pub notas: Vec<f32>,
// }

// fn validacao_presenca(aluno: &Aluno) -> Result<bool, String> {
//     if aluno.nome.trim().is_empty() {
//         return Err(String::from("Nome é obrigatório"));
//     }

//     Ok(true)
// }

// fn main() {
//     let aluno = Aluno {
//         nome: "".to_string(),
//         matricula: "1".to_string(),
//         notas: vec![9.0,4.5]
//     };

//     // não é a melhor forma, mas é uma opção - Isso cai em erro de lint
//     let validado = validacao_presenca(&aluno).unwrap();

//     if validado {
//         println!("A validação está correta")
//     };
// }





struct MeuEmptyErro {
    mensagem: String,
}

struct MeuSizeErro {
    mensagem: String,
}

enum TipoErro {
    MeuEmptyErro(MeuEmptyErro),
    MeuSizeErro(MeuSizeErro),
}

pub struct Aluno {
    pub nome: String,
    pub matricula: String,
    pub notas: Vec<f32>,
}

fn validacao_presenca(aluno: &Aluno) -> Result<bool, TipoErro> {
    if aluno.nome.trim().is_empty() {
        return Err(
            TipoErro::MeuEmptyErro(
                MeuEmptyErro { mensagem: String::from("Nome é obrigatório") }
            )
        );
    }

    if aluno.nome.trim().chars().count() < 10 {
        return Err(
            TipoErro::MeuSizeErro(
                MeuSizeErro { mensagem: String::from("Nome precisa ser igual ou maior que 10 caracteres") }
            )
        );
    }

    Ok(true)
}

fn main() {
    let aluno = Aluno {
        nome: "Danilo Aparecido dos Santos".to_string(),
        matricula: "1".to_string(),
        notas: vec![9.0, 4.5],
    };

    let validado = match validacao_presenca(&aluno) {
        Ok(_) => true,
        Err(tipo_erro) => {
            match tipo_erro {
                TipoErro::MeuEmptyErro(val) => {
                    println!("Presença - {}", val.mensagem);
                },
                TipoErro::MeuSizeErro(val) => {
                    println!("Tamanho - {}", val.mensagem);
                },
            };
            
            false
        }
    };

    if validado {
        println!("A validação está correta");
    }
}




