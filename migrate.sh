[ -f "env.sh" ] && source env.sh

SQL_COMMAND="CREATE DATABASE IF NOT EXISTS $DATABASE_DB; USE $DATABASE_DB; CREATE TABLE IF NOT EXISTS __migrations_files__ (file_executed VARCHAR(1000));"
mysql -u"$DATABASE_USER" -p"$DATABASE_PASSWORD" -e "$SQL_COMMAND"

for file in migrations/*.sql; do
    SQL_COMMAND_CHECK="USE $DATABASE_DB; SELECT file_executed FROM __migrations_files__ WHERE file_executed = '$file';"
    result=$(mysql -u"$DATABASE_USER" -p"$DATABASE_PASSWORD" -s -N -e "$SQL_COMMAND_CHECK")

    if [[ -z "$result" ]]; then
        mysql -u"$DATABASE_USER" -p"$DATABASE_PASSWORD" -e "USE $DATABASE_DB; $(cat "$file")"
        if [[ $? -eq 0 ]]; then
            SQL_COMMAND_INSERT="USE $DATABASE_DB; INSERT INTO __migrations_files__ (file_executed) VALUES ('$file');"
            mysql -u"$DATABASE_USER" -p"$DATABASE_PASSWORD" -e "$SQL_COMMAND_INSERT"
            echo "===[ $file - Migrated with success ]==="
        else
            echo "===[ Erro ao rodar migração: $file ]==="
            exit 1
        fi
    fi
done