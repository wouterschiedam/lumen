# lumen
[WIP] Free and open-source CLI tool to summarise git commits using AI

Supports OpenAI, Phind, and Groq

![lumen-demo](https://github.com/user-attachments/assets/2e84e4aa-a86f-47e6-b939-1e34035dbb02)

## Installation
1. Cargo: `cargo install lumen`

## Prerequisites
1. git
2. [fzf](https://github.com/junegunn/fzf) (optional): Required for `lumen list` command
3. [mdcat](https://github.com/swsnr/mdcat) (optional): Required for pretty output formatting

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
