#!/bin/bash

iterations=1
testdata_path="./testdata"

source .env


if [ $1 = "run" ]; then
    anvil -m "$MNEMONIC" -b 5
elif [ $1 = "deploy" ]; then
    cd $testdata_path
    PRIVATE_KEY=$PRIVATE_KEY forge script script/Counter.s.sol:CounterScript --fork-url "$ANVIL_RPC_URL" --broadcast
elif [ $1 = "counter_increment" ]; then
    for i in $(seq 1 $iterations); do
        cast send $CONTRACT_ADDRESS 'increment()' --async --gas-limit 100000 --nonce $(date +%s%N) --rpc-url $ANVIL_RPC_URL --private-key $PRIVATE_KEY
    done
elif [ $1 = "counter_decrement" ]; then
    for i in $(seq 1 $iterations); do
        cast send $CONTRACT_ADDRESS 'decrement()' --async --gas-limit 100000 --nonce $(date +%s%N) --rpc-url $ANVIL_RPC_URL --private-key $PRIVATE_KEY
    done
elif [ $1 =  "rust_core" ]; then
    echo $DATABASE_URL
    RUST_ENV=$RUST_ENV cargo run --bin core $CONTRACT_ADDRESS $TX_HASH $ABI_PATH
else
    echo "Unknown command, please check scripts/anvil.sh script for usage"
    exit 1
fi