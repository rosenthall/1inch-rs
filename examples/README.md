# Examples for one-inch

This `examples` directory provides practical samples for using the `one-inch` library to interact with the 1inch API.

## Quick Start

To run an example, use `ONE_INCH_API_TOKEN=YOUR_API_KEY cargo run --example [example_name]` from the root of the cloned repository.

## Examples

- `allowance`: How to check the allowance of a token for specified account.
- `approve`: How to get approvement tranaction to change allowance 
- `info` : Get information about available for swapping tokens and about liquidity protocols 
- `swap`: Different examples of usage swap api + error handling.
- `quote`: Pretty the same as `swap` example, but about `quote` api.
- `prices`: Get information about all supported currencies and check price for specific tokens.