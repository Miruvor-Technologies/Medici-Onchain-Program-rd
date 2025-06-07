# Medici On‑chain Program

Medici.ac is a **peer‑to‑peer scholarships platform** that lets donors pick verified students, choose any amount of USDC, and fund their education **securely and directly on Solana**. Every transfer is transparent, on‑chain, and subject to a small, configurable platform fee directly transferred to Medici's Solana wallet.  

---

### Repository at a glance

This repo contains the **minimal smart‑contract back‑end** needed for the MVP’s *many‑to‑one* donor flow, where any number of donors can send funds into a single student’s wallet. Key folders:

| Path                               | What it holds                                                                                                                        |
| ---------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------ |
| `programs/medici-onchain-program/` | The Anchor workspace — Rust code for all on‑chain logic.                                                                             |
| `programs/…/src/instructions/`     | Three instructions, including the core `send_amount_from_donor_to_student.rs` that moves tokens donor ➜ student and applies the fee. |
| `programs/…/src/state/`            | Persistent on‑chain accounts (PDAs) such as `FeesConfigurationAccount`.                                                              |
| `tests/`                           | Anchor/TypeScript tests illustrating the donor → student flow.                                                                       |
| `migrations/`                      | Optional deployment helper script.                                                                                                   |

Together these pieces demonstrate the **basic many‑to‑one flow** we’ll ship in the MVP: a donor signs a transaction, our program calculates the platform fee, and the net amount lands in the student’s token account.

---

### Technical stack

- **Solana + Anchor** — Rust smart contract framework that handles PDAs, CPI, and serialization.
- **Rust** (`no_std`) for on‑chain logic; `anchor_spl::token` for SPL‑Token transfers.
- **TypeScript** tests run via `anchor test`, giving a localnet harness for rapid iteration.
- Built‑in hooks for adding additional instructions (e.g. treasury fee collection) and for expanding tests and migrations as the product grows.

---

> **Next milestones**: perform comprehensive payments testing, complete a smart contract audit and explore new implementation prototypes — including milestone‑based scholarship releases, income-sharing agreements, and on-chain verification via Solana Attestation Service.
