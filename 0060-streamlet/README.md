# 0060: Streamlet

## Historical context

Chan, Pass, Shi published "Streamlet: Textbook Streamlined
Blockchains" at AFT 2020. Streamlet is the *teaching*
streamlined BFT: two-phase commit with explicit notarisation
and finality rules. Designed to be simple to teach and prove
correct.

## System and threat model

Partial synchrony, `f < n / 3`.

## Theory

Each block has a leader (round-robin). A block is
*notarised* on `2/3` votes. A block is *finalised* when
three consecutive notarised blocks form a chain.

The simplicity makes Streamlet a popular teaching example.

## Practice

Pedagogical only; not deployed in production. Aptos's Pala
(module 0061) is a related successor.

## Verifiability

**Tag: `friendly`.** Standard ~10^6 per block.

## References

- Chan, Pass, Shi, "Streamlet: Textbook Streamlined
  Blockchains", AFT 2020.

See also [`HISTORY.md`](../HISTORY.md), section "2020 to 2023".
