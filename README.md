
# <p align="center"><img src="https://github.com/user-attachments/assets/896f9239-134a-4428-9bb5-50ea59cdb5c3" alt="lumen" /></p>
![Crates.io Total Downloads](https://img.shields.io/crates/d/lumen)
![GitHub License](https://img.shields.io/github/license/jnsahaj/lumen)
![Crates.io Size](https://img.shields.io/crates/size/lumen)




### lumen is a free CLI tool that uses AI to generate Git Commit Summary without requiring an API key.

![demo](https://github.com/user-attachments/assets/0d029bdb-3b11-4b5c-bed6-f5a91d8529f2)

### Supported providers

| Provider                                                                                                             | API Key Required | Models                                                                                      |
|----------------------------------------------------------------------------------------------------------------------|------------------|---------------------------------------------------------------------------------------------|
| [Phind](https://www.phind.com/agent) `phind` (Default)                                                             | No              | `Phind-70B`                                                                                |
| [Groq](https://groq.com/) `groq`                                                                                   | Yes (free)      | `llama2-70b-4096`, `mixtral-8x7b-32768` (default: `mixtral-8x7b-32768`)                     |
| [OpenAI](https://platform.openai.com/docs/guides/text-generation/chat-completions-api) `openai`                    | Yes             | `gpt-4o`, `gpt-4o-mini`, `gpt-4`, `gpt-3.5-turbo` (default: `gpt-4o-mini`)                  |
| [Claude](https://claude.ai/new) `claude`                                                                     | Yes             | [see list](https://docs.anthropic.com/en/docs/about-claude/models#model-names) (default: `claude-3-5-sonnet-20241022`) |                                                                                |



# Installation
### Using [Homebrew](https://brew.sh/) (MacOS and Linux)
```
brew tap jnsahaj/lumen
brew install lumen --formula
```
### Using [Cargo](https://github.com/rust-lang/cargo)

> [!IMPORTANT]
> `cargo` is a package manager for `rust`,
> and is installed automatically when you install `rust`.
> see [installation guide](https://doc.rust-lang.org/cargo/getting-started/installation.html)
```
cargo install lumen
```

# Prerequisites
1. git
2. [fzf](https://github.com/junegunn/fzf) (optional): Required for `lumen list` command
3. [mdcat](https://github.com/swsnr/mdcat) (optional): Required for pretty output formatting

# Usage

```zsh
$ lumen --help

# summarise a commit by giving its SHA-1
# eg: lumen explain HEAD
# eg: lumen explain cc50651f
$ lumen explain <commit-sha>

# fuzzy-search (using fzf) commits, and then `explain`
$ lumen list
```
AI Provider can be configured by using CLI arguments or Environment variables.
```sh
-p, --provider <PROVIDER>  [env: LUMEN_AI_PROVIDER] [default: phind] [possible values: openai, phind, groq]
-k, --api-key <API_KEY>    [env: LUMEN_API_KEY]
-m, --model <MODEL>        [env: LUMEN_AI_MODEL]

# eg: lumen -p="openai" -k="<your-api-key>" -m="gpt-4o" explain HEAD
# eg: lumen -p="openai" -k="<your-api-key>" -m="gpt-4o" list

```
