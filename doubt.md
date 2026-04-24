## Why use the rustls, chrono, uuid did the sqlx not provide this then what exactly i provide
sqlx = { version = "0.8", features = [ "runtime-tokio-rustls", "postgres", "uuid", "decimal", "chrono" ] }


## Why we need Type Aliases
Type Aliases: Using pub type DbPool = PgPool;;

## What is ProtoBuf ???