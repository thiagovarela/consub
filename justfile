
tag := env_var_or_default('tag', 'latest')
api_name := env_var_or_default('api_name', 'api')

database_url := "postgres://consub_rw:localpwd@localhost/consub"

db: 
    docker compose up -d timescale --remove-orphans

run: db    
   cargo run -p api

dev: db
    cargo watch -x run -p api 

docs:    
   cargo run -p cli

docker: prep  
    docker build . --tag api

release:
    cargo build -p api --release    

reset:
    sqlx database reset --database-url {{database_url}}

prep: 
    cargo sqlx prepare --database-url {{database_url}} --merged

test:
    cargo test --workspace

deploy:
    fly launch --image registry.fly.io/{{api_name}}:{{tag}} --force-machines  --region mad

check-db:
    sqlx migrate info --database_url=postgres://consub_rw:localpwd@localhost/consub    