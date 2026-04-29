# Module 0017 Exercises

## Exercise 1 [T]: BV-broadcast properties

Prove the three BV-broadcast properties (validity,
justification, uniformity) from the protocol's amplification
and delivery rules.

## Exercise 2 [T]: MMR termination

Reproduce the MMR termination proof: with a fair common coin,
expected `O(1)` rounds. Identify the role of BV uniformity in
ensuring all honest see the same `bin_values`.

## Exercise 3 [T]: signature-free vs signed

Compare the security assumptions of MMR vs CKS. Identify what
breaks under (i) compromised hash function, (ii) compromised
threshold key, (iii) compromised common-coin source.

## Exercise 4 [P]: BV variants

Implement an alternative BV-broadcast that uses *two* phases
(pre-vote and vote) instead of MMR's amplification rule.
Compare the message counts.

## Exercise 5 [P]: post-quantum readiness

The MMR signature-free design is appealing for post-quantum
deployments. Identify the changes needed to make the common
coin post-quantum: which beacon constructions are PQ-secure?

## Exercise 6 [F]: pseudo-Lean BV

Define `BinaryValueBroadcast` as a Lean typeclass with the
three properties. Prove validity follows from the
amplification rule (assuming `> f` honest contributors).

## Exercise 7 [V]: zk-MMR

Estimate the SNARK constraint count of one MMR round. Compare
to CKS (module 0016): MMR avoids signature verifications but
has more public-input witnesses for echo certificates.

## Exercise 8 [V]: hash-based vs signature-based circuits

Discuss the trade-off between hash-based attestations
(MMR-style) and signature-based attestations (CKS-style) in a
SNARK circuit. Hash-based: cheaper per check, more public
inputs. Signature-based: more expensive per check, fewer
public inputs.
