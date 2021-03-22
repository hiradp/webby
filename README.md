# webby [![Docker Image CI](https://github.com/hiradp/webby/actions/workflows/main.yml/badge.svg)](https://github.com/hiradp/webby/actions/workflows/main.yml)
:spider_web: Spin up a web presence quickly

## Development
This is a cargo project and works similar to most others.

```shell
# Check for error
cargo check

# Build binaries
cargo build
cargo build --release

# Run the code for development
cargo run

# Test - what tests?
cargo test 

# Lint and format (successful lint is required for all PRs).
cargo fmt
cargo clippy
```

### Helm Charts
To install a chart locally run from the root of the repo:

```shell
 helm install <name> charts/webby
```

Use the following to list installed helm charts:

```shell
helm list
```

And to uninstall:

```shell
helm uninstall <name>
```

To set up port-forwarding to the service run:

```shell
kubectl port-forward svc/<name> <port>:8000
```

then you can go to http://localhost:<port>

## Why? 
You may be asking yourself, why go through building a dockerized web application with rust and rollout Helm Charts. It's a very valid question. If the end goal was to just publish a portfolio, the most efficient way is definitely not this. But as with everything else in life, the point here is learning. So consider this, why not?
