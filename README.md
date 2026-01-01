# 🔥 Orderbook DEX

![Status](https://img.shields.io/badge/Status-Work%20In%20Progress-yellow)
![Progress](https://img.shields.io/badge/Progress-23%2F100%20Rings-blue)

> 🚧 **This project is actively under development.** Core data structures are complete; matching engine and instructions coming soon.

A **crankless on-chain orderbook DEX** being built on Solana using **raw Solana** (no Anchor) for maximum performance. Inspired by [Phoenix](https://github.com/Ellipsis-Labs/phoenix-v1).

> ⚡ **Target: ~50μs matching latency** — 4x faster than Anchor-based DEXs

---

## 🎯 Why This Project?

This is a **production-grade rebuild** of the Phoenix orderbook protocol, demonstrating:

- Deep understanding of on-chain orderbook architecture
- FIFO (Price-Time Priority) matching engine implementation
- Zero-copy deserialization for high-frequency trading
- Type-safe Rust programming for DeFi

---

## 🏗️ Architecture

```
src/
├── lib.rs                     # Entry + security.txt
├── quantities.rs              # Type-safe lot/tick system
└── state/
    ├── enums.rs               # Side, SelfTradeBehavior
    ├── trader_state.rs        # Trader position tracking
    ├── inflight_order.rs      # In-flight orders
    ├── matching_engine_response.rs
    └── markets/
        ├── fifo.rs            # FIFOMarket, FIFOOrderId, FIFORestingOrder
        └── market_traits.rs   # Market trait abstraction
```

### Data Structures

| Structure | Purpose | Implementation |
|-----------|---------|----------------|
| `FIFOMarket` | Main market state | Zero-copy from single account |
| `bids` / `asks` | Orderbooks | Red-Black Trees (O(log n) ops) |
| `traders` | Trader registry | Red-Black Tree |

---

## 🔢 Core Formulas

### Lot & Tick System

Discrete units ensuring precision across all operations:

| Unit | Description |
|------|-------------|
| **Atoms** | Smallest on-chain unit (1 SOL = 10⁹ lamports) |
| **Lots** | Orderbook's discrete trading unit |
| **Ticks** | Discrete price increments |

**Conversions:**
```
Base Atoms = Base Lots × Base Atoms Per Base Lot
Quote Atoms = Quote Lots × Quote Atoms Per Quote Lot
```

### Trade Value Calculation

For `B` base lots at price `P` ticks:

$$\text{Quote Lots} = B \times P \times \frac{\text{tick\\_size}}{\text{base\\_lots\\_per\\_unit}}$$

### Fee Calculation (Maker-Taker Model)

$$\text{Fee} = \lceil \frac{\text{trade\\_value} \times \text{taker\\_fee\\_bps}}{10000} \rceil$$

- Only **takers** pay fees
- Fees **rounded up** to prevent dust exploitation

---

## 📊 Order ID & Sorting

### FIFOOrderId Structure

```rust
struct FIFOOrderId {
    price_in_ticks: Ticks,
    order_sequence_number: u64,  // MSB encodes side
}
```

**Side Encoding:** MSB of sequence number = `1` → Bid, `0` → Ask

### Price-Time Priority

| Side | Primary Sort | Secondary Sort |
|------|--------------|----------------|
| **Ask** | Price ↑ (lowest first) | Time ↑ (oldest first) |
| **Bid** | Price ↓ (highest first) | Time ↓ |

---

## 📈 Order Types

| Type | Description |
|------|-------------|
| **PostOnly** | Maker-only, rejects if would cross |
| **Limit** | Match then rest remainder |
| **IOC** | Immediate-or-Cancel, no resting |
| **FOK** | Fill-or-Kill, all or nothing |

### Self-Trade Behavior

| Option | Action |
|--------|--------|
| `DecrementTake` | Execute self-trade |
| `CancelProvide` | Cancel resting order |
| `AbortTransaction` | Abort entire tx |

---

## 🔐 Security

Embedded security metadata via `security_txt!` macro for responsible disclosure.

---

## 🛠️ Tech Stack

```toml
solana-program = "1.14.9"
borsh = "0.9.3"      # Serialization
bytemuck = "1.13.0"  # Zero-copy
sokoban = "0.3.0"    # Red-Black Trees
```

---

## 🚀 Building

```bash
cargo build-sbf   # Build for Solana BPF
cargo test        # Run tests
```

---

## 📖 Progress

Building via **100-ring spiral methodology**:

| Phase | Rings | Status |
|-------|-------|--------|
| Foundation | BR-1→5 | ✅ Complete |
| Quantities System | BR-6→11 | ✅ Complete |
| Basic State | BR-12→16 | ✅ Complete |
| Market Structures | BR-17→24 | 🔄 In Progress |
| Order Schema | BR-25→30 | ⬜ Pending |
| Matching Engine | BR-54→62 | ⬜ Pending |
| Testing | BR-84→97 | ⬜ Pending |

---

## 📚 References

- [Phoenix V1](https://github.com/Ellipsis-Labs/phoenix-v1) — Original implementation
- [Phoenix Docs](https://ellipsis-labs.gitbook.io/phoenix-dex/) — Official documentation
- [Sokoban](https://docs.rs/sokoban) — Red-Black Tree library

---

## 👤 Author

**Veerbal Singh**  
📧 veerbal.singh369@gmail.com

---

<p align="center">
  <i>Built with 🦀 Rust on ◎ Solana</i>
</p>
