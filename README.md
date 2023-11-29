# 1inch-rs

1inch-rs is a high-level, type-safe API wrapper for the 1inch API written for Rust.

**Note: Currently the project is under development!**
Because of this, it only covers some of the API, and may also contain some bugs.

## Currently Supported Endpoints

- `/swap/liquidity-sources`
- `/swap/tokens`
- `/swap/quote/`
- `/swap/swap/`
- `/swap/approve/spender`
- `/swap/approve/transaction`
- `/swap/approve/allowance`

## Usage

There are a lot of usage-examples for each request type. See `/examples/`.

You can run any example with this command:
```bash
ONE_INCH_API_TOKEN=YOUR_API_KEY cargo run --example [example_name]
```
Replace [example_name] with the name of the example you want to launch.



License
BSD 3-Clause License
