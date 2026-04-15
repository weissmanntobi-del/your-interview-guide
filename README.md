## Complete Rust Material : https://tobiweissmann.gumroad.com/l/gnuvxu
# Rust Interview Prep

A structured, progressive challenge set to prepare for senior Rust infrastructure engineer roles in the Solana ecosystem.

## Quick Start

```bash
# Clone and enter
git clone https://github.com/meowyx/rust-interview-prep
cd rust-interview-prep

# Get the exercises to start practicing (main branch has solutions and notes)
git checkout exercises

# Run everything (expected to fail until you solve challenges)
cargo test

# Run one phase
cargo test -p phase1-rust-systems

# Run one challenge (test name filter)
cargo test -p phase1-rust-systems -- challenge_01

# Optional: auto-rerun tests on save
cargo install cargo-watch
cargo watch -x "test -p phase1-rust-systems -- challenge_01"

# Quality checks
cargo check
cargo clippy -- -W clippy::all
```

## How To Work Through It

1. Start with `rust-concepts/` to learn Rust fundamentals concept by concept.
2. Move to Phase 1 when you feel confident with ownership, borrowing, structs, and traits.
3. For each challenge, read tests first: tests are the spec.
4. Solve without changing tests unless a prompt explicitly says to.
5. Move phase by phase; do not skip phases 1-3.
6. Time-box yourself (30-75 min per challenge depending on difficulty).

## Roadmap At A Glance

| Phase | Theme | Challenges |
| --- | --- | --- |
| concepts | Rust fundamentals (concept-by-concept teaching notes) | 24 pure Rust + 4 Solana/Pinocchio |
| 1 | Rust systems fundamentals | 8 |
| 2 | Data structures | 8 |
| 3 | Algorithms | 8 |
| 4 | Serialization and encoding | 2 |
| 5 | Cryptography primitives | 3 |
| 6 | Solana runtime internals | 8 |
| 7 | Networking and P2P | 7 |
| 8 | Performance and benchmarking | 6 |
| 9 | System design (written) | 6 |
| 10 | Advanced Rust | 8 |
| 11 | Accounts DB | 6 |
| 12 | Fee and scheduling | 5 |
| 13 | Program runtime | 5 |
| 14 | RPC and data | 6 |
| 15 | DeFi internals | 5 |
| 16 | Testing and debugging | 5 |
| 17 | Production ops | 2 |

## Detailed Phase Breakdown

<details>
<summary><strong>Rust Concepts — Fundamentals Learning Path</strong></summary>

23 concepts taught one at a time with explanations, code examples, and mini quizzes.
See [`rust-concepts/README.md`](rust-concepts/README.md) for the full list.

| Stop | Concepts | Topics |
| --- | --- | --- |
| 1-2 | 1–6 | Ownership, moves, borrowing, `&mut`, borrow checker, slices |
| 3 | 7–10 | Structs, methods, enums, pattern matching |
| 4 | 11–14 | Generics, traits, trait bounds, lifetimes |
| 5 | 15–17 | Closures, `Fn`/`FnMut`/`FnOnce`, iterator chains |
| 6 | 18–20 | `Box`, `Rc`, `RefCell` |
| 7 | 21–24 | `impl Trait` vs `dyn Trait`, function pointers, error handling, `From`/`Into` |
| Solana | S1–S4 | Borsh, zero-copy parsing, Bytemuck, Wincode |

</details>

<details>
<summary><strong>Phase 1 - Rust Systems Fundamentals</strong></summary>

| # | Challenge | Key Concept | Time | Difficulty |
| --- | --- | --- | --- | --- |
| 1.1 | Lifetime Arena Allocator | Custom allocator, lifetimes, unsafe | 60min | Hard |
| 1.2 | Zero-Copy Parser | &[u8] parsing without allocation | 45min | Medium |
| 1.3 | Lock-Free Queue | AtomicPtr, CAS, memory ordering | 75min | Hard |
| 1.4 | Thread Pool | Channel-based work stealing | 60min | Hard |
| 1.5 | Async Stream Processor | tokio Stream, backpressure, cancellation | 60min | Hard |
| 1.6 | Custom Smart Pointer | Deref, Drop, interior mutability | 45min | Medium |
| 1.7 | Error Handling Framework | thiserror, type-state error propagation | 30min | Medium |
| 1.8 | Generic Trait Constraints | Send + Sync + 'static, trait objects | 45min | Medium |

</details>

<details>
<summary><strong>Phase 2 - Data Structures</strong></summary>

| # | Challenge | Key Concept | Time | Difficulty |
| --- | --- | --- | --- | --- |
| 2.1 | LRU Cache | O(1) get/put with capacity eviction | 45min | Medium |
| 2.2 | Merkle Tree | Build, proof generation, verification | 60min | Hard |
| 2.3 | Merkle Patricia Trie | Insert, lookup, delete, proof | 90min | Hard |
| 2.4 | Priority Queue (Min-Heap) | Transaction fee ordering | 40min | Medium |
| 2.5 | Bloom Filter | Probabilistic membership testing | 30min | Medium |
| 2.6 | Skip List | O(log n) ordered map alternative | 60min | Hard |
| 2.7 | Concurrent HashMap | Sharded locking, lock-free reads | 75min | Hard |
| 2.8 | Ring Buffer | Fixed-size circular buffer for logging | 30min | Easy |

</details>

<details>
<summary><strong>Phase 3 - Algorithms</strong></summary>

| # | Challenge | Key Concept | Time | Difficulty |
| --- | --- | --- | --- | --- |
| 3.1 | Transaction Dependency Graph | Topological sort with conflict detection | 45min | Medium |
| 3.2 | Token Swap Router | Dijkstra / Bellman-Ford on DEX graph | 60min | Hard |
| 3.3 | Block Packing (Knapsack) | Maximize gas usage within block limit | 45min | Medium |
| 3.4 | Reorg Detection | Longest chain / fork choice with rollback | 60min | Hard |
| 3.5 | Nonce Gap Resolution | Windowed sorting with gap detection | 40min | Medium |
| 3.6 | Rate Limiter | Token bucket + sliding window | 40min | Medium |
| 3.7 | Parallel Batch Scheduler | Dependency-aware parallel execution | 60min | Hard |
| 3.8 | Binary Search on Chain State | Bisection search for state at block N | 30min | Medium |

</details>

<details>
<summary><strong>Phase 4 - Serialization & Encoding</strong></summary>

| # | Challenge | Key Concept | Time | Difficulty |
| --- | --- | --- | --- | --- |
| 4.1 | Borsh Encoder/Decoder | Solana's binary serialization | 45min | Medium |
| 4.2 | Zero-Copy Account Parser | Parse Solana account data without alloc | 45min | Medium |

</details>

<details>
<summary><strong>Phase 5 - Cryptography Primitives</strong></summary>

| # | Challenge | Key Concept | Time | Difficulty |
| --- | --- | --- | --- | --- |
| 5.1 | Merkle Proof Verifier | Verify inclusion with hash path | 30min | Easy |
| 5.2 | HD Key Derivation | BIP-32/BIP-44 hierarchical key paths | 60min | Hard |
| 5.3 | PDA Derivation | Solana Program Derived Addresses | 30min | Medium |

</details>

<details>
<summary><strong>Phase 6 - Solana Runtime Internals</strong></summary>

| # | Challenge | Key Concept | Time | Difficulty |
| --- | --- | --- | --- | --- |
| 6.1 | Account Model | Owner, lamports, data, rent, executable | 45min | Medium |
| 6.2 | Compute Budget Meter | CU tracking per instruction | 30min | Medium |
| 6.3 | Transaction Processor | Sigverify → load accounts → execute | 60min | Hard |
| 6.4 | Parallel Executor (Sealevel) | Non-conflicting tx parallel execution | 75min | Hard |
| 6.5 | Vote Processing | Tower BFT vote lockout and fork choice | 60min | Hard |
| 6.6 | Leader Schedule | Stake-weighted leader selection per slot | 45min | Medium |
| 6.7 | Shred Generator | Split block data into erasure-coded shreds | 60min | Hard |
| 6.8 | Entry Verification | Proof of History tick verification | 45min | Medium |

</details>

<details>
<summary><strong>Phase 7 - Networking & P2P</strong></summary>

| # | Challenge | Key Concept | Time | Difficulty |
| --- | --- | --- | --- | --- |
| 7.1 | TCP Echo Server | tokio TCP, framing, connection handling | 30min | Easy |
| 7.2 | Request Multiplexer | Async request routing with timeouts | 45min | Medium |
| 7.3 | Gossip Protocol | Peer discovery and message propagation | 60min | Hard |
| 7.4 | JSON-RPC Server | Method dispatch, params parsing, batching | 60min | Hard |
| 7.5 | Connection Pool | Reusable connections with health checks | 45min | Medium |
| 7.6 | Circuit Breaker | Failure detection and auto-recovery | 30min | Medium |
| 7.7 | Block Propagation Simulator | Tree-based propagation with latency model | 60min | Hard |

</details>

<details>
<summary><strong>Phase 8 - Performance & Benchmarking</strong></summary>

| # | Challenge | Key Concept | Time | Difficulty |
| --- | --- | --- | --- | --- |
| 8.1 | Optimize a Slow Hasher | Profile and fix intentionally slow code | 45min | Medium |
| 8.2 | Batch vs Single Processing | Amortize overhead through batching | 45min | Medium |
| 8.3 | SIMD-Friendly Layout | Struct-of-arrays vs array-of-structs | 60min | Hard |
| 8.4 | Memory Pool Allocator | Object pool to avoid allocation churn | 60min | Hard |
| 8.5 | Parallel Merkle Root | Parallelize tree hashing with rayon | 45min | Medium |
| 8.6 | Database Write Optimization | Batch writes, WAL tuning, compression | 60min | Hard |

</details>

<details>
<summary><strong>Phase 9 - System Design (Written)</strong></summary>

| # | Challenge | Key Concept | Time | Difficulty |
| --- | --- | --- | --- | --- |
| 9.1 | High-Throughput RPC Node | Caching, load balancing, rate limiting | 60min | Hard |
| 9.2 | Jito MEV / Block Engine | Bundle flow, leader forwarding, latency | 60min | Hard |
| 9.3 | Cross-Chain Bridge | Proof verification, security, liveness | 60min | Hard |
| 9.4 | Stablecoin Settlement Layer | Batch processing, finality, compliance | 60min | Hard |
| 9.5 | Validator Monitoring System | Metrics collection, alerting, dashboards | 45min | Medium |
| 9.6 | Transaction Indexer at Scale | 50k+ TPS ingestion, query optimization | 60min | Hard |

</details>

<details>
<summary><strong>Phase 10 - Advanced Rust</strong></summary>

| # | Challenge | Key Concept | Time | Difficulty |
| --- | --- | --- | --- | --- |
| 10.1 | Typestate Pattern | Transaction lifecycle, compile-time state machine | 60min | Hard |
| 10.2 | Declarative Macros | Account/error/instruction codegen, macro_rules! | 60min | Hard |
| 10.3 | Pin & Self-Referential Structs | Pin, self-ref pointer, manual Future | 60min | Hard |
| 10.4 | Cow Optimization | LogParser with Cow<'a, str>, zero-copy when possible | 45min | Medium |
| 10.5 | Middleware Pipeline | Tower-style composable services, rate limit, retry | 60min | Hard |
| 10.6 | Memory-Mapped File Reader | Zero-copy ledger record parsing from buffer | 60min | Hard |
| 10.7 | Iterator Combinators | batch, dedup_by_key, take_while_inclusive | 45min | Medium |
| 10.8 | Unsafe Audit | Soundness verdicts on 6 unsafe code blocks | 45min | Hard |

</details>

<details>
<summary><strong>Phase 11 - Accounts DB</strong></summary>

| # | Challenge | Key Concept | Time | Difficulty |
| --- | --- | --- | --- | --- |
| 11.1 | Append-Only Storage | AppendVec: serialize accounts, read by offset | 45min | Medium |
| 11.2 | Account Index | pubkey → (slot, offset), latest + historical | 60min | Hard |
| 11.3 | Snapshot Generator | Deterministic snapshot, hash, serialize/verify | 60min | Hard |
| 11.4 | Account Hash Accumulator | Order-independent incremental hash over accounts | 45min | Medium |
| 11.5 | Dead Account Cleanup | Zero-lamport GC, reclaimable storage, shrink | 45min | Medium |
| 11.6 | Write Cache | Write-through cache, batch flush, dedupe by pubkey | 60min | Hard |

</details>

<details>
<summary><strong>Phase 12 - Fee & Scheduling</strong></summary>

| # | Challenge | Key Concept | Time | Difficulty |
| --- | --- | --- | --- | --- |
| 12.1 | Priority Fee Calculator | CU limit, micro-lamport price, ranking | 45min | Medium |
| 12.2 | Write Lock Scheduler | Conflict batching, greedy parallel batches | 60min | Hard |
| 12.3 | Bundle Processor | Jito-style all-or-nothing execution, tip | 60min | Hard |
| 12.4 | Durable Nonce | Init, advance, verify, use (nonce lifecycle) | 45min | Medium |
| 12.5 | Fee Estimator | Sliding window, percentile fee estimate | 45min | Medium |

</details>

<details>
<summary><strong>Phase 13 - Program Runtime</strong></summary>

| # | Challenge | Key Concept | Time | Difficulty |
| --- | --- | --- | --- | --- |
| 13.1 | Mini eBPF Interpreter | VM: registers, memory, instruction set | 90min | Hard |
| 13.2 | Syscall Dispatcher | Route by id, compute metering, SolLog/SolGetClock | 60min | Hard |
| 13.3 | BPF Account Serialization | Wire format for program input accounts | 45min | Medium |
| 13.4 | CPI Depth Tracker | Depth limit, reentrancy, privilege escalation | 60min | Hard |
| 13.5 | LRU Program Cache | Compiled program cache, eviction, slot invalidation | 60min | Hard |

</details>

<details>
<summary><strong>Phase 14 - RPC & Data</strong></summary>

| # | Challenge | Key Concept | Time | Difficulty |
| --- | --- | --- | --- | --- |
| 14.1 | getAccountInfo | Encoding: Base58, Base64, JsonParsed | 45min | Medium |
| 14.2 | getTransaction | Compiled tx parse, account key resolution | 45min | Medium |
| 14.3 | Account Subscription | WebSocket pubsub, commitment filtering | 60min | Hard |
| 14.4 | Slot Subscription | Slot tracking, forks, finalization | 60min | Hard |
| 14.5 | Token Account Index | SPL token indexing, by owner/mint, ranked | 60min | Hard |
| 14.6 | DAS getAsset | Compressed NFTs, Merkle proof verification | 60min | Hard |

</details>

<details>
<summary><strong>Phase 15 - DeFi Internals</strong></summary>

| # | Challenge | Key Concept | Time | Difficulty |
| --- | --- | --- | --- | --- |
| 15.1 | Constant Product AMM | x*y=k, swap, add/remove liquidity | 45min | Medium |
| 15.2 | Concentrated Liquidity | Tick-based CLMM, sqrt_price Q64.64 | 75min | Hard |
| 15.3 | Order Book | Price-time priority matching | 60min | Hard |
| 15.4 | Liquidation Engine | Health factor, collateral/debt, liquidate | 60min | Hard |
| 15.5 | Oracle Aggregator | Median, TWAP, staleness filtering | 45min | Medium |

</details>

<details>
<summary><strong>Phase 16 - Testing & Debugging</strong></summary>

| # | Challenge | Key Concept | Time | Difficulty |
| --- | --- | --- | --- | --- |
| 16.1 | Property Testing | Proptest strategies, transfer/serialize invariants | 45min | Medium |
| 16.2 | Fuzz Deserializer | Find bugs in buggy parser with proptest | 45min | Medium |
| 16.3 | Program Test Harness | LiteSVM-style: accounts, signers, dispatch | 60min | Hard |
| 16.4 | Reorg & Replay | Fork detection, rollback, reapply | 60min | Hard |
| 16.5 | Debug Deadlock | Fix lock-order/await deadlocks, simple detector | 45min | Hard |

</details>

<details>
<summary><strong>Phase 17 - Production Ops</strong></summary>

| # | Challenge | Key Concept | Time | Difficulty |
| --- | --- | --- | --- | --- |
| 17.1 | Structured Logging | Levels, spans, key-value fields | 45min | Medium |
| 17.2 | Metrics Exporter | Counter, Gauge, Histogram, Prometheus text | 45min | Medium |

</details>

## Where Things Live

- Concept teaching notes: `rust-concepts/`
- Coding phases: `phase1-rust-systems` through `phase17-production-ops`
- System design prompts: `phase9-system-design/README.md`
- System design template: `phase9-system-design/TEMPLATE.md`

<details>
<summary><strong>Progress Tracker</strong></summary>

```
Concepts:     [x] 1   [x] 2   [x] 3   [x] 4   [x] 5   [x] 6   [x] 7   [x] 8
              [x] 9   [x] 10  [x] 11  [x] 12  [x] 13  [x] 14  [x] 15  [x] 16
              [x] 17  [x] 18  [x] 19  [x] 20  [x] 21  [x] 22  [x] 23  [x] 24
Solana/Pinocchio: [x] S1-Borsh  [x] S2-ZeroCopy  [x] S3-Bytemuck  [x] S4-Wincode
Phase 1:  Rust Systems       [ ] 1.1  [ ] 1.2  [ ] 1.3  [ ] 1.4  [ ] 1.5  [ ] 1.6  [ ] 1.7  [ ] 1.8
Phase 2:  Data Structures    [ ] 2.1  [ ] 2.2  [ ] 2.3  [ ] 2.4  [ ] 2.5  [ ] 2.6  [ ] 2.7  [ ] 2.8
Phase 3:  Algorithms         [ ] 3.1  [ ] 3.2  [ ] 3.3  [ ] 3.4  [ ] 3.5  [ ] 3.6  [ ] 3.7  [ ] 3.8
Phase 4:  Serialization      [ ] 4.1  [ ] 4.2
Phase 5:  Cryptography       [ ] 5.1  [ ] 5.2  [ ] 5.3
Phase 6:  Solana Runtime     [ ] 6.1  [ ] 6.2  [ ] 6.3  [ ] 6.4  [ ] 6.5  [ ] 6.6  [ ] 6.7  [ ] 6.8
Phase 7:  Networking         [ ] 7.1  [ ] 7.2  [ ] 7.3  [ ] 7.4  [ ] 7.5  [ ] 7.6  [ ] 7.7
Phase 8:  Performance        [ ] 8.1  [ ] 8.2  [ ] 8.3  [ ] 8.4  [ ] 8.5  [ ] 8.6
Phase 9:  System Design      [ ] 9.1  [ ] 9.2  [ ] 9.3  [ ] 9.4  [ ] 9.5  [ ] 9.6
Phase 10: Advanced Rust      [ ] 10.1 [ ] 10.2 [ ] 10.3 [ ] 10.4 [ ] 10.5 [ ] 10.6 [ ] 10.7 [ ] 10.8
Phase 11: Accounts DB        [ ] 11.1 [ ] 11.2 [ ] 11.3 [ ] 11.4 [ ] 11.5 [ ] 11.6
Phase 12: Fee & Scheduling   [ ] 12.1 [ ] 12.2 [ ] 12.3 [ ] 12.4 [ ] 12.5
Phase 13: Program Runtime    [ ] 13.1 [ ] 13.2 [ ] 13.3 [ ] 13.4 [ ] 13.5
Phase 14: RPC & Data         [ ] 14.1 [ ] 14.2 [ ] 14.3 [ ] 14.4 [ ] 14.5 [ ] 14.6
Phase 15: DeFi Internals     [ ] 15.1 [ ] 15.2 [ ] 15.3 [ ] 15.4 [ ] 15.5
Phase 16: Testing & Debug    [ ] 16.1 [ ] 16.2 [ ] 16.3 [ ] 16.4 [ ] 16.5
Phase 17: Production Ops     [ ] 17.1 [ ] 17.2
```

</details>

---

## What Senior Infra Interviewers Look For

1. **Ownership semantics** — Do you fight the borrow checker or work with it? Can you explain *why* a lifetime is needed?
2. **Concurrency correctness** — Can you reason about data races, deadlocks, and memory ordering? Do you know when `Arc<Mutex<T>>` is fine vs when you need lock-free structures?
3. **Performance awareness** — Do you know the cost of allocation, cache misses, and system calls? Can you profile and optimize with data?
4. **Error handling** — Do you use `unwrap()` everywhere or do you propagate errors meaningfully with context?
5. **Unsafe reasoning** — Can you explain why a piece of unsafe code is sound? What invariants must hold?
6. **Systems thinking** — Can you reason about latency, throughput, backpressure, and failure modes at the system level?
7. **Blockchain depth** — Do you understand the execution layer, consensus, networking, and state management at the protocol level?
8. **Trade-off articulation** — Can you explain WHY you chose approach A over B, including what you're giving up?

---

## Tips

- **Start with `rust-concepts/`.** Even if you know some Rust, the concept notes build the mental models interviewers test directly.
- **Don't skip phases 1–3.** The fundamentals in phases 1–3 are what interviewers actually test. Blockchain-specific questions come *after* they've verified your Rust skills.
- **Time yourself.** Interview pressure is real. Practice under constraints.
- **Read the tests first.** The tests ARE the spec. Understand what's expected before coding.
- **Run clippy on everything.** `cargo clippy -- -W clippy::all` — interviewers notice when your code is clean.
- **Add benchmarks where indicated.** Some challenges ask for criterion benchmarks. Don't skip these — performance awareness is a core evaluation criterion.
- **For system design:** Practice speaking your answer out loud. Record yourself. Draw diagrams. The ability to articulate trade-offs clearly is what separates senior from mid-level.
- **When stuck:** Think for 15 more minutes, THEN look at references. The struggle is where learning happens.
- **After completing all challenges:** Pick 5 random ones and redo them with a 45-minute timer. That's your interview simulation.

---
