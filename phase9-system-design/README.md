# Phase 9: System Design (Written)

No code — write your answers using the [TEMPLATE.md](./TEMPLATE.md) for each challenge. Practice explaining your design out loud as if you're in a 45–60 minute interview.

---

## How to Use This Phase

1. **Pick a challenge** below and read the problem + key areas.
2. **Clarify requirements** (out loud or in writing): scale, latency, consistency, constraints.
3. **Sketch the architecture** on paper or a whiteboard: components, data flow, boundaries.
4. **Fill in TEMPLATE.md** — copy it per challenge (e.g. `09-1-rpc-node.md`, `09-2-jito-mev.md`) or keep one and replace content each time.
5. **Practice speaking** your design: "First I'd add a cache layer because…", "The bottleneck will be…", "I'm trading off X for Y because…".

---

## 9.1 High-Throughput RPC Node

**One-liner:** Design an RPC layer that serves thousands of concurrent `getAccountInfo` / `getTransaction` / `getBlock` requests with low latency and high availability.

**Why it matters:** Helius, Triton, QuickNode, and validator-operated RPC endpoints all need to scale read traffic without overloading the validator or falling behind. Caching and load balancing are table stakes.

**Suggested scale:**
- 50k+ read RPC requests per second aggregate.
- P99 latency < 200 ms for cacheable queries.
- Multiple validator replicas; clients can hit any of several RPC nodes.

**Key areas to cover:**
- **Caching:** What to cache (account data, blocks, transactions). TTL and invalidation (e.g. by slot or commitment). Cache key design (pubkey + commitment?). Where does the cache live (per-node, distributed like Redis)?
- **Load balancing:** How traffic is spread across RPC nodes and across backend validators. Health checks and removing bad backends.
- **Rate limiting:** Per-IP vs per-API-key. Token bucket or sliding window. How to avoid one client starving others.
- **Read path:** Request → LB → RPC node → cache or validator. When to go to the validator vs serve from cache.
- **Failure modes:** Validator lag, cache stampede, network partition. How you detect and degrade (e.g. stale reads, 503s).

**Interview-style prompts:**
- "A client is hammering getAccountInfo for the same account. How do you protect the backend?"
- "How do you know when to invalidate cached account data?"
- "How would you add support for websocket subscriptions without overloading the system?"

---

## 9.2 Jito MEV / Block Engine

**One-liner:** Design the infrastructure that lets searchers submit bundles to block producers and that ensures bundles are executed atomically (all-or-nothing) with minimal latency.

**Why it matters:** Jito-style block engines sit between searchers and leaders. Searchers need fast, reliable bundle submission and clear rules (ordering, tips, inclusion). Leaders need a clean API and protection from abuse.

**Suggested scale:**
- Hundreds of searchers, many bundles per slot.
- Bundle submission to execution decision in single-digit milliseconds.
- Leader changes every slot (~400 ms); the engine must work with the current leader’s schedule.

**Key areas to cover:**
- **Bundle flow:** Searcher → API (auth, validation) → relay/block engine → leader. How bundles are received, validated (signatures, balance, nonce), and forwarded.
- **Leader forwarding:** How the engine knows the current and upcoming leaders. Private vs public channels. Retries and fallbacks if a leader is slow or down.
- **Ordering and inclusion:** How bundles are ordered (e.g. by tip, by arrival). How the leader selects which bundles land in the block. Interaction with normal (non-bundle) transactions.
- **Latency:** Where time is spent (network RTT, serialization, validation, leader processing). How you’d reduce tail latency.
- **Atomicity and rollback:** All-or-nothing execution. What happens if the leader applies the block then the network reorgs.

**Interview-style prompts:**
- "How do you prevent a searcher from submitting a bundle that front-runs others but never pays the tip?"
- "The leader for this slot is in a different datacenter. How does that change your design?"
- "How would you add a 'bundle simulation' API so searchers can dry-run before submitting?"

---

## 9.3 Cross-Chain Bridge

**One-liner:** Design a system that locks or burns assets on chain A and mints or releases them on chain B, with secure verification of state and events across chains.

**Why it matters:** Bridges (e.g. Wormhole, LayerZero) are critical for moving value and messages. Failures are high-impact, so interviewers care about threat model, proof verification, and liveness.

**Suggested scale:**
- Assume Solana as one side; the other chain could be Ethereum or another L1.
- Finality and block times differ; design for asynchronous verification.

**Key areas to cover:**
- **Proof verification:** How chain B learns about an event on chain A (relayers, attestations, light clients, state proofs). What you actually verify (signatures, Merkle proofs, block inclusion).
- **Security:** Who can mint on the destination? Multisig vs MPC vs light client. Attack vectors: fake proofs, reorgs, validator collusion.
- **Liveness:** Relayer failure, chain congestion, finality delays. How users and liquidity are protected when the bridge is slow or halted.
- **Data model:** Lock/mint vs burn/mint. Vaults, attestation logs, replay protection (nonces, processed txs).
- **Operational:** Upgrades, key rotation, pause/unpause. How to handle chain reorgs on either side.

**Interview-style prompts:**
- "An attacker claims they locked 1M USDC on chain A. How does chain B verify that?"
- "The relayers are down for an hour. What can and can’t users do?"
- "How do you handle a reorg on the source chain after attestations were already sent?"

---

## 9.4 Stablecoin Settlement Layer

**One-liner:** Design a layer that settles high-volume stablecoin transfers (e.g. USDC) with clear finality, compliance hooks, and optional batching to reduce cost or latency.

**Why it matters:** Stablecoins are used for payments, CEX flows, and institutional settlement. The system must be auditable, handle blacklists/allowlists, and scale.

**Suggested scale:**
- Large volume of transfers (e.g. 10k+ TPS if batched or off-chain).
- Settlement finality within a defined window (e.g. seconds to minutes).
- Compliance: ability to freeze/block addresses and report large or suspicious flows.

**Key areas to cover:**
- **Batch processing:** How transfers are collected (off-chain, mempool, or RPC), batched, and submitted on-chain. Who batches (operator, relayer, users).
- **Finality:** When is a transfer "final" (e.g. confirmed + N slots, or checkpoint). How you communicate finality to clients and downstream systems.
- **Compliance:** Integration with sanction lists, freeze lists, and optional KYC/AML flags. How you enforce at the RPC or contract layer without leaking private data unnecessarily.
- **Data model:** Ledger of transfers, balances, and freezes. Indexing for "all transfers for address X" or "all transfers above $Y."
- **Failure and recovery:** Operator failure, chain congestion, upgrade path. How you preserve consistency and auditability.

**Interview-style prompts:**
- "Regulators require the ability to freeze an address within 10 minutes. Where does that live in your design?"
- "How do you batch 100k transfers into a single on-chain transaction?"
- "What happens if the chain reorgs after you’ve already told the user their transfer is final?"

---

## 9.5 Validator Monitoring System

**One-liner:** Design a system that collects metrics and logs from many validators, aggregates them, and supports alerting and dashboards so operators can keep the network healthy.

**Why it matters:** Validator and RPC operators need visibility into performance, votes, skip rate, disk, and errors. Good monitoring is the difference between catching issues early and post-mortems.

**Suggested scale:**
- Hundreds to thousands of validators (or RPC nodes); each emits metrics and logs.
- Metrics: vote latency, TPS, disk usage, CPU, memory, slot participation.
- Alerts: down nodes, high skip rate, disk full, consensus lag.

**Key areas to cover:**
- **Metrics collection:** Push vs pull. Agent on each validator (e.g. Prometheus exporter, custom). What metrics you expose (counters, gauges, histograms). Cardinality and sampling.
- **Aggregation and storage:** Where metrics land (Prometheus, Thanos, M3, etc.). Retention and downsampling. Logs (Loki, Elastic, etc.) if needed.
- **Alerting:** Rules (e.g. "skip rate > 5%", "no heartbeat for 2 min"). Alert manager, routing, and escalation. Reducing noise (grouping, inhibition).
- **Dashboards:** Per-validator and fleet-wide views. Key panels: participation, latency, resource usage, errors.
- **Operations:** How you deploy and upgrade the monitoring stack. How validators are discovered and labeled (region, operator, cluster).

**Interview-style prompts:**
- "How do you avoid one validator’s high cardinality (e.g. per-account metrics) blowing up your storage?"
- "You get 1000 alerts when the network has a hiccup. How do you make that actionable?"
- "How would you detect that a validator is voting on the wrong fork?"

---

## 9.6 Transaction Indexer at Scale

**One-liner:** Design a system that ingests every transaction (and optionally every account change) from the chain at 50k+ TPS, then supports low-latency queries (by signature, by account, by program, by time range).

**Why it matters:** Explorers, analytics, and dApps need indexed history. Keeping up with chain throughput while answering complex queries is a classic infra problem.

**Suggested scale:**
- Ingestion: 50k+ TPS sustained; burst higher during congestion.
- Queries: "All txs for address X in the last 24h", "All txs for program Y at slot Z", "Tx by signature." P95 query latency < 500 ms.

**Key areas to cover:**
- **Ingestion pipeline:** How you consume data (validator stream, Geyser plugin, RPC polling). Ordering (by slot, by block). Backpressure and checkpointing so you can resume.
- **Parsing and enrichment:** Decoding instructions, resolving accounts, extracting program IDs. What you store per transaction (full payload vs normalized rows). Deduplication (reorgs).
- **Storage and indexing:** Choice of DB (e.g. ClickHouse, Timescale, Cassandra, or specialized). Primary and secondary indexes (signature, account, program, slot, time). Partitioning and TTL.
- **Query optimization:** How each query type is served. Caching hot queries. Avoiding full scans.
- **Reorg handling:** Detecting reorgs, rolling back or marking stale data, and re-ingesting the new canonical chain. Idempotency and consistency.

**Interview-style prompts:**
- "The chain just had a 10-slot reorg. How does your indexer correct its data?"
- "A user asks for 'all transactions that touched program X in the last 7 days.' How is that query executed?"
- "Ingestion falls behind by 5 minutes during a spike. How do you catch up without dropping data?"

---

## Checklist Before You Call It Done

For each challenge, make sure you’ve:

- [ ] Stated assumptions (scale, latency, consistency).
- [ ] Drawn or described a high-level architecture.
- [ ] Explained the data model and main access patterns.
- [ ] Deep-dived on at least one critical component.
- [ ] Discussed scaling, bottlenecks, and failure modes.
- [ ] Articulated trade-offs ("I optimized for X; I’m giving up Y").
- [ ] Mentioned monitoring and operations.

Use [TEMPLATE.md](./TEMPLATE.md) to structure your written answers, then practice delivering the same content out loud.
