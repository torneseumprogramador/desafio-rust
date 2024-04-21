# Comandos principais
```shell
# Configurar sqlx-cli
cargo install sqlx-cli --no-default-features --features native-tls,mysql

# Criar migration
sqlx migrate add create_students_table

# Aplicar migration
sqlx migrate run

# Voltar migration
sqlx migrate revert
```