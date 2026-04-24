# 🗳️ VoteChain — On-Chain Voting dApp on Stellar

> A decentralised, tamper-proof voting application built with **Soroban** smart contracts on the **Stellar** blockchain.

---

## 📖 Project Description

VoteChain is a trustless voting dApp that runs entirely on the Stellar network via Soroban — Stellar's smart-contract platform. All proposals and ballots are stored on-chain, so the results are **transparent, auditable, and immutable**. No central server holds the data; anyone with a Stellar wallet can participate.

Whether you're running a DAO governance vote, a community poll, or an organisation-wide decision, VoteChain gives you a lightweight and cost-effective foundation to build on.

---

## ✨ What It Does

| Step | Actor | Action |
|------|-------|--------|
| 1 | **Admin** | Deploys and initialises the contract, setting themselves as the owner |
| 2 | **Admin** | Opens or closes the voting window at any time |
| 3 | **Anyone** | Submits a new proposal (title + description) while voting is open |
| 4 | **Anyone** | Casts a vote on any proposal they haven't already voted on |
| 5 | **Anyone** | Queries live vote counts, proposal details, and voter status |

All authentication is handled by Stellar's native signature verification — no passwords, no sessions.

---

## 🚀 Features

### Core Voting Logic
- **Create proposals** — any authenticated address can submit a titled proposal with a description
- **One vote per address per proposal** — the contract enforces this on-chain; double-voting panics immediately
- **Live vote counts** — query real-time tallies for any proposal at any time

### Access Control
- **Admin initialisation** — a single admin is set at deploy time; the contract panics if `initialize` is called twice
- **Voting window control** — the admin can open or close voting; all write operations (proposals + votes) respect the window
- **Auth-required actions** — every state-changing call (`add_proposal`, `vote`, `set_voting_open`) requires the caller to sign the transaction

### Transparency & Auditability
- **Persistent on-chain storage** — proposals and vote records live in Soroban's persistent storage, surviving ledger TTL extensions
- **Has-voted query** — anyone can verify whether a specific address has voted on a specific proposal
- **Admin query** — the current admin address is always publicly readable

### Developer Experience
- **Pure Rust / `no_std`** — compiles to a tiny WASM binary optimised for the Soroban runtime
- **Full test suite** — unit tests cover happy paths, double-vote prevention, and closed-window enforcement
- **Workspace layout** — clean Cargo workspace ready to add more contracts (token integration, NFT badge minting, etc.)

---

## 🗂️ Project Structure

```
voting-dapp/
├── Cargo.toml                        # Workspace root
└── contracts/
    └── voting/
        ├── Cargo.toml                # Contract crate
        └── src/
            └── lib.rs                # Contract + tests
```

---

## 🛠️ Prerequisites

| Tool | Version | Install |
|------|---------|---------|
| Rust | ≥ 1.74 | `curl https://sh.rustup.rs -sSf \| sh` |
| `wasm32-unknown-unknown` target | — | `rustup target add wasm32-unknown-unknown` |
| Stellar CLI | ≥ 21 | [stellar.org/developers](https://developers.stellar.org/docs/tools/developer-tools/cli/install-stellar-cli) |

---

## ⚙️ Build

```bash
# Clone
cd voting-dapp

# Build optimised WASM
stellar contract build
# Output: target/wasm32-unknown-unknown/release/voting_contract.wasm
```

---

## 🧪 Run Tests

```bash
cargo test -p voting-contract
```

All five test cases should pass:

```
test tests::test_initialize                       ... ok
test tests::test_add_proposal                     ... ok
test tests::test_vote                             ... ok
test tests::test_double_vote_fails                ... ok
test tests::test_vote_when_closed_fails           ... ok
test tests::test_multiple_proposals_and_voters    ... ok
```

---

## 🚢 Deploy to Testnet

```bash
# Configure Testnet identity (one-time)
stellar keys generate --global alice --network testnet
stellar keys fund alice --network testnet   # free Friendbot XLM

# Deploy
stellar contract deploy \
  --wasm target/wasm32-unknown-unknown/release/voting_contract.wasm \
  --source alice \
  --network testnet

# The CLI prints your CONTRACT_ID — save it!

# Initialise (replace placeholders)
stellar contract invoke \
  --source alice \
  --network testnet \
  -- initialize \```

---

## 📜 Contract API Reference

### Write Functions

| Function | Parameters | Description |
|----------|-----------|-------------|
| `initialize` | `admin: Address` | One-time setup; sets admin and opens voting |
| `set_voting_open` | `open: bool` | Admin only — open or close the voting window |
| `add_proposal` | `creator, title, description` | Submit a new proposal; returns its `u32` id |
| `vote` | `voter: Address, proposal_id: u32` | Cast one vote on a proposal |

### Read Functions

| Function | Returns | Description |
|----------|---------|-------------|
| `get_proposal` | `Proposal` | Full proposal struct by id |
| `proposal_count` | `u32` | Total number of proposals |
| `get_vote_count` | `u32` | Current votes for a proposal |
| `has_voted` | `bool` | Whether an address voted on a proposal |
| `voting_open` | `bool` | Is the voting window currently open? |
| `admin` | `Address` | Current admin address |

---

## 🗺️ Roadmap

- [ ] **Token-weighted voting** — integrate Stellar's native asset or a custom SEP-41 token for vote weight
- [ ] **Voting deadlines** — use Soroban's ledger sequence number to set automatic close times
- [ ] **NFT participation badge** — mint a Soroban NFT to every voter as proof of participation
- [ ] **Frontend** — React + `@stellar/stellar-sdk` UI for wallet connection and proposal browsing
- [ ] **Event emission** — emit Soroban events for off-chain indexers and dashboards

---


wallet address: GCY2E42DK2GAAXN4UM32PUUVGXDSYWPT2KUMTXND3R4WX7DTPEFRQRHF

contract address: CBBKLLFCH67LLV2MM7PHQVF3KBW42TZAS2IQO6KU4QBMLZLGB5S4HAOS

https://stellar.expert/explorer/testnet/contract/CBBKLLFCH67LLV2MM7PHQVF3KBW42TZAS2IQO6KU4QBMLZLGB5S4HAOS

<img width="1918" height="947" alt="image" src="https://github.com/user-attachments/assets/da75c160-2fb0-4d29-b608-bb323fa07708" />
