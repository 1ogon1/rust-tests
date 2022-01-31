# Rust tests

## Getting started 

For creating the new account for deploying contract use next command
```
near create-account subAccount.yourAccount.testnet --masterAccount yourAccount.testnet --initialBalance 10
```

Create constants 
```
export CLIENT_ID=yourAccount.testnet
export CLIENT_PRIVATE_KEY=...  // private key from .near-credentials for yourAccount.testnet
export CONTRACT_NAME=subAccount.yourAccount.testnet
```

First of all - you will need to compile the wasm file of contracts
```
./build.sh
```

And then deploy it like that
```
near deploy $CONTRACT_NAME --wasmFile=res/robot_simulator.wasm
```

Then initialize contract with command
```
near call $CONTRACT_NAME new '{"x": "0", "y": "0", "direction": "North"}' --accountId $CONTRACT_NAME
```

## Test commands
1. Run all tests
```
cargo test
```
2. Run with ignored tests
```
cargo test -- --include-ignored
```
3. Run only ignored tests
```
cargo test -- --ignored
```
4. Show output from println macros
```
cargo test -- --show-output
```
5. Filter test by name. We can specify part of the name
```
cargo test robot
```
6. Set test threads
```
cargo test -- --test-threads 1
```
## Call functions
```
near call $CONTRACT_NAME turn_right '{}' --accountId $CLIENT_ID
```
```
near call $CONTRACT_NAME turn_left '{}' --accountId $CLIENT_ID
```
```
near call $CONTRACT_NAME advance '{}' --accountId $CLIENT_ID
```
Set default position (0, 0) and direction North
```
near call $CONTRACT_NAME reset '{}' --accountId $CLIENT_ID
```
Available instructions:
- R - turn right
- L - turn left
- A - advance
```
near call $CONTRACT_NAME instructions '{"instructions": "LAAR"}' --accountId $CLIENT_ID
```
## View functions
```
near view $CONTRACT_NAME position
```
```
near view $CONTRACT_NAME direction
```
```
near view $CONTRACT_NAME last_move
```
```
near view $CONTRACT_NAME get_owner
```