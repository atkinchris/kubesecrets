# kubesecrets

Tool to manage secrets in Kubernetes with kubectl

- [kubesecrets](#kubesecrets)
- [Usage](#usage)
- [Commands](#commands)

## Installation

```sh-session
brew install atkinchris/tools/kubesecrets
```

The tool depends on `kubectl`, which will be installed as a dependency by `brew`.

## Usage

```sh-session
$ kubesecrets [SUBCOMMAND]

$ kubesecrets (-V|--version)
kubesecrets <VERSION>

$ kubesecrets (-h|--help) [SUBCOMMAND]
USAGE
  $ kubesecrets SUBCOMMAND
...
```

## Commands

- [`kubesecrets pull`](#kubesecrets-pull)
- [`kubesecrets push <input>`](#kubesecrets-push-input)

### `kubesecrets pull`

Pull secrets from kubernetes.

```sh-session
USAGE:
    kubesecrets pull [FLAGS] [OPTIONS]

FLAGS:
    -a, --all        get all secrets
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -o, --output <FILE>    output to file
```

### `kubesecrets push <input>`

Push secrets to kubernetes.

```sh-session
USAGE:
    kubesecrets push [FLAGS] <input>

FLAGS:
    -p, --purge      purge managed secrets on kubernetes that are not in the input
    -h, --help       Prints help information
    -V, --version    Prints version information

ARGS:
    <input>    input file containing secrets
```
