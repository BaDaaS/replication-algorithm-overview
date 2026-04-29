# 0126: Internet Computer

## Historical context

The Internet Computer (ICP) launched mainnet in May 2021,
developed by the DFINITY Foundation (founder Dominic Williams).
ICP's consensus stack is built around *threshold relay*:

- *Threshold-BLS random beacon.* Validators run a threshold
  BLS signature scheme to produce unbiasable random values.
- *Random committee selection.* Each round selects a small
  random committee for block proposals.
- *Notarisation + finalisation.* Blocks are first
  *notarised* (validators agree they are valid candidates)
  then *finalised* (committed irreversibly).

ICP is a *subnet*-based architecture: many subnets run in
parallel, each with its own consensus committee. Subnets
communicate via certified messages.

## System and threat model

- **Network.** Bounded-delay PSS.
- **Failures.** Byzantine `f < n/3` per subnet.
- **Cryptography.** Threshold BLS signatures, NIZK distributed
  key generation.
- **Goal.** Production-grade BFT for cloud-style applications.

## Theory

### Threshold-BLS beacon

ICP runs a threshold BLS scheme: validators jointly produce
random values such that no minority subset can bias the output.
The beacon drives:

- Block-proposal lottery.
- Committee selection.
- Application-level random seeds.

### Notarisation and finalisation

Each block is processed in two stages:

1. *Notarisation.* `>= f + 1` validators sign that the block is
   a valid candidate. There may be multiple notarised blocks
   per height (during forks).
2. *Finalisation.* `2f + 1` validators sign that they consider
   one specific notarised block canonical. Once finalised, no
   competing block can finalise.

### Subnets

Subnets are independent BFT chains, each running its own
threshold-BLS beacon and consensus committee. Cross-subnet
calls use *certified responses*: a subnet's threshold-BLS
signature proves the claim is finalised on that subnet.

### Comparison: ICP vs Cosmos vs Polkadot

| property             | ICP             | Cosmos        | Polkadot       |
| -------------------- | --------------- | ------------- | -------------- |
| structure            | subnets         | zones         | parachains     |
| inter-chain          | certified calls | IBC           | XCM            |
| consensus per subnet | Threshold BFT   | CometBFT      | BABE+GRANDPA   |
| block time           | 1-3 sec         | 6 sec         | 6 sec          |
| smart contracts      | Wasm canisters  | CosmWasm/EVM  | Substrate      |
| year                 | 2021            | 2019          | 2020           |

### Properties

- *Threshold-BLS beacon* (unbiasable randomness).
- *Subnet scalability* (horizontal).
- *Cloud-style apps* (Wasm canisters with state, network
  access).
- *Certified messages* for inter-subnet.

### Limitations

- *Subnet validator selection.* DFINITY-controlled NNS (Network
  Nervous System) governance.
- *Threshold-BLS DKG.* Distributed key generation is complex
  and expensive.
- *Application semantics.* Wasm canisters are powerful but
  novel; ecosystem still maturing.

### Subsequent work

- *Verifiable canisters.* SEV/SGX-style attestation.
- *NNS governance.* On-chain protocol upgrades.
- *Chain Fusion.* ICP integration with Bitcoin and Ethereum.

## Practice

- *Internet Computer mainnet.* Production since May 2021.
- ~30 subnets, ~13 validators per subnet.
- Block time: 1-3 seconds.
- Throughput: thousands of tx/s per subnet.

### Production-implementation notes

- *NNS subnet.* Governance subnet with the most validators.
- *Application subnets.* User-deployed canister hosting.
- *Threshold ECDSA.* Recently added for cross-chain (Bitcoin).

## Verifiability and circuit encoding

**tag: `friendly`.**

Threshold BLS is highly SNARK-friendly. ICP's certified
responses are essentially BLS aggregate signatures on a
state root, easily verifiable in `O(1)` constraints in a
SNARK light client.

## Known attacks and limitations

- *NNS governance attacks.* If DFINITY/NNS is compromised,
  subnet membership can be manipulated.
- *Subnet validator collusion.* `> 1/3` per subnet breaks
  safety.
- *DKG abort.* Adversary can cause DKG to fail; recovery
  protocol needed.

## References

- DFINITY, "Internet Computer Consensus", 2021.
- Hanke, Movahedi, Williams, "Threshold Relay", 2018.
- Camenisch, Drijvers, Hanke, Pignolet, Williams,
  Shoup, "Internet Computer Consensus", 2022.

## Implementation notes

The crate provides a `Notarised` and `Finalised` set tracking
status of blocks per height. Tests verify the status
transitions.

See also [`HISTORY.md`](../HISTORY.md), section "2017 to 2020".
