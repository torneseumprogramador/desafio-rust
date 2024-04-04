mod tela;
mod entidades;
mod logica_negocio;
mod menu;

fn main(){
    /*
    === Passo 1: ===

    Sua misão é contruir um menu de sistema console

    O que você deseja fazer ?
    1 - Cadastrar aluno { iniciando cadastro de aluno }
    2 - Alterar aluno  { iniciando alteracao de aluno }
    3 - Excluir aluno { iniciando exclusão de aluno }
    4 - Listar alunos { listando alunos }
    5 - Sair do programa

    === Passo 2: ====

    Agora que vc já sabe criar uma função que vc já sabe como utilizar
    um array ou tupla ou vetor

    faça a implementação da opção 1 e da opção 4
    o que colocar no cadastro de aluno
    nome, matricula, notas{vetor(f32)}

    === Passo 3: ====
    Agora que vc já conhece o struct, implemente os passos 2 e 3


    amanha
    - metodos para calcular dados de alunos "média"
    - modulos separados em pastas "Montagem de arquitetura distribuida"
    - persistencia
    */

    menu::carregar();
}