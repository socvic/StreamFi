# StreamFi

**Tokenized Future Income Protocol**

StreamFi enables tokenization of verified future income streams, creating liquid assets from regular payments. Built on Stellar Soroban.

## Overview

StreamFi allows users to:
- Tokenize future income streams (salary, freelance, rental income)
- Create tradeable tokens backed by verified payments
- Stream income continuously to recipients
- Build reputation through reliable payment history
- Trade income positions on the marketplace

## Architecture

- **Income Token Contract**: Mint/burn tokens representing future income claims
- **Streaming Contract**: Continuous payment distribution from realized income
- **Reputation Contract**: Track payment reliability and history
- **Marketplace Contract**: P2P trading of income token positions

## Quick Start

### Smart Contracts

```bash
cd contracts
cargo build
cargo test
```

### Frontend

```bash
cd frontend
npm install
npm run dev
```

## Project Structure

```
StreamFi/
├── contracts/
│   └── contracts/
│       ├── income_token/
│       ├── streaming/
│       ├── reputation/
│       └── marketplace/
├── frontend/
│   └── src/
│       ├── app/
│       ├── components/
│       ├── hooks/
│       └── lib/
├── backend/
└── docs/
```

## License

MIT
