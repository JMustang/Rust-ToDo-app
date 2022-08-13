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