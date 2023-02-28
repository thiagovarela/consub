# Consub 

Consub is an approach to a lightweight Headless CMS written in Rust.

The goal is to support a multi-tenant modern blog(?) engine.

**Experimental / pet project phase**

## Internals

### Basics

It is built with Rust and PostgreSQL. Each "app" consists of a feature, with its own scheme. This is somewhat a virtual segregation. It is allowed to leak the `accounts` constraints to other apps.

- `accounts`: provides the basics for multi tenants, user authentication (and eventually authorization)
- `blogs`: basic blogging with enough to get started
- `leads`: the starting point of a contact form

A crate `api` glues it all together into a deployable unit.

Given that there is only one database, migrations are shared in a single folder `migrations`.

### Running

It depends on `cargo` and locally it spins up postgres using `docker compose`. 

Also, you can install `just` (`cargo install just`), and use it to `just run` locally.

### Tests

Most of what I'm testing are based on API request and responses.
