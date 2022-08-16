[![LOGO](rust-logo.png)](https://www.rust-lang.org)


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

## Tokio library

- Um runtime  para escrever aplicativos confiáveis, assíncronos e finos com a linguagem de programação Rust. De forma que:

1. Rápido: as abstrações de custo zero do Tokio oferecem desempenho bare-metal.

2. Confiável: Tokio aproveita a propriedade, o sistema de tipos e o modelo de simultaneidade do Rust para reduzir bugs e garantir a segurança do thread.

3. Escalável: Tokio tem uma pegada mínima e lida com contrapressão e cancelamento naturalmente.


## futures-rs

- futures-rs é uma biblioteca que fornece as bases para programação assíncrona em Rust. Ele inclui definições de características-chave como Stream, bem como utilitários como join!, select! e vários métodos combinadores de futuros que permitem um fluxo de controle assíncrono expressivo.


## warp

- Uma estrutura de servidor web superfácil e que pode ser composta para velocidades de distorção.

- O bloco de construção fundamental do warp é o Filter: eles podem ser combinados e compostos para expressar requisitos ricos em solicitações.

- Graças ao seu sistema de Filter, a warp fornece estes itens fora da caixa:

1. Roteamento de caminho e extração de parâmetros
2. Requisitos de cabeçalho e extração
3. Desserialização da string de consulta
4. Corpos JSON e de formulário
5. Dados de formulário de várias partes
6. Arquivos estáticos e diretórios
7. Websockets
8. Registro de acesso
9. Compressão Gzip, Deflate e Brotli
- Como ele se baseia no hyper, você obtém automaticamente:

1. HTTP/1
2. HTTP/2
3. Assíncrono
4. Uma das implementações HTTP mais rápidas
5. Testado e correto

Exemplo
- Adicione warp e Tokio às suas dependências:
  
```rs
tokio = { version = "1", features = ["full"] }
warp = "0.3"
```

e eles iniciam no main.rs:

```rs
use warp::Filter;

#[tokio::main]
async fn main() {
    // GET /hello/warp => 200 OK with body "Hello, warp!"
    let hello = warp::path!("hello" / String)
        .map(|name| format!("Hello, {}!", name));

    warp::serve(hello)
        .run(([127, 0, 0, 1], 3030))
        .await;
}
```


## slqx

- SQLx é uma crate Rust† SQL assíncrona e pura com consultas verificadas em tempo de compilação sem um DSL.

1. Verdadeiramente Assíncrono. Construído desde o início usando async/await para máxima simultaneidade.

2. Consultas verificadas em tempo de compilação (se desejar). Veja SQLx não é um ORM.

3. Banco de dados agnóstico. Suporte para PostgreSQL, MySQL, SQLite e MSSQL.

4. Rust Pura. Os drivers Postgres e MySQL/MariaDB são escritos em puro Rust usando zero código inseguro††.

5. Tempo de execução agnóstico. Funciona em diferentes tempos de execução (async-std / tokio / actix) e backends TLS (native-tls, rustls).


## slqb

IMPORTANTE - As versões 0.0.x serão experimentais e provavelmente quebrarão as APIs em cada versão.

sqlb é (será) um SQLBuilder simples e expressivo para Rust.

Simples - Focado em fornecer um esquema expressivo, combinável e razoavelmente tipado para construir e executar (via sqlx por enquanto) instruções SQL parametrizadas. O objetivo NÃO é abstrair o SQL, mas torná-lo expressivo e combinável usando construções programáticas do Rust.
NÃO é um executor/driver de banco de dados (Estará usando SQLX como executor de sql)
NÃO é um ORM, embora eventualmente se possa construir um ORM em cima dele.
Expressivo - De entrada e saída de dados digitados arbitrários (lista de nomes/valores) a regras de estrutura e mapeamento.
Focado
sqlx - O primeiro "executor de banco de dados" fornecido será sqlx.
PostgreSQL - O primeiro suporte de banco de dados será o Postgres (via sqlx). Dependendo do interesse e das solicitações pull, outro suporte de banco de dados pode ser adicionado.
Declaração preparada SOMENTE!
NOTA: Os SQL Builders normalmente não são usados ​​diretamente pela lógica de negócios do aplicativo, mas sim para serem encapsulados em alguma camada de acesso a dados do aplicativo (por exemplo, DAOs ou MACs - Model Access Controller -). Na verdade, mesmo ao usar ORMs, geralmente é um bom design de código envolver esses acessos por meio de algumas camadas de acesso a dados.

Metas para as primeiras versões 0.1.x:

sqlx - Provavelmente será centralizado em SQLX.
PostgreSQL - Provavelmente suportará apenas SQLX. Contribuições para outro banco de dados (via sqlx) são bem-vindas
Macros - para manter a coisa DRY (mas são opcionais, todas podem ser implementadas através de objetos trait)
Exemplo de API inicial (apenas conceitual por enquanto)


```rs
#[derive(sqlx::FromRow)] // Optional: to be able to use the sqlx_exec::fetch_as...
pub struct Todo {
    pub id: i64,
    pub title: String,
}

#[derive(sqlb::Fields)] // implements sqlb::HasFields for dynamic binding
pub struct TodoPatch {
    pub title: Option<String>,
}

let patch_data = TodoPatch {
	title: Some("Hello Title".to_string())
};

// INSERT - Insert a new Todo from a Partial todo
let sb = sqlb::insert().table("todo").data(patch_data.fields());
let sb = sb.returning(&["id", "title"]);
let (_id, title) = sb.fetch_one::<(i64, String), _>(&db_pool).await?;

// SELECT - Get all todos
let sb = sqlb::select().table("todo").columns(&["id", "title"]).order_by("!id");
let todos: Vec<Todo> = sb.fetch_as_all(&db_pool).await?;
assert_eq!(1, todos.len());
 ```

## Para sqlb Dev

- Iniciar um PostgreSQL

```rs
# In terminal 1 - start postges
docker run --rm --name pg -p 5432:5432  -e POSTGRES_PASSWORD=welcome  postgres:14

# In terminal 2 - (optional) launch psql on the Postgres instance above
docker exec -it -u postgres pg psql

# In terminal 3 - MUST run with `--test-threads=1` to avoid database access conflicts
cargo test -- --test-threads=1

# or watch a particular test target
cargo watch -q -c -x 'test --test test_sb_insert -- --test-threads=1'
```