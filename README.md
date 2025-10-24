# DevQuest - Solana Task Management Platform

A decentralized task management dApp built on Solana where developers complete tasks, earn points, and showcase their work.

## Overview

DevQuest is a gamified platform that allows developers to:
- Complete coding tasks and earn points
- Showcase projects
- Compete with other developers

**Live Demo:** https://questdev.vercel.app/

**Program ID (Devnet):** `G6kyX8C369qABor4ZVXDFXneZw9jqDpbfB79imLgEGfA`

## Architecture

```
Frontend (React) → Solana RPC → Anchor Program
```

**Key Components:**
- **Frontend**: React with Solana wallet integration
- **Backend**: Anchor program on Solana blockchain
- **Storage**: On-chain account data using PDAs

## Features

### User Features
- **Wallet Authentication**: Connect with a wallet
- **Profile Management**: Update name, bio, GitHub username
- **Task Completion**: Submit GitHub repos for tasks
- **Point System**: Earn points for completed tasks
- **Project Showcase**: Showcase your projects

### Admin Features
- **Task Creation**: Create new tasks with points rewards

## Technical Stack

- **Blockchain**: Solana
- **Framework**: Anchor (Rust)
- **Frontend**: React + TypeScript
- **Wallet**: Solana Wallet Adapter
- **RPC**: Solana Web3.js

## Project Structure

```
dev-quest/
├── programs/
│   └── dev-quest/
│       ├── src/
│       │   ├── lib.rs
│       │   ├── instructions/
│       │   ├── state/
│       │   └── errors.rs
├── tests/
   └── dev-quest.ts

```

## Getting Started

### Prerequisites

Install all necessary dependencies with one command:

```bash
curl --proto '=https' --tlsv1.2 -sSfL https://solana-install.solana.workers.dev | bash
```

This installs:
- Rust
- Solana CLI
- Anchor CLI
- Node.js
- Yarn

Verify installation:
```bash
rustc --version && solana --version && anchor --version && node --version && yarn --version
```

### Installation

1. **Clone repository**
```bash
git clone <repo-url>
cd dev-quest
```

2. **Build program**
```bash
anchor build
```

3. **Deploy to devnet**
```bash
anchor deploy
```

4. **Run tests**
```bash
anchor test
```

## Program Instructions

| Instruction | Description | Access |
|-------------|-------------|---------|
| `initialize_config` | Setup program configuration | Admin only |
| `initialize_user` | Create user profile | Public |
| `create_task` | Add new task | Admin only |
| `submit_task` | Submit task completion | Public |

## Security Features

- **PDA-based accounts**: All data stored in Program Derived Addresses
- **Admin controls**: Only authorized admins can create tasks
- **Duplicate prevention**: Users can't submit same task twice
- **Input validation**: All inputs validated on-chain

## Testing

The project includes tests covering for:
- Account initialization
- Task creation and submission
- Point rewards system

Run tests with:
```bash
anchor test
```

## Future Enhancements
- Token rewards
- NFT rewards for milestones as badges
