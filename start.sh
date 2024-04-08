[ -f "env.sh" ] && source env.sh
sh migrate.sh
cargo run