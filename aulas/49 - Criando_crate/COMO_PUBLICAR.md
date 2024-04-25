=== Para criar a crate ===
```shell
cargo new validador_cpf --lib
```

=== Criar conta no https://crates.io ===
- https://crates.io/settings/profile

=== Para publicar fazer o login ===
```shell
cargo login
```
=== ou fazer o login passando o token que vc consegue aqui ===
- https://crates.io/settings/tokens

```shell
cargo login YOUR_API_TOKEN
```
=== Para sair ===
```shell
cargo logout
```
=== Para gerar o pacode de publicação ===
```shell
cargo package
```
=== Para publicar fazer o push da crate ===
```shell
cargo publish
```