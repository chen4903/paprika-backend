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

