# Example

Replace the contract address by the one you get from `counter:deploy`

```bash
pnpm anvil
pnpm counter:deploy
CONTRACT_ADDRESS=0x9fe46736679d2d9a65f0992f2272de9f3c7fa6e0 pnpm counter:increment
CONTRACT_ADDRESS=0x9fe46736679d2d9a65f0992f2272de9f3c7fa6e0 pnpm counter:decrement
```
