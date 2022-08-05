## Banco de Dados

```sh
# Iniciando o DB
docker run --rm -p 5432:5432 -e "POSTGRES_PASSWORD=postgres" --name pg postgres:14

# Optional psql (outro terminal )
docker exec -it -u postgres pg psql
```

## Dev Test

```sh
cargo watch -q -c -w src/ -x 'test model_db_ -- --test-threads=1 --nocapture'
```