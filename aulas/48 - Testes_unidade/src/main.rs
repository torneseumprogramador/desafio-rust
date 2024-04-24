mod soma;
mod validador_cpf;
mod validador_cnpj;
mod divide_zero;
mod busca_por_id_struct;

fn main(){
   let resultado = soma::soma(9,10);
   println!("O resultado foi {}", resultado); 
}