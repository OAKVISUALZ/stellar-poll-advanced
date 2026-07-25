# StellarPoll - Real-time Voting on Stellar Testnet

**Stellar Journey to Mastery — Level 3 Orange Belt Submission**

A real-time voting dApp built on Stellar Soroban smart contracts that allows users to deploy a Live Poll contract, create polls, vote, and see results update in real-time.

![CI](https://github.com/YOUR_USERNAME/stellar-poll/actions/workflows/ci.yml/badge.svg)

## Live Demo

🔗 **https://stellar-poll.netlify.app/**

## What We Built

### Core Features

- **Multi-Wallet Integration** — Connect via Freighter and other Stellar wallets using Stellar Wallets Kit
- **Smart Contract Deployment** — Deploy a Soroban Live Poll contract directly from the frontend to testnet
- **Contract Interaction** — Create polls, vote, close polls, and query results from the frontend
- **Real-time State Sync** — Poll results auto-refresh every 15 seconds with manual refresh
- **Transaction Status Tracking** — Live pending/success/error states with Stellar Expert links
- **Error Handling** — Handles wallet not found, transaction rejected, and insufficient balance errors
- **Responsive Design** — Clean dark-themed UI that works on desktop and mobile

### Level 3 Features

- **Inter-contract Communication** — VotingBadge contract awards non-transferable NFT badges to voters, demonstrating cross-contract calls
- **12 Contract Unit Tests** — Comprehensive Rust tests covering all contract functions, edge cases, and events
- **7 Frontend Unit Tests** — Vitest tests for utility functions and component logic
- **CI/CD Pipeline** — GitHub Actions workflow runs contract tests, frontend tests, lint, and build on every push/PR
- **Soroban Event Streaming** — Contract emits structured events for CREATE, VOTE, and CLOSE operations

## Architecture

```
┌─────────────────────────────────────────────────────────┐
│                    StellarPoll Frontend                  │
│            React 19 + TypeScript + Vite                 │
├─────────────┬───────────────┬───────────────────────────┤
│  Wallet     │  Contract     │  Event                    │
│  Manager    │  Client       │  Streamer                 │
├─────────────┴───────────────┴───────────────────────────┤
│            Stellar SDK (soroban-rpc + Horizon)           │
├─────────────────────────────────────────────────────────┤
│                  Stellar Testnet                         │
├──────────────────────┬──────────────────────────────────┤
│   LivePoll Contract  │   VotingBadge Contract           │
│   (polls + voting)   │   (inter-contract badges)        │
└──────────────────────┴──────────────────────────────────┘
```

### Wallet Options Available
![Wallet Connection]
<img width="1366" height="597" alt="image" src="https://github.com/user-attachments/assets/ee133b8a-f2e0-4b76-aab5-ace44c5791e0" />
<img width="1366" height="581" alt="image" src="https://github.com/user-attachments/assets/beb7af1b-c403-456a-a338-4dd0792bab43" />
<img width="1366" height="636" alt="image" src="https://github.com/user-attachments/assets/c012013a-e92c-4547-9b11-a9d6de58257f" />
<img width="1366" height="688" alt="image" src="https://github.com/user-attachments/assets/e6da8f4d-5082-4b2e-b4d7-a7854445ef75" />
<img width="1366" height="606" alt="image" src="https://github.com/user-attachments/assets/d265c55a-df1c-4e2c-b12d-00949b987d74" />




### Poll Creation
![Poll Creator](screenshots/poll-creator.png)

### Voting & Results
<img width="1366" height="686" alt="image" src="https://github.com/user-attachments/assets/8244ec03-56eb-4e2a-93a7-371cab701eaf" />

## Smart Contracts

### LivePoll Contract (`contract/src/lib.rs`)

The primary contract managing poll lifecycle on Stellar Testnet.

| Function | Description |
|---|---|
| `initialize()` | Initialize the contract |
| `create_poll(creator, question, options)` | Create a new poll with 2-10 options |
| `vote(voter, poll_id, option_index)` | Cast a vote (one vote per wallet per poll) |
| `get_poll(poll_id)` | Get full poll data |
| `get_results(poll_id)` | Get vote counts |
| `has_voted(poll_id, voter)` | Check if a wallet has voted |
| `get_poll_count()` | Get total number of polls |
| `close_poll(caller, poll_id)` | Close a poll (creator only) |

**Events emitted:**
- `CREATE` — New poll created with question and options
- `VOTE` — Vote cast with voter address, poll ID, and option index
- `CLOSE` — Poll closed by its creator

### VotingBadge Contract (`voting-badge-contract/src/lib.rs`)

Inter-contract communication demo: awards non-transferable voting badges to users who participate in polls.

| Function | Description |
|---|---|
| `initialize(admin)` | Set contract admin |
| `award_badge(admin, user, poll_id)` | Award a voting badge to a user |
| `has_badge(user)` | Check if user holds a badge |
| `get_holder_count()` | Get total badge holders |

## Tests

### Contract Tests (12 passing)

```bash
cd contract && cargo test
```

Tests cover: initialization, double-init panic, poll creation, vote casting, duplicate vote prevention, poll closing, unauthorized close rejection, voting on closed polls, multiple voters, poll counting, and event emission.

### VotingBadge Tests (6 passing)

```bash
cd voting-badge-contract && cargo test
```

Tests cover: initialization, badge awarding, holder counting, badge checking, multi-user scenarios, and event emission.

### Frontend Tests (7 passing)

```bash
npm test
```

Tests cover: address shortening, custom character count, Stellar address validation, invalid address rejection, vote percentage calculations, and zero-vote edge cases.

## Tech Stack

- **React 19** + **TypeScript** + **Vite** — Frontend framework
- **Tailwind CSS v4** — Styling
- **Vitest** + **@testing-library/jest-dom** — Frontend testing
- **@creit.tech/stellar-wallets-kit** — Multi-wallet support
- **@stellar/stellar-sdk** v16 — Stellar blockchain and Soroban interaction
- **Soroban SDK** v22 — Smart contract framework (Rust/WASM)
- **GitHub Actions** — CI/CD pipeline

## Prerequisites

- [Node.js](https://nodejs.org/) v18+
- [Rust](https://rustup.rs/) with `wasm32-unknown-unknown` target (for contract builds)
- [Freighter Browser Extension](https://freighter.app/) installed and set to **Testnet** mode

## Setup Instructions

1. **Clone the repository**
   ```bash
   git clone https://github.com/YOUR_USERNAME/stellar-poll.git
   cd stellar-poll
   ```

2. **Install dependencies**
   ```bash
   npm install
   ```

3. **Build the smart contract** (pre-built WASM is included in `public/`)
   ```bash
   cd contract
   cargo build --release --target wasm32-unknown-unknown
   cp target/wasm32-unknown-unknown/release/live_poll.wasm ../public/live_poll.wasm
   cd ..
   ```

4. **Run tests**
   ```bash
   cd contract && cargo test && cd ..
   cd voting-badge-contract && cargo test && cd ..
   npm test
   ```

5. **Start the development server**
   ```bash
   npm run dev
   ```

6. **Get testnet XLM**
   Use the [Stellar Testnet Faucet](https://friendbot.stellar.org/) to fund your wallet:
   ```bash
   curl "https://friendbot.stellar.org?addr=YOUR_PUBLIC_KEY"
   ```

## How to Use

1. Click **Connect Wallet** and select your wallet (Freighter recommended)
2. Click **Deploy Contract** to deploy the Live Poll contract to testnet
3. **Create a Poll** with a question and 2-10 options
4. **Vote** on any active poll by clicking an option
5. Watch results update in **real-time** (auto-refreshes every 15s)
6. View all transactions on **Stellar Expert**

## Error Types Handled

1. **Wallet Not Found** — When no Stellar wallet extension is installed
2. **Transaction Rejected** — When the user rejects a transaction in their wallet
3. **Insufficient Balance** — When the user doesn't have enough XLM for fees

## Project Structure

```
src/
  lib/
    stellar.ts              # Wallet connection and balance operations
    contract.ts             # Soroban contract interaction (deploy, call, query)
  components/
    WalletConnection.tsx    # Wallet connect/disconnect UI
    BalanceDisplay.tsx      # XLM and asset balance display
    ContractSetup.tsx       # Contract deployment UI
    PollCreator.tsx         # Create new poll form
    PollList.tsx            # List of polls with auto-refresh
    PollCard.tsx            # Individual poll with voting and results
    TransactionStatus.tsx   # Transaction pending/success/error display
  test/
    contract.test.ts        # Frontend unit tests (7 tests)
    setup.ts                # Vitest test setup
  App.tsx                   # Main application component
  main.tsx                  # App entry point
  index.css                 # Tailwind CSS + custom styles
contract/
  src/
    lib.rs                  # LivePoll Soroban smart contract (Rust)
  Cargo.toml                # Contract dependencies (soroban-sdk v22)
  Cargo.lock                # Locked deps (ed25519-dalek v2.2.0)
voting-badge-contract/
  src/
    lib.rs                  # VotingBadge inter-contract demo (Rust)
  Cargo.toml                # Badge contract dependencies
.github/
  workflows/
    ci.yml                  # GitHub Actions CI/CD pipeline
```

## License

MIT
