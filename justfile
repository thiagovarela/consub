
tag := env_var_or_default('tag', 'latest')

database_url := "postgres://consub_rw:localpwd@localhost/consub"

db: 
    docker compose up -d postgres

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

ts:
    typeshare ./apps/accounts --lang=typescript --output-file=./admin/src/lib/api/types/accounts.ts
    typeshare ./apps/blogs --lang=typescript --output-file=./admin/src/lib/api/types/blogs.ts
    typeshare ./apps/clippings --lang=typescript --output-file=./admin/src/lib/api/types/clippings.ts

prep: ts     
    cargo sqlx prepare --database-url {{database_url}} --merged


test:
    cargo test --workspace
    cd admin && pnpm test

deploy:
    fly launch --image registry.fly.io/consub-api:{{tag}} --force-machines  --region mad