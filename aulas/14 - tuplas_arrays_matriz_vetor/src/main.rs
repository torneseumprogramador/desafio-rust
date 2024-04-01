// fn main() {
//     // Criando uma tupla com três elementos de tipos diferentes
//     let tupla: (i32, f64, u8) = (500, 6.4, 66);

//     // Acessando os elementos da tupla
//     let (x, y, z) = tupla;

//     println!("O valor de x é: {}", x);
//     println!("O valor de y é: {}", y);
//     println!("O valor de z é: {}", z);

//     // Acessando diretamente os elementos da tupla
//     println!("O primeiro valor é: {}", tupla.0);
//     println!("O segundo valor é: {}", tupla.1);
//     println!("O terceiro valor é: {}", tupla.2);
// }


// fn calcular_dimensoes() -> (i32, i32) {
//     // Suponha que esses valores foram calculados
//     let largura = 30;
//     let altura = 50;
//     (largura, altura) // Retornando uma tupla
// }

// fn main() {
//     let dimensoes = calcular_dimensoes();
//     println!("Largura: {}, Altura: {}", dimensoes.0, dimensoes.1);

//     let (largura, altura) = calcular_dimensoes();
//     println!("Largura: {}, Altura: {}", largura, altura);
// }



// fn main() {
//     let tupla_aninhada: ((i32, i32), String) = ((5, 10), String::from("Rust"));

//     // Acessando elementos da tupla aninhada
//     println!("Número: ({}, {})", (tupla_aninhada.0).0, (tupla_aninhada.0).1);
//     println!("String: {}", tupla_aninhada.1);
// }



// fn main() {
//     // Declara um array mutável de 3 inteiros.
//     let mut valores: [i32; 3] = [10, 20, 30];

//     // Modifica o segundo elemento do array.
//     valores[1] = 25;

//     println!("valor do indice 0: {}", valores[0]);
//     println!("quantidade de valores do array: {}", valores.len());

//     for n in valores.iter() {
//         println!("{}", n);
//     }

// }



// fn main() {
//     // Declara um array mutável de 3 inteiros.
//     let mut valores: [[i32; 2]; 3] = [[10, 5], [20, 70], [30, 70]];

//     // Modifica o segundo elemento do array.
//     valores[1][0] = 25;

//     println!("valor do indice 0: {}", valores[0][0]);
//     println!("quantidade de valores do matriz: {}", valores.len());
//     println!("quantidade de valores do array da matriz: {}", valores[0].len());

//     for n in valores.iter() {
//         for m in n.iter() {
//             println!("{}", m);
//         }
//         println!("---------");
//     }
// }



// fn main(){
//     // Cria um vetor vazio de inteiros e adiciona elementos a ele.
//     let mut vetor: Vec<i32> = Vec::new();
//     vetor.push(10);
//     vetor.push(20);
//     vetor.push(20);
//     vetor.push(20);
//     vetor.push(21);

//     println!("quantidade de valores do array: {}", vetor.len());

//     for n in vetor.iter() {
//         println!("{}", n);
//     }

//     //forma 1
//     let valor: Option<i32> = vetor.pop();
//     if let Some(numero) = valor{
//         println!("O valor de pop: {}", numero);
//     }

//     //forma 2
//     // let numero = vetor.pop().unwrap();
//     // println!("O valor de pop: {}", numero);

//     vetor.pop();
//     vetor.pop();
//     vetor.pop();

//     println!("quantidade de valores do array: {}", vetor.len());

//     for n in vetor.iter() {
//         println!("{}", n);
//     }

// }




// fn main(){
//     // Cria um vetor vazio de inteiros e adiciona elementos a ele.
//     let mut vetor: Vec<i32> = vec![1, 2, 3, 4, 5];
//     vetor.push(10);
//     vetor.push(20);
//     vetor.push(21);

//     println!("quantidade de valores do array: {}", vetor.len());

//     for n in vetor.iter() {
//         println!("{}", n);
//     }

//     //forma 2
//     let numero = vetor.pop().unwrap();
//     println!("O valor de pop: {}", numero);

//     println!("quantidade de valores do array: {}", vetor.len());

//     for n in vetor.iter() {
//         println!("{}", n);
//     }
// }



fn main(){
    // Cria um vetor vazio de inteiros e adiciona elementos a ele.
    let mut vetor: Vec<Vec<i32>> = vec![vec![1, 2], vec![3, 4, 5]];
    vetor[0].push(10);

    vetor.push(vec![5,6,7,8]);

    for l in vetor.iter() {
        println!("----------------");
        for c in l.iter() {
            println!("{}", c);
        }
    }
}