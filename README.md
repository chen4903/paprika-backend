# Paprika Backend

A simple EVM bytecode analysis tool that provides various capabilities through a RESTful API. This is the backend of [Paprika](https://paprika-evm.xyz/).

## Features

- **Bytecode Analysis**
  - Disassemble EVM bytecode
  - Generate Control Flow Graphs (CFG)
  - Compare contract similarities
  - Guess function signatures

- **Smart Contract Interaction**
  - ABI generation from bytecode
  - Function signature lookup
  - Call data generation
  - Transaction simulation

- **EVM Multi-Chain Support**

## Cache Strategy

- In-memory LRU cache with size limit of 100 items
- SQLite persistence for runtime code and signatures
- Auto cleanup of expired data every 30 minutes (configurable)

## Design

See more details in `paper/*`

## Quick Start

1. Make sure we have installed [Foundry](https://getfoundry.sh/)

2. Copy `.env.example` to `.env` and fill out all values

3. Run `make start`

4. You can make calls through the examples in `example/api.md`.

## Support
We are listed in [Giveth](https://giveth.io/project/paprika?apcid=0068ef085cbb8443e47c5800)!
<img width="1332" height="623" alt="image" src="https://github.com/user-attachments/assets/41e2caec-7c03-492d-b295-a0cb1d0f1364" />

