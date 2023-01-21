
database_url := "postgres://consub_rw:localpwd@localhost/consub"

db: 
    docker compose up -d postgres

docker:
    cargo sqlx prepare --database-url {{database_url}} --merged
    docker build -t consub .