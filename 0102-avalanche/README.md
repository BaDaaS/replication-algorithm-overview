# 0102: Avalanche / Snow Family

## Historical context

The "Avalanche" family was introduced in 2018 by an anonymous
group calling themselves *Team Rocket* (later identified as
Yin Sirer's lab at Cornell, with co-authors Maofan Yin, Kevin
Sekniqi, Robbert van Renesse, and Emin Gun Sirer). The paper
"Snowflake to Avalanche: A Novel Metastable Consensus Protocol
Family" (IPFS-distributed white paper 2018, peer-reviewed
2020) introduces a probabilistic gossip-based consensus
family.

The family consists of four protocols, each building on the
previous:

1. *Slush.* A naive sample-and-update protocol (terminates with
   constant probability).
2. *Snowflake.* Adds a counter for stable confirmation.
3. *Snowball.* Adds memory for hysteresis.
4. *Avalanche.* DAG-based version of Snowball.

Avalanche underpins the Avalanche Network (AVAX) blockchain,
launched mainnet 2020.

## System and threat model

- **Network.** Asynchronous (no timing assumption for safety).
- **Failures.** Byzantine, `f < n/3` (or weaker for Avalanche;
  exact threshold depends on parameters).
- **Cryptography.** Standard signatures.
- **Goal.** Highly scalable consensus via gossip-based
  probabilistic voting.

## Theory

### Slush (basic protocol)

Each round, a node:

1. Sample `k` random other nodes.
2. Ask them their current preference.
3. If `>= alpha * k` agree on a value `v`, adopt `v`.

Slush converges with constant probability; Snowflake adds a
counter for finality.

### Snowflake

Adds a counter `cnt`. Each successful query (i.e., `>= alpha * k`
agreeing) increments `cnt`. After `cnt >= beta` consecutive
successes, the value is *finalised*.

### Snowball

Adds *confidence counters* per value. Each successful query
records a confidence point for the chosen value. Switches
preference only when another value's confidence exceeds the
current one (hysteresis).

### Avalanche (DAG variant)

Snowball extended to a DAG of transactions. Each transaction
references parents; queries return a transaction's preferred
ancestor set. The DAG structure lets the protocol process many
transactions in parallel.

### Theorem (Team Rocket 2018, informal)

Under asynchronous network with `f < n/3` Byzantine, Snowflake/
Snowball/Avalanche reach probabilistic consensus with
exponentially small error in the parameters `k, alpha, beta`.
Specifically: probability of disagreement decays as
`exp(-Theta(beta))`.

### Comparison: Avalanche vs PBFT vs PoW

| property            | Avalanche        | PBFT          | PoW Bitcoin   |
| ------------------- | ---------------- | ------------- | ------------- |
| network model       | async            | partial sync  | bounded delay |
| finality            | probabilistic    | deterministic | probabilistic |
| confirmation time   | seconds          | seconds       | minutes-hours |
| scalability         | thousands+       | hundreds      | thousands+    |
| communication       | gossip O(k * log n) | all-to-all O(n^2) | broadcast |
| messages per commit | ~k * log n       | O(n^2)        | O(n)          |
| open membership     | yes (PoS)        | no            | yes (PoW)     |

### Properties

- *Probabilistic finality.* Confirmation depth `beta` for
  user-chosen confidence.
- *Async safety.* No timing assumption.
- *Sub-linear communication.* Each round samples `k` peers,
  not all `n`.
- *Highly scalable.* Throughput thousands of tx/s on tested
  networks.

### Limitations

- *Probabilistic finality only.* Not deterministic; small but
  non-zero error.
- *Liveness under partition.* Asynchronous safety is preserved,
  but liveness halts during long partitions.
- *Parameter tuning.* Performance depends on `k, alpha, beta`
  tuning.

### Subsequent work

- *Snowman.* Linear-chain version of Avalanche, used for
  Avalanche Subnets.
- *Frosty.* Avalanche-style protocol with formal security
  bounds (Yin Sirer 2024).
- *Avalanche Subnets.* Multiple parallel Avalanche chains for
  app-specific use.

## Practice

- *Avalanche Network.* Production since 2020. C-Chain (EVM),
  X-Chain (UTXO), P-Chain (platform).
- ~1000 validators, ~thousands of tx/s, sub-second finality.
- Subnets enable app-specific chains with custom validator sets.

### Production-implementation notes

- *Sample size `k`.* ~20 in production; trade-off between
  query cost and convergence rate.
- *Alpha (quorum threshold).* Typically `0.6 * k = 12`.
- *Beta (finality counter).* ~15 to 20 successive confirmations.
- *Stake-weighted sampling.* Each query weights by stake to
  resist Sybil attacks.

## Verifiability and circuit encoding

**tag: `partial`.**

Avalanche circuits would encode the gossip-query state machine
plus signature verification. The probabilistic nature of
finality complicates SNARK-based verification: a circuit must
encode "this transaction has received at least `beta` successive
positive queries", which means tracking query histories.

A more practical approach: encode only finalised transactions
(after `beta` confirmations) and rely on out-of-band probabilistic
confidence.

## Known attacks and limitations

- *Sybil attacks on sampling.* Avalanche resists via stake-
  weighting; PoS-style attacks (long-range, posterior
  corruption) apply.
- *Liveness under partition.* Like all async protocols, halts
  during long partitions.
- *Probabilistic finality.* High-value transactions need many
  beta confirmations.

## References

- Team Rocket, "Snowflake to Avalanche: A Novel Metastable
  Consensus Protocol Family for Cryptocurrencies", IPFS 2018.
- Yin, Sekniqi, van Renesse, Sirer, "Snow*: Avalanche-style
  Probabilistic Consensus", Avalanche white paper, 2020.

## Implementation notes

The crate provides a simple Snowball-style preference machine:
each node has a current preference and a confidence counter;
queries are simulated by a deterministic RNG. Tests verify
that repeated queries with consistent answers converge to a
fixed preference.

See also [`HISTORY.md`](../HISTORY.md), section "2017 to 2020".
