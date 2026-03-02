<div align="center">

# ⚽ Scout Chain: Solana Scouting Program

[![Solana](https://img.shields.io/badge/Solana-Blockchain-9945FF?style=for-the-badge&logo=solana&logoColor=white)](https://solana.com/)
[![Rust](https://img.shields.io/badge/Rust-Programming_Language-000000?style=for-the-badge&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![Anchor](https://img.shields.io/badge/Anchor-Framework-1E1E1E?style=for-the-badge)](https://www.anchor-lang.com/)
[![License](https://img.shields.io/badge/License-MIT-green?style=for-the-badge)](LICENSE)

**A decentralized scouting platform for football players and scouts built on Solana.**

</div>

---

## 🎯 About The Project

**Scout Chain** is a smart contract built using the Anchor framework that facilitates a transparent scouting ecosystem. It allows players to showcase their stats and scouts to provide verified on-chain reviews.

### Core Architecture: PDAs & CRUD
The program utilizes **Program Derived Addresses (PDAs)** to ensure data integrity and user-specific ownership. It implements a full **CRUD** (Create, Read, Update, Delete) cycle for:
* **Players:** Personal profiles with stats and video links.
* **Scouts:** Professional profiles with organization and experience.
* **Reviews:** Dynamic reviews linked between a scout and a player.

---

## ✨ Features

### ⚽ Player Management
* **Create Profile:** Initialize a unique PDA based on the user's wallet.
* **Update Stats:** Edit position, age, and performance metrics (CRUD: Update).
* **On-Chain Metrics:** Track total reviews and average ratings (CRUD: Read).

### 🔍 Scouting & Reviews
* **Verified Scouts:** Only registered scouts can leave reviews.
* **Dynamic Review PDAs:** Reviews are derived using the player's key, the scout's key, and a review counter.
* **Deactivation:** Ability to deactivate profiles to prevent further interactions.
* **Rent Recovery:** Delete reviews to close accounts and recover SOL (CRUD: Delete).

### 🔐 Security
* **Owner Validation:** `require!` macros ensure only the account creator can modify their data.
* **PDA Seeds:** * `["player", authority]`
    * `["scout", authority]`
    * `["review", player_pubkey, scout_pubkey, review_count]`

---

## 🏗️ Program Architecture

### 📦 Player Account
```rust
Player {
    authority: Pubkey,
    name: String,
    position: String,
    age: u8,
    stats: String,
    video_url: String,
    review_count: u32,
    rating_sum: u32,
    is_active: bool,
}

```

### 🔑 PDA Seeds

```text
["player", player_authority_pubkey]
["scout", scout_authority_pubkey]
["review", player_pda, scout_pda, review_index]

```

---

## 🌐 Deployment Information

The program is live on **Solana Devnet**. You can verify the instructions and account states on the explorer:

**Program ID:** `CXYaPQ451V8DD43pWCepNyddkf2Pnjk3mqvLgchJR5Fb`

[View on Solana Explorer](https://www.google.com/search?q=https://explorer.solana.com/address/CXYaPQ451V8DD43pWCepNyddkf2Pnjk3mqvLgchJR5Fb%3Fcluster%3Ddevnet)

---

## 🧪 Testing & Devnet Validation

The test suite is built in TypeScript and performs a full lifecycle check. It uses fresh `Keypair` generation for every run to avoid PDA collisions.

### Test Screenshot

<div align="center">

`![Test Results Placeholder](./assets/test-results.png)`

</div>

### How to run the tests

1. Ensure you have Devnet SOL in your local wallet (`solana balance`).
2. Run the following command:

```bash
anchor test --provider.cluster devnet --skip-deploy --skip-local-validator

```

The test will log:

* **Transaction Links:** Direct URLs to Solana Explorer for every action.
* **CRUD Logs:** Confirmation of player creation (Pedri), scout creation (Xavi), and review management.

---

## 🔧 Installation & Setup

### Prerequisites

| Requirement | Version |
| --- | --- |
| Rust | Latest stable |
| Solana CLI | 1.18+ |
| Anchor CLI | 0.30+ |
| Node.js | 20+ |

### Quick Start

```bash
# Clone the repository
git clone [https://github.com/Andres/scout-chain.git](https://github.com/Andres/scout-chain.git)

# Install dependencies
npm install

# Build the program
anchor build

# Run tests on Devnet
anchor test --provider.cluster devnet

```

---

## 🛡️ Error Handling

Custom errors defined in the program:

* `InvalidRating`: Rating must be between 1 and 5.
* `Unauthorized`: Only the owner can modify the account.
* `InactiveProfile`: Cannot interact with deactivated accounts.

---

## 📜 License

This project is licensed under the MIT License.

---

## 👨‍💻 Author

<div align="center">

**Andrés**

Built with ❤️ using Rust, Anchor & Solana

</div>
