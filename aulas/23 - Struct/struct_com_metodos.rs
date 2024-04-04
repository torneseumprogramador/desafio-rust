struct Aluno {
    nome: String,
    matricula: String,
    notas: Vec<f32>
}

impl Aluno {
    fn mostrar(&self){
        println!("Nome: {}", self.nome);
        println!("Matricula: {}", self.matricula);
        println!("Notas: {:?}", self.notas);
        println!("Media: {:?}", self.media());
    }
}

impl Aluno {
    // fn media_imperativa(&self) -> f32 {
    //     let mut soma: f32 = 0.0;
    //     for valor in self.notas.iter() {
    //         soma += valor
    //     }

    //     let media = soma / self.notas.len() as f32;

    //     return media;
    // }

    fn media(&self) -> f32 {
        let soma: f32 = self.notas.iter().sum();
        soma / self.notas.len() as f32
    }

    fn media_com_ponto_extra(&self, pontos_extras: f32) -> f32 {
        let soma: f32 = self.notas.iter().sum();
        let media = soma / self.notas.len() as f32;
        media + pontos_extras
    }

    // metodo like statico do java
    fn metodo_statico(param1: String){
        println!("O valor recebido foi {}", param1)
    }
}

fn main(){
    let aluno = Aluno {
        nome: String::from("Heli Ribeiro"),
        matricula: String::from("HLI330"),
        notas: vec![
            8.0,
            9.0,
            5.0
        ]
    };


    aluno.mostrar();

    let extra = 2.0;
    println!("O aluno {}, recebeu {} pontos extras e sua média é de {}", aluno.nome, extra, aluno.media_com_ponto_extra(extra));


    Aluno::metodo_statico("Testeeeeee".to_string());
}

