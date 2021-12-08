# Sanctuary Sneak-peek

### This repo is meant to provide evidence that sanctuary is being actively worked on. Some parts of the code have been redacted to prevent anyone from copying them (they are the most important and hardest to implement), as such, the code found in this repository does not reflect the current or final state of the sanctuary codebase.

#### Dependencies

- Node.js minimum v16.13.1
- Rust
- Anchor CLI
- Yarn

#### First steps

In Anchor.toml, change the `wallet` path to `<path_to_your_solana_wallet>`.

Install all node dependencies with `yarn`.

#### Building the staking program

Run `anchor build` to build the staking program.

#### Testing the staking program

Run `anchor test` to run the tests defined in `tests/` on the staking program.