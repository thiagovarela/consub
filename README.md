# Consub 

Consub is an approach to a lightweight Headless CMS written in Rust.

The goal is to support a multi-tenant modern blog(?) engine.

**Experimental / pet project phase**

## Internals

### Basics

It is built with Rust and ~~PostgreSQL~~ TimescaleDB. Each "app" consists of a feature, with its own scheme. This is somewhat a virtual segregation. It is allowed to leak the `accounts` constraints to other apps.

- `accounts`: provides the basics for multi tenants, user authentication (and eventually authorization).
- `blogs`: basic blogging with enough to get started.
- `media`: a media library manager (currenly images only get uploaded to a predefined CDN).
- `clippings`: linking to full or partial media news.
- `analytics`: a set of timescaledb tables to allow page views and page events.
- `leads`: the starting point of a contact form.

A crate `api` glues it all together into a deployable unit.

Given that there is only one database, migrations are shared in a single folder `migrations`.

### Roadmap

- [ ] ~~Media Uploader sucks, I need to define a set of recommended image sizes to be uploaded in the first place, then resize accordinly and generate the "image set", allow alt and/or caption to be included, define a decent webp compression.~~ Added a resize progress and upload progress bar, very ugly but ok. Postponing other changes (alt, caption, delete image)
- [ ] Get the analytics app set up
- [ ] Get the Content Editor into a usable state (fix the bubble menu, maybe add the floating), add localstorage save?
- [ ] Revisit the test suite since I'm happy with current state of public/admin apis.
- [ ] Cargo docs... not sure..
- [ ] Workers app -- postgres skip locked job queue to initially send emails...
- [ ] Leads app -- should be quick to set up a couple of endpoints.
- [ ] Account password change/recovery, invitation
- [ ] Account profile management so it can be properly used by Blogs
- [ ] Set up an API metrics exporter to be used by fly.io managed prometheus/grafana
- [ ] API rate limiting/cors
- [ ] Remove Opendal if I'm just sticking with S3 to upload files.
- [ ] Pages app, depends on how the content editor supports more flexible types of content/layouts.

For now, search is staying as postgres queries. 

### Running

It depends on `cargo` and locally it spins up postgres using `docker compose`. 

Also, you can install `just` (`cargo install just`), and use it to `just run` locally.

### Tests

Most of what I'm testing are based on API request and responses.

### Decisions

(To my future self)

API is powered by *lost count of rust libraries*, notably axum, tokio, sqlx, serde. This is growing a lot, but I'm not focused on carefully thinking about this.

The admin is SvelteKit with SSR. All calls to the backend are done in a server environment, so a domain/path scoped cookie is carried and then forwarded to the API. The idea is that it can run on the *edge*

All SQL queries are compile-time checked with prepared statements, so there is a bit of learning curve here in comparison to a regular ORM, specially when trying to fit optional inputs and custom types.

The API docs are generated through OpenAPI specs (thanks aide!) and a typescript client can be generated. ~~At the moment I'm not splitting 'public facing' apis (allowed unauthenticated access) from the restricted ones from the spec.~~. There is a public facing API, where clients can call using an api key header. The other is the admin API that is authenticated using bearer tokens. 

I added TimescaleDB to allow *page events* / *analytics*, like simple, privacy aware stats. Maybe this is stupid, but :shrug:  

~~For now I'm using a "general" authorization_layer to protect routes, but this is likely going to change in favor of explicit scopes in the route signature and avoid middlewares for this purpose. Axum extractors are quite handy.~~
Each protected route is responsible for doing so via axum extractors, this avoids a concept of generic middleware in favour of explicitness.

Each account can issue several account keys, a keypair is then generated and stored in the DB (which I know is stupid, but leave it there for simplicity, maybe there are other ways of ensuring safeness that I don't know of). Then a user can request an access token and specify the account key to be used, the JWT is then signed and will only work if that key keeps valid. 

An app is a rust library added to the `apps` folder, theoretically it exposes a axum::Router that the `api` create nests/merges to the main router for exposure.

#### Fly.io

At the moment I'm deploying this to fly.io, so you may find some references to fly. I'll likely share the set up in a different medium than the repo. 

```mermaid

```

#### Reasoning

Hi, I decided to build this for a few reasons:

1) I wanted to.
2) To get involved into Rust by building something likely simple.