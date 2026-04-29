# Module Index

Complete list of all 136 modules in the course, organised by part.
Each entry links to the module's `README.md`.

## Front matter

- [`0000-introduction-smr/`](0000-introduction-smr/README.md)
  Introduction to state-machine replication.

## Part I. Foundations and impossibilities

- [`0001-system-models/`](0001-system-models/README.md) timing models
  (sync, async, partial sync).
- [`0002-failure-models/`](0002-failure-models/README.md) crash,
  omission, Byzantine, mobile failures.
- [`0003-two-generals/`](0003-two-generals/README.md) Two Generals
  paradox.
- [`0004-byzantine-generals/`](0004-byzantine-generals/README.md)
  Lamport-Shostak-Pease 1982.
- [`0005-flp-impossibility/`](0005-flp-impossibility/README.md)
  Fischer-Lynch-Paterson 1985.
- [`0006-dolev-strong/`](0006-dolev-strong/README.md) Dolev-Strong
  authenticated synchronous BFT.
- [`0007-dolev-reischuk/`](0007-dolev-reischuk/README.md) message-
  complexity lower bound.
- [`0008-cap-pacelc/`](0008-cap-pacelc/README.md) CAP and PACELC
  trade-offs.
- [`0009-broadcasts/`](0009-broadcasts/README.md) reliable, atomic,
  causal broadcasts.
- [`0010-quorum-systems/`](0010-quorum-systems/README.md) quorum
  intersection theory.
- [`0011-crypto-prereqs/`](0011-crypto-prereqs/README.md) hashes,
  signatures, threshold-BLS, VRF.

## Part II. Failure detectors and randomised consensus

- [`0012-failure-detectors/`](0012-failure-detectors/README.md)
  Chandra-Toueg failure detector hierarchy.
- [`0013-ben-or/`](0013-ben-or/README.md) Ben-Or randomised
  consensus.
- [`0014-rabin/`](0014-rabin/README.md) Rabin's randomised BA.
- [`0015-bracha-aba/`](0015-bracha-aba/README.md) Bracha asynchronous
  BA.
- [`0016-cks-ckps/`](0016-cks-ckps/README.md) Cachin-Kursawe-Shoup
  asynchronous BFT.
- [`0017-mmr/`](0017-mmr/README.md) Mostefaoui-Moumen-Raynal.
- [`0018-common-coins/`](0018-common-coins/README.md) common-coin
  protocols.

## Part III. Crash-fault state-machine replication

- [`0019-2pc-3pc/`](0019-2pc-3pc/README.md) two- and three-phase
  commit.
- [`0020-viewstamped-replication/`](0020-viewstamped-replication/README.md)
  Oki-Liskov 1988.
- [`0021-vr-revisited/`](0021-vr-revisited/README.md) Liskov-Cowling
  2012.
- [`0022-paxos-synod/`](0022-paxos-synod/README.md) Paxos synod
  protocol.
- [`0023-multi-paxos/`](0023-multi-paxos/README.md) Multi-Paxos.
- [`0024-cheap-paxos/`](0024-cheap-paxos/README.md) Cheap Paxos.
- [`0025-fast-paxos/`](0025-fast-paxos/README.md) Fast Paxos.
- [`0026-generalized-paxos/`](0026-generalized-paxos/README.md)
  Generalized Paxos.
- [`0027-epaxos/`](0027-epaxos/README.md) EPaxos.
- [`0028-mencius/`](0028-mencius/README.md) Mencius.
- [`0029-stoppable-vertical-paxos/`](0029-stoppable-vertical-paxos/README.md)
  Vertical and Stoppable Paxos.
- [`0030-disk-paxos/`](0030-disk-paxos/README.md) Disk Paxos.
- [`0031-flexible-paxos/`](0031-flexible-paxos/README.md) Flexible
  Paxos.
- [`0032-compartmentalized-paxos/`](0032-compartmentalized-paxos/README.md)
  Compartmentalized Paxos.
- [`0033-raft/`](0033-raft/README.md) Raft consensus.
- [`0034-zab/`](0034-zab/README.md) ZooKeeper Atomic Broadcast.
- [`0035-chain-replication/`](0035-chain-replication/README.md)
  van Renesse-Schneider.
- [`0036-craq/`](0036-craq/README.md) CRAQ.
- [`0037-spanner-truetime/`](0037-spanner-truetime/README.md)
  Spanner / TrueTime.
- [`0038-caspaxos/`](0038-caspaxos/README.md) CASPaxos.
- [`0039-atlas/`](0039-atlas/README.md) Atlas.

## Part IV. Classical Byzantine fault tolerance

- [`0040-rampart/`](0040-rampart/README.md) Rampart.
- [`0041-securering/`](0041-securering/README.md) SecureRing.
- [`0042-pbft/`](0042-pbft/README.md) Practical Byzantine Fault
  Tolerance (Castro-Liskov 1999).
- [`0043-qu/`](0043-qu/README.md) Q/U.
- [`0044-hq/`](0044-hq/README.md) HQ.
- [`0045-zyzzyva/`](0045-zyzzyva/README.md) Zyzzyva.
- [`0046-aardvark/`](0046-aardvark/README.md) Aardvark.
- [`0047-upright/`](0047-upright/README.md) UpRight.
- [`0048-spinning/`](0048-spinning/README.md) Spinning.
- [`0049-prime/`](0049-prime/README.md) Prime.
- [`0050-steward/`](0050-steward/README.md) Steward.
- [`0051-a2m-trinc/`](0051-a2m-trinc/README.md) A2M / TrInc.
- [`0052-minbft/`](0052-minbft/README.md) MinBFT.
- [`0053-bft2f/`](0053-bft2f/README.md) BFT2F.
- [`0054-bft-smart/`](0054-bft-smart/README.md) BFT-SMaRt.

## Part V. Streamlined and pipelined BFT

- [`0055-tendermint/`](0055-tendermint/README.md) Tendermint.
- [`0056-hotstuff/`](0056-hotstuff/README.md) HotStuff (Yin et al.
  2018).
- [`0057-librabft-diembft/`](0057-librabft-diembft/README.md)
  LibraBFT / DiemBFT.
- [`0058-jolteon/`](0058-jolteon/README.md) Jolteon.
- [`0059-hotstuff-2/`](0059-hotstuff-2/README.md) HotStuff-2
  (Malkhi-Nayak 2023).
- [`0060-streamlet/`](0060-streamlet/README.md) Streamlet.
- [`0061-pala-pili/`](0061-pala-pili/README.md) Pala / Pili.
- [`0062-sync-hotstuff/`](0062-sync-hotstuff/README.md) Sync HotStuff.
- [`0063-sbft/`](0063-sbft/README.md) SBFT.
- [`0064-ditto/`](0064-ditto/README.md) Ditto.

## Part VI. Asynchronous BFT

- [`0065-honeybadger/`](0065-honeybadger/README.md) HoneyBadger BFT.
- [`0066-beat/`](0066-beat/README.md) BEAT.
- [`0067-dumbo/`](0067-dumbo/README.md) Dumbo.
- [`0068-speeding-dumbo/`](0068-speeding-dumbo/README.md) Speeding
  Dumbo.

## Part VII. DAG-based BFT

- [`0069-hashgraph/`](0069-hashgraph/README.md) Hashgraph.
- [`0070-aleph/`](0070-aleph/README.md) Aleph.
- [`0071-dag-rider/`](0071-dag-rider/README.md) DAG-Rider.
- [`0072-narwhal-tusk/`](0072-narwhal-tusk/README.md) Narwhal / Tusk.
- [`0073-bullshark/`](0073-bullshark/README.md) Bullshark.
- [`0074-cordial-miners/`](0074-cordial-miners/README.md) Cordial
  Miners.
- [`0075-shoal/`](0075-shoal/README.md) Shoal.
- [`0076-mysticeti/`](0076-mysticeti/README.md) Mysticeti.
- [`0077-mahi-mahi/`](0077-mahi-mahi/README.md) Mahi-Mahi.
- [`0078-sailfish/`](0078-sailfish/README.md) Sailfish.
- [`0079-shoal-plus-plus/`](0079-shoal-plus-plus/README.md) Shoal++.
- [`0080-autobahn/`](0080-autobahn/README.md) Autobahn.
- [`0081-bbca-chain/`](0081-bbca-chain/README.md) BBCA-chain.
- [`0082-motorway/`](0082-motorway/README.md) Motorway.

## Part VIII. Nakamoto-style and proof-of-work

- [`0083-bitcoin/`](0083-bitcoin/README.md) Bitcoin / Nakamoto
  consensus.
- [`0084-bitcoin-backbone/`](0084-bitcoin-backbone/README.md)
  Bitcoin Backbone Protocol.
- [`0085-pass-seeman-shelat/`](0085-pass-seeman-shelat/README.md)
  PSS asynchronous-network analysis.
- [`0086-ghost/`](0086-ghost/README.md) GHOST.
- [`0087-selfish-mining/`](0087-selfish-mining/README.md) selfish
  mining (Eyal-Sirer).
- [`0088-bitcoin-ng/`](0088-bitcoin-ng/README.md) Bitcoin-NG.
- [`0089-sleepy/`](0089-sleepy/README.md) sleepy model of consensus.
- [`0090-byzcoin/`](0090-byzcoin/README.md) ByzCoin.
- [`0091-solida/`](0091-solida/README.md) Solida.
- [`0092-hybrid-consensus/`](0092-hybrid-consensus/README.md)
  Hybrid Consensus.
- [`0093-fruitchains/`](0093-fruitchains/README.md) FruitChains.
- [`0094-spectre/`](0094-spectre/README.md) SPECTRE.
- [`0095-thunderella/`](0095-thunderella/README.md) Thunderella.
- [`0096-phantom/`](0096-phantom/README.md) PHANTOM / GHOSTDAG.
- [`0097-prism/`](0097-prism/README.md) Prism.
- [`0098-ohie/`](0098-ohie/README.md) OHIE.
- [`0099-conflux/`](0099-conflux/README.md) Conflux.

## Part IX. Federated Byzantine agreement and Avalanche

- [`0100-stellar-scp/`](0100-stellar-scp/README.md) Stellar SCP.
- [`0101-ripple/`](0101-ripple/README.md) Ripple RPCA.
- [`0102-avalanche/`](0102-avalanche/README.md) Avalanche / Snow*
  family.
- [`0103-snowman/`](0103-snowman/README.md) Snowman.
- [`0104-frosty/`](0104-frosty/README.md) Frosty.

## Part X. Proof of stake: foundations and gadgets

- [`0105-slasher/`](0105-slasher/README.md) Slasher (Buterin 2014).
- [`0106-casper-ffg/`](0106-casper-ffg/README.md) Casper FFG.
- [`0107-casper-cbc/`](0107-casper-cbc/README.md) Casper CBC.
- [`0108-algorand/`](0108-algorand/README.md) Algorand BA*.
- [`0109-snow-white/`](0109-snow-white/README.md) Snow White.

## Part XI. Ouroboros family

- [`0110-ouroboros-classic/`](0110-ouroboros-classic/README.md)
  Ouroboros Classic.
- [`0111-ouroboros-praos/`](0111-ouroboros-praos/README.md)
  Ouroboros Praos.
- [`0112-ouroboros-genesis/`](0112-ouroboros-genesis/README.md)
  Ouroboros Genesis.
- [`0113-ouroboros-crypsinous/`](0113-ouroboros-crypsinous/README.md)
  Ouroboros Crypsinous.
- [`0114-ouroboros-chronos/`](0114-ouroboros-chronos/README.md)
  Ouroboros Chronos.
- [`0115-ouroboros-leios/`](0115-ouroboros-leios/README.md)
  Ouroboros Leios.
- [`0116-ouroboros-peras/`](0116-ouroboros-peras/README.md)
  Ouroboros Peras.
- [`0117-mina-samasika/`](0117-mina-samasika/README.md) Mina
  Samasika.

## Part XII. Production blockchain consensus

- [`0118-gasper/`](0118-gasper/README.md) Ethereum Gasper
  (LMD-GHOST + Casper FFG).
- [`0119-cosmos-cometbft/`](0119-cosmos-cometbft/README.md) Cosmos
  / CometBFT.
- [`0120-polkadot/`](0120-polkadot/README.md) Polkadot
  (BABE + GRANDPA + BEEFY).
- [`0121-solana/`](0121-solana/README.md) Solana (PoH + Tower BFT).
- [`0122-aptos/`](0122-aptos/README.md) Aptos AptosBFT + Block-STM.
- [`0123-sui/`](0123-sui/README.md) Sui Lutris + Mysticeti.
- [`0124-hedera/`](0124-hedera/README.md) Hedera Hashgraph in
  production.
- [`0125-near/`](0125-near/README.md) NEAR Doomslug + Nightshade.
- [`0126-internet-computer/`](0126-internet-computer/README.md)
  Internet Computer ICP consensus.
- [`0127-filecoin/`](0127-filecoin/README.md) Filecoin Expected
  Consensus + F3.
- [`0128-hyperledger-fabric/`](0128-hyperledger-fabric/README.md)
  Hyperledger Fabric.
- [`0129-tezos/`](0129-tezos/README.md) Tezos LPoS + Tenderbake.

## Part XIII. Verifiable replication and SNARK-friendly consensus

- [`0130-zkbridge/`](0130-zkbridge/README.md) zkBridge succinct
  cross-chain proofs.
- [`0131-aleo-snarkos/`](0131-aleo-snarkos/README.md) Aleo /
  snarkOS.
- [`0132-zk-rollup-sequencers/`](0132-zk-rollup-sequencers/README.md)
  zk-rollup sequencer consensus.
- [`0133-threshold-crypto-in-circuit/`](0133-threshold-crypto-in-circuit/README.md)
  threshold cryptography in SNARK circuits.

## Part XIV. Capstones, meta-theory, and open problems

- [`0134-modular-blockchain/`](0134-modular-blockchain/README.md)
  modular blockchain meta-theory.
- [`0135-open-problems/`](0135-open-problems/README.md) open
  problems and meta-theory capstone.

## Verifiability tag legend

- `na`: not applicable (e.g., crash-fault SMR with no on-chain
  verifiability story).
- `friendly`: SNARK-friendly primitives; circuit cost moderate.
- `partial`: encoded in some form; significant constraint count.
- `deployed`: production system with succinct verification.

## Module count by part

| Part  | Range         | Count |
| ----- | ------------- | ----- |
| Front | 0000          | 1     |
| I     | 0001-0011     | 11    |
| II    | 0012-0018     | 7     |
| III   | 0019-0039     | 21    |
| IV    | 0040-0054     | 15    |
| V     | 0055-0064     | 10    |
| VI    | 0065-0068     | 4     |
| VII   | 0069-0082     | 14    |
| VIII  | 0083-0099     | 17    |
| IX    | 0100-0104     | 5     |
| X     | 0105-0109     | 5     |
| XI    | 0110-0117     | 8     |
| XII   | 0118-0129     | 12    |
| XIII  | 0130-0133     | 4     |
| XIV   | 0134-0135     | 2     |
| Total |               | 136   |
