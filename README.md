# embassy-testing
Test application evaluating embassy-rs for use in VL6.

## Note
This repo should be used as a template for future embassy applications, if we decide to move forward with it, because configuration is a royal pain in the ass

## How to Change STM32 Model

- Line 2 in `Embed.toml`
- Line 12 in `Cargo.toml`
- Update `FLASH_SIZE_KIB` and `RAM_SIZE_KIB` in `size.py`
- Add svd file for the model to `.vscode/`

## Start

```bash
cargo embed
```
