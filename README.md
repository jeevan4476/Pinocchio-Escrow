# Escrow Program 

Designed for transparency, control, and raw performance, this program manually handles account verification, instruction parsing, and token transfers — with no strings attached (except the escrow ones).

> 🧪 **Status**: Core logic implemented. Tests are underway.

---

## 🎯 Features

- SPL Token escrow with strict access control
- Support for:
  - Creating an escrow vault
  - Depositing funds (maker)
  - Claiming funds (taker)
  - Refunding to original sender
  - Canceling the escrow
- Manual instruction parsing and PDA derivation
- Designed for reliability and auditability


---

## 🛠 Instructions Overview

| Instruction | Role     | Description                                      |
|-------------|----------|--------------------------------------------------|
| `Make`      | Maker    | Deposits tokens into escrow                      |
| `Take`      | Taker    | Takes tokens if criteria are met                 |
| `Refund`    | Maker    | Refunds tokens back to maker (e.g., timeout)     |

---

