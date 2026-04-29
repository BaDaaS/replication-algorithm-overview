# 0077: Mahi-Mahi

## Historical context

Mahi-Mahi (Babel-Sonnino-Spiegelman 2024) is a successor to
Mysticeti optimised for *adversarial* networks: when up to
`f` validators are slow or Byzantine, Mahi-Mahi maintains
sub-second commit latency. Mysticeti's fast path degrades
under adversarial scheduling; Mahi-Mahi tolerates this.

## System and threat model

Same as Mysticeti.

## Theory

The key idea: replace Mysticeti's leader-vote with a
*multi-leader* commit rule. Multiple leaders propose per
round; the consensus rule ranks them and commits the highest-
support anchor.

### How Mahi-Mahi differs from Mysticeti

| property              | Mysticeti      | Mahi-Mahi       |
| --------------------- | -------------- | --------------- |
| leaders per round     | 1              | several          |
| latency under attack  | degrades       | stable          |
| typical latency       | ~390ms (Sui)   | ~600ms          |
| robustness            | high           | very high       |

Mahi-Mahi sacrifices a bit of best-case latency to gain
robustness; in adversarial regions, it still commits in
sub-second.

## Practice

- Sui roadmap mentions Mahi-Mahi as a future upgrade.
- Reference design as of 2025.

## Verifiability

**tag: `friendly`.** Multi-anchor verification adds modest
constraint cost.

## References

- Babel, Sonnino, Spiegelman, "Mahi-Mahi", 2024.

See also [`HISTORY.md`](../HISTORY.md), section "2024 to
2026".
