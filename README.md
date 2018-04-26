# Wraith

A web service written in Rust using the rocket-rs (web) and diesel-rs (database) libraries.

## Background

I wanted to use Rust to setup a web service that is able to accept a JSON document and then display all that content.

I started with the Building a [RESTful CRUD API with Rust](https://medium.com/sean3z/building-a-restful-crud-api-with-rust-1867308352d8) article to get started but then modified it to use SQLlite. This required looking at the diesel-rs [sqllite examples](https://github.com/diesel-rs/diesel/tree/master/examples/sqlite/getting_started_step_3).

## Setup

```
$ rustup default nightly
$ rustup update && cargo update
$ diesel setup
$ diesel migration run
$ cargo build
$ cargo run
```

Then view the site [http://localhost:8000/scans](http://localhost:8000/scans).