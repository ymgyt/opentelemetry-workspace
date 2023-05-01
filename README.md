# Opentelemetry workspace

## TODO

- [ ] python
- [ ] context propagation
- [ ] X-Ray id generator
- [ ] Sampling
- [ ] Overview image

## Usage

```sh
# Run jaeger
cargo make jaeger:run

# Run opentelemetry-collector-contrib
cargo make collector:run

# Run graphql server
cargo make graphql:run
```

Jaeger UI: `localhost:16686`

## Tasks

### `cargo make graphql:schema`

Print Graphql introspected schema.  
Use `--quiet` cargo make option to redirect outout.

### `cargo make graphql:schema:update`

Update graphql schema.


### `cargo make graphql:generate`

Generate rust client code from schema and query.


### `cargo make request:{graphql,rest}`

Run loadtest scenario once.  use `SCENARIO` environment variable to specify target scenario.  

`cargo make --env SCENARIO=hello request:graphql`  
`cargo make --env=SCENARIO=foo request:rest`
