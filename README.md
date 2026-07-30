StellarPoll 

Real-Time Decentralized Voting on the Stellar Testnet

Stellar Journey to Mastery — Level 3 (Orange Belt) Submission

StellarPoll is a decentralized voting application powered by Stellar Soroban smart contracts. It enables users to deploy a live polling contract, create polls, cast secure on-chain votes, and view results that update automatically in real time.

Built with React, TypeScript, Soroban, and the Stellar Wallets Kit, the application demonstrates end-to-end decentralized application development—from wallet integration and smart contract deployment to inter-contract communication and automated testing.


---

🌐 Live Demo

Application: https://sparkly-squirrel-aeb32f.netlify.app/

Demo Video: https://www.loom.com/share/aa755a8b332f408a85f79d2671170350

Contract: https://stellar.expert/explorer/testnet/contract/CBYI6EQHVO6DUPOWFWVAYWZMTVNH3433YG45XTDJP2C6HKRK7N2KY

Sample Transaction: https://stellar.expert/explorer/testnet/tx/3b13c663436f8f7c336e8005029b592430b9cf4b455a48fa7853433ec34d0262





---

✨ Features

Core Functionality

🔐 Multi-wallet authentication using Stellar Wallets Kit

🚀 Deploy Soroban smart contracts directly from the frontend

📊 Create and manage polls on-chain

🗳️ One-wallet-one-vote enforcement

📈 Live vote tally updates with automatic refresh

🔄 Manual synchronization of contract state

⏳ Transaction status tracking with direct Stellar Expert links

⚠️ Comprehensive error handling for wallet and blockchain interactions

📱 Responsive interface optimized for desktop and mobile devices



---

Level 3 Requirements

This submission includes all required Level 3 capabilities:

✅ Inter-contract communication

✅ Soroban event emission

✅ Comprehensive Rust unit tests

✅ Frontend unit tests

✅ Continuous Integration with GitHub Actions


Inter-Contract Communication

A dedicated VotingBadge contract awards non-transferable NFT voting badges whenever a user successfully participates in a poll, demonstrating cross-contract interaction within Soroban.

Event Streaming

The LivePoll contract emits structured blockchain events for:

CREATE

VOTE

CLOSE


These events enable external applications and indexers to monitor poll activity in real time.


---

🏗️ Architecture

┌─────────────────────────────────────────────────────────┐
│                  StellarPoll Frontend                   │
│             React 19 + TypeScript + Vite               │
├─────────────┬───────────────┬───────────────────────────┤
│ Wallet Kit  │ Contract API  │ Event Stream             │
├─────────────┴───────────────┴───────────────────────────┤
│        Stellar SDK (Soroban RPC + Horizon)             │
├─────────────────────────────────────────────────────────┤
│                 Stellar Testnet                        │
├──────────────────────┬──────────────────────────────────┤
│   LivePoll Contract  │   VotingBadge Contract          │
│ Poll Management      │ NFT Badge Rewards              │
└──────────────────────┴──────────────────────────────────┘


---

📸 Application Preview

Wallet Connection

<img width="1366" height="597" alt="Wallet" src="https://github.com/user-attachments/assets/ee133b8a-f2e0-4b76-aab5-ace44c5791e0" /><img width="1366" height="581" alt="Wallet" src="https://github.com/user-attachments/assets/beb7af1b-c403-456a-a338-4dd0792bab43" /><img width="1366" height="636" alt="Wallet" src="https://github.com/user-attachments/assets/c012013a-e92c-4547-9b11-a9d6de58257f" /><img width="1366" height="688" alt="Wallet" src="https://github.com/user-attachments/assets/e6da8f4d-5082-4b2e-b4d7-a7854445ef75" /><img width="1366" height="606" alt="Wallet" src="https://github.com/user-attachments/assets/d265c55a-df1c-4e2c-b12d-00949b987d74" />
---

Poll Creation




---

Voting & Live Results

<img width="1366" height="686" alt="Voting" src="https://github.com/user-attachments/assets/8244ec03-56eb-4e2a-93a7-371cab701eaf" />
---

📜 Smart Contracts

LivePoll Contract

Responsible for the complete poll lifecycle.

Function	Purpose

initialize()	Initializes the contract
create_poll()	Creates a new poll
vote()	Records a user's vote
get_poll()	Retrieves poll details
get_results()	Returns vote counts
has_voted()	Checks whether a user has voted
get_poll_count()	Returns total polls created
close_poll()	Closes an active poll


Events

Event	Description

CREATE	Poll created
VOTE	Vote submitted
CLOSE	Poll closed



---

VotingBadge Contract

Demonstrates inter-contract communication by rewarding voters with a non-transferable participation badge.

Function	Purpose

initialize()	Sets contract administrator
award_badge()	Awards a badge to a voter
has_badge()	Checks badge ownership
get_holder_count()	Returns total badge holders



---

🧪 Testing

Smart Contract Tests

LivePoll

cd contract
cargo test

12 passing tests covering:

Initialization

Poll creation

Voting

Duplicate vote prevention

Closing polls

Authorization

Event emission

Edge cases



---

VotingBadge

cd voting-badge-contract
cargo test

6 passing tests covering:

Initialization

Badge issuance

Badge ownership

Holder counting

Multiple users

Event emission



---

Frontend

npm test

7 passing tests covering:

Address utilities

Validation

Vote percentage calculations

Zero-vote scenarios

Component logic



---

🛠️ Technology Stack

Frontend

React 19

TypeScript

Vite

Tailwind CSS v4


Blockchain

Soroban SDK v22

Stellar SDK v16

Stellar Wallets Kit


Testing

Rust Unit Tests

Vitest

Testing Library


DevOps

GitHub Actions

Continuous Integration



---

🚀 Getting Started

Prerequisites

Node.js 18+

Rust

wasm32-unknown-unknown target

Freighter Wallet (Testnet)



---

Installation

Clone the repository:

git clone https://github.com/YOUR_USERNAME/stellar-poll.git
cd stellar-poll

Install dependencies:

npm install

Build the smart contract:

cd contract

cargo build --release --target wasm32-unknown-unknown

cp target/wasm32-unknown-unknown/release/live_poll.wasm ../public/live_poll.wasm

Run all tests:

cd contract
cargo test

cd ../voting-badge-contract
cargo test

cd ..
npm test

Start the application:

npm run dev

Fund your wallet using Friendbot:

curl "https://friendbot.stellar.org?addr=YOUR_PUBLIC_KEY"


---

📖 Usage

1. Connect a Stellar wallet.


2. Deploy the LivePoll smart contract.


3. Create a poll with 2–10 options.


4. Cast votes from connected wallets.


5. Monitor live results.


6. View all transactions through Stellar Expert.




---

⚠️ Error Handling

The application gracefully handles common blockchain interaction errors, including:

Wallet not installed

User-rejected transactions

Insufficient XLM balance

Invalid contract state

Failed contract execution



---

📂 Project Structure

src/
├── components/
├── lib/
├── test/
├── App.tsx
├── main.tsx

contract/
├── src/
└── Cargo.toml

voting-badge-contract/
├── src/
└── Cargo.toml

.github/
└── workflows/


---

🎯 Highlights

Fully decentralized voting application

Soroban smart contract deployment from the frontend

Cross-contract communication

NFT voting badges

Real-time blockchain state synchronization

Comprehensive automated testing

CI/CD with GitHub Actions

Mobile-friendly user interface



---

📄 License

Licensed under the MIT License.