# LibrePass

**LibrePass** is a decentralized micro-payment and administrative clearance dApp designed for university libraries. It enables students to settle outstanding late book fees instantly via stablecoins and automatically updates their clearance status on-chain, eliminating long physical cash queues and administrative document friction.

---

## 📌 Problem & Solution

* **The Problem:** University students at public colleges in Manila, Philippines, experience delayed graduation clearances and hours of manual queuing because library overdue micro-fines must be paid in cash at centralized cashiers who often misplace physical paper receipts.
* **The Solution:** LibrePass allows students to clear library late fees instantly via a lightweight web app using Stellar USDC, executing a Soroban smart contract that programmatically updates their library clearance status on-chain for a fraction of a cent.

---

## 🛠️ Stellar Features Used

* **Stellar USDC Transfers:** Provides low-fee, high-speed, stable currency payments matching local economic constraints.
* **Soroban Smart Contracts:** Handles fine state registration, enforces authorization boundaries, and mutates administrative compliance status immutably.
* **Trustlines:** Ensures secure token balance holding capabilities across student and institutional infrastructure.

---

## 🚀 Vision and Purpose

The primary vision of LibrePass is to eliminate administrative gridlocks in emerging public education sectors. By moving high-frequency micro-payment flows onto Stellar, institutions minimize administrative overhead, securely maintain records outside silos, and provide students with instant, cryptographically verifiable proof of graduation clearance.

---

## ⏳ Project Timeline (Bootcamp Sprint)

```text
 📅 Days 1–3  ───► Soroban Smart Contract Architecture & Unit Testing Suites
 📅 Days 4–6  ───► Local Anchor integration Mocks (Fiat-to-USDC) & Web Frontend Hooks
 📅 Day 7     ───► Testnet Deployment, Contract Invocations, and Final Demo Recording

## Deployed Contract

| Field | Value |
|-------|-------|
| Contract ID | `CA5HKTN2YAFV6HGPAFAFQPM66BSOMFREKEZJ2UUIEYWD3MCK5WARJC5X` |
| Network | testnet |
| Explorer | [View on stellar.expert](https://stellar.expert/explorer/testnet/contract/CA5HKTN2YAFV6HGPAFAFQPM66BSOMFREKEZJ2UUIEYWD3MCK5WARJC5X) |
| Deploy Tx | [View transaction](https://stellar.expert/explorer/testnet/tx/7ad75263b7f819f6e5cf26684136ec19c5f0567c4dddab0cdf3b6040bb1c5bbe) |
| Deployed | 2026-06-26 06:33:15 UTC |
| Wallet | freighter (`GBVV…TRHX`) |
