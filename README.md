# solana-local-dev-starter


This starter follows the solana dev learning guide here (which has dependency issues)
https://solana.com/docs/programs/rust

generte new keypairs
```aiignore
 solana-keygen new -o ./target/deploy/solana_local_dev_starter-keypair.json --force
```

build
```aiignore
cargo build-sbf
```

test
```aiignore
cargo test -- --nocapture
```

set the solana config to point to local validator
```aiignore
solana config set -ul
```


start the local validator
```aiignore
solana-local-validator
```


deploy
```aiignore
solana program deploy target/deploy/solana-local-dev-starter.so
```

run program
```aiignore
cargo run --example client
```