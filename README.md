# Stellar Soroban: Decentralized Polling App

## Project Description
This project is a decentralized Polling Application built on the Stellar blockchain using Soroban smart contracts. Written in Rust, it provides a lightweight, highly efficient mechanism for recording and tallying votes on an immutable ledger. This app serves as a foundational template for decentralized governance, community voting, and consensus building on the Stellar network.

## What it does
The Polling App allows users to submit a vote for a specific option (e.g., "Yes", "No", "CandidateA") and permanently records that interaction on the blockchain. It manages state by keeping a running tally of how many votes each option has received. Users and client applications can query the smart contract at any time to get real-time, tamper-proof polling results.

## Features
* **Decentralized Voting:** Votes are cast directly via smart contract invocations, removing the need for a central database.
* **Immutable Results:** Once a vote is cast and the state is updated, the tally cannot be fraudulently altered.
* **Efficient Storage:** Uses Soroban's `Symbol` type and instance storage for highly optimized gas costs.
* **Public Read Access:** Anyone can query the `get_votes` function to see the current state of the poll with zero transaction fees.
* **State Preservation:** Automatically extends the Time To Live (TTL) of the storage upon voting to ensure the poll data remains active on the ledger.

## Deployed Smart Contract Link: Polling app
* **Network:** Stellar Testnet
* **Contract ID:** `CAUSMBOFYFOA6QAZCFMCIOE22MK3A4DOCSF7R3JHSZODQ2AWIQ5WIUS7`
* **Stellar Expert Explorer:** [View on Stellar Expert](https://stellar.expert/explorer/testnet/contract/CAUSMBOFYFOA6QAZCFMCIOE22MK3A4DOCSF7R3JHSZODQ2AWIQ5WIUS7)

---
*Note: To interact with this contract, you will need the [Soroban CLI](https://soroban.stellar.org/docs/getting-started/setup) installed.*
<img width="1919" height="1038" alt="image" src="https://github.com/user-attachments/assets/8139475e-4bf0-4ebf-8365-c9be019aa9f8" />
