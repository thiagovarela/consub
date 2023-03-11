# Consub 

Consub is an approach to a lightweight Headless CMS written in Rust.

The goal is to support a multi-tenant modern blog(?) engine.

**Experimental / pet project phase**

## Internals

### Basics

It is built with Rust and ~~PostgreSQL~~ TimescaleDB. Each "app" consists of a feature, with its own scheme. This is somewhat a virtual segregation. It is allowed to leak the `accounts` constraints to other apps.

- `accounts`: provides the basics for multi tenants, user authentication (and eventually authorization)
- `blogs`: basic blogging with enough to get started
- `clippings`: linking to full or partial media news
- `leads`: the starting point of a contact form

A crate `api` glues it all together into a deployable unit.

Given that there is only one database, migrations are shared in a single folder `migrations`.

### Running

It depends on `cargo` and locally it spins up postgres using `docker compose`. 

Also, you can install `just` (`cargo install just`), and use it to `just run` locally.

### Tests

Most of what I'm testing are based on API request and responses.

### Decisions

(To my future self)

API is powered by *lost count of rust libraries*, notably axum, tokio, sqlx, serde. This is growing a lot, but I'm not focused on carefully thinking about this.

The admin is SvelteKit with SSR. All calls to the backend are done in a server environment, so a domain/path scoped cookie is carried. The idea is that it can run on the *edge*

All SQL queries are compile-time checked with prepared statements, so there is a bit of learning curve here in comparison to a regular ORM, specially when trying to fit optional inputs and custom types.

The API docs are generated through OpenAPI specs (thanks aide.) and a typescript client can be generated. At the moment I'm not splitting 'public facing' apis (allowed unauthenticated access) from the restricted ones from the spec.

I added TimescaleDB to allow *page events* / *analytics*, like simple, privacy aware stats. Maybe this is stupid, but :shrug:  

For now I'm using a "general" authorization_layer to protect routes, but this is likely going to change in favor of explicit scopes in the route signature and avoid middlewares for this purpose. Axum extractors are quite handy.

Each account can issue several account keys, a keypair is then generated and stored in the DB (which I know is stupid, but leave it there for simplicity, maybe there are other ways of ensuring safeness that I don't know of). Then a user can request an access token and specify the account key to be used, the JWT is then signed and will only work if that key keeps valid. 

An app is a rust library added to the `apps` folder, theoretically it exposes a axum::Router that the `api` create nests/merges to the main router for exposure.

#### Fly.io

At the moment I'm deploying this to fly.io, so you may find some references to fly. I'll likely share the set up in a different medium than the repo. 

#### Reasoning

Hi, I decided to build this for a few reasons:

1) I wanted to.
2) To get involved into Rust by building something likely simple.