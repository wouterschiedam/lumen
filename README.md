# lumen
[WIP] Free and open-source CLI tool to summarise git commits using AI

Supports OpenAI, Phind, and Groq

## Installation

### Cargo

```bash
cargo install lumen
```

## Usage

```sh
$ lumen --help
$ lumen explain <commit-sha> # eg: lumen explain HEAD
$ lumen list # requires: fzf
```
Default model: `phind`

To use a different model
```sh
$ lumen --provider="openai" --api-key="<your-api-key>" explain HEAD
$ lumen --provider="openai" --api-key="<your-api-key>" list
```
You can also set `LUMEN_AI_PROVIDER` and `LUMEN_API_KEY` env variables and use `lumen` commands without specifying `provider`
