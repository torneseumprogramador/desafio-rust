/// Esta é uma função que adiciona dois números. <br>
/// Criando uma outra linha de exemplo. <br>
/// Colocando tagas <b>HTML</b> para <u>mostrar</u> que é possível. <br>
///
/// # Argumentos
///
/// * `a` - O primeiro número.
/// * `b` - O segundo número.
///
/// # Retorno
///
/// O resultado da adição de `a` e `b`.
///
/// # Como utilizar ?
///
/// ```
/// let r = adicionar(1,1);
/// println!("{}", r);
///  ```
pub fn adicionar(a: i32, b: i32) -> i32 {
    a + b
}

fn main(){
    let r = adicionar(1,1);
    println!("{}", r);
}