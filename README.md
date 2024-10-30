
# <p align="center"><img src="https://github.com/user-attachments/assets/896f9239-134a-4428-9bb5-50ea59cdb5c3" alt="lumen" /></p>


lumen is a free CLI tool that uses AI to summarise git commits.

![lumen-demo](https://github.com/user-attachments/assets/2e84e4aa-a86f-47e6-b939-1e34035dbb02)

### Supported providers
| Provider                                                                                                             | API Key Required | Models                                                                                      |
|----------------------------------------------------------------------------------------------------------------------|------------------|---------------------------------------------------------------------------------------------|
| [Groq](https://groq.com/) `groq`                                                                                   | Yes (free)      | `llama2-70b-4096`, `mixtral-8x7b-32768` (default: `mixtral-8x7b-32768`)                     |
| [OpenAI](https://platform.openai.com/docs/guides/text-generation/chat-completions-api) `openai`                    | Yes             | `gpt-4o`, `gpt-4o-mini`, `gpt-4`, `gpt-3.5-turbo` (default: `gpt-4o-mini`)                  |
| [Phind](https://www.phind.com/agent) `phind` (Default)                                                             | No              | `Phind-70B`                                                                                |


## Installation
1. Cargo: `cargo install lumen`

## Prerequisites
1. git
2. [fzf](https://github.com/junegunn/fzf) (optional): Required for `lumen list` command
3. [mdcat](https://github.com/swsnr/mdcat) (optional): Required for pretty output formatting

## Usage

```zsh
$ lumen --help

# summarise a commit by giving its SHA-1
# eg: lumen explain HEAD
# eg: lumen explain cc50651f
$ lumen explain <commit-hash>

# fuzzy-search (using fzf) commits, and then `explain`
$ lumen list
```

### Configure AI Provider
Using CLI args: 
```sh
$ lumen --provider="openai" --api-key="<your-api-key>" --model="gpt-4o" explain HEAD
$ lumen --provider="openai" --api-key="<your-api-key>" list
```
Using Environment variables:
```
LUMEN_AI_PROVIDER (default: "phind")
LUMEN_API_KEY (when applicable)
LUMEN_AI_MODEL (when applicable)
```
