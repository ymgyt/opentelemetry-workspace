# Opentelemetry workspace

## TODO

- [ ] otel-layer
- [ ] collector-contirb submodule
- [ ] collector
- [ ] collector logging
- [ ] signoz
- [ ] python
- [ ] context propagation
- [ ] X-Ray id generator
- [ ] goose
- [ ] Sampling
- [ ] Overview image

## Usage

```sh
# Run graphql server
cargo make graphql:run

# Run opentelemetry-collector-contrib
cargo make collector:run
```


## Tasks

### `cargo make graphql:schema`

Print Graphql introspected schema.  
Use `--quiet` cargo make option to redirect outout.

### `cargo make graphql:schema:update`

Update graphql schema.


### `cargo make graphql:generate`

Generate rust client code from schema and query.


### `cargo make request:scenario`

Run loadtest scenario once.  use `SCENARIO` environment variable to specify target scenario.  

`cargo make --env SCENARIO=Hello request:scenario`