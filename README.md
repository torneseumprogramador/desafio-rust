# Aqui terá todos os exemplos de código do desafio de Rust
### Link do desafio abaixo:
- https://www.torneseumprogramador.com.br/cursos/desafio_rust/aulas

# Prompt GPT 27/03/2024

Tipo de dados
- https://chat.openai.com/share/a8784f57-5faf-42af-826f-415ec202db66

Tipos de memória (Static, Stack, Heap)
- https://drive.google.com/file/d/1vT08-xtgAYM-9rZABqQojI-9QPv2E4w9/view?usp=sharing


# Aula 29/03/2024
### Condicionais em Rust

Em Rust, as estruturas condicionais são usadas para executar diferentes partes do código baseado em determinadas condições.

## if

A expressão `if` é a mais simples estrutura condicional, que avalia uma condição e executa um bloco de código se a condição for verdadeira.

```rust
if condicao {
    // código a ser executado se `condicao` for verdadeira
}
```

## else e else if

Se a condição do `if` não for verdadeira, você pode usar `else` para especificar um bloco de código a ser executado. O `else if` permite testar condições adicionais.

```rust
if condicao {
    // código se `condicao` for verdadeira
} else if outra_condicao {
    // código se `outra_condicao` for verdadeira
} else {
    // código se nenhuma das condições anteriores for verdadeira
}
```

