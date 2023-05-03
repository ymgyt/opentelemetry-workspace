# Opentelemetry workspace

## Setup

```sh
git clone --recursive https://github.com/ymgyt/opentelemetry-workspace.git
```

We use 
 * [cargo-make](https://github.com/sagiegurari/cargo-make) as a task runner 
 * [poetry](https://python-poetry.org/docs/) as python dependency management

```sh
# Install cargo-make
cargo install cargo-make

# Install poetry
curl -sSL https://install.python-poetry.org | python3 -
```


## TODO

- [ ] Init task
  - submodule update
  - ui
- [ ] Overview image

## Usage

```sh
# Run jaeger
cargo make jaeger:run

# Run opentelemetry-collector-contrib
cargo make collector:run

# Run graphql server
cargo make graphql:run

# Run ui
cargo make ui:run
```

Jaeger UI: `localhost:16686`
UI: `localhost:5173`

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
`cargo make --env SCENARIO=foo request:rest`
