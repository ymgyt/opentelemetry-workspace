# Opentelemetry workspace

Repository for play with Opentelemetry SDKs for each language

![Project overview](./project_overview.png)

## Setup

```sh
git clone --recursive https://github.com/ymgyt/opentelemetry-workspace.git
```

We use 
 * Rust(graphql-server, graphql-client)
   * [cargo-make](https://github.com/sagiegurari/cargo-make) as a task runner 
 * Python(rest)
   * [poetry](https://python-poetry.org/docs/) as python dependency management
 * Go(opentelemetry-collector)
 * Node(ui)


```sh
# Install cargo-make
cargo install cargo-make

# Install poetry
curl -sSL https://install.python-poetry.org | python3 -

cargo make project:init
```

## Usage

```sh
# Run openobserve
cargo make openobserve:run

# Run jaeger
cargo make jaeger:run

# Run opentelemetry-collector-contrib
cargo make collector:run

# Run rest server
cargo make rest:run

# Run graphql server
cargo make graphql:run

# Run ui
cargo make ui:run
```

Openobserve UI: `localhost:5080` (root@ymgyt.io/openobserve)
Jaeger UI: `localhost:16686`  
RabbitMQ UI: `localhost:15672`  (guest/guest)   
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
