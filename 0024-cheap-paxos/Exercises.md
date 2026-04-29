# Module 0024 Exercises

## Exercise 1 [T]: cost-savings analysis

For `n = 5, 7, 9` Cheap-Paxos deployments, compute the
hardware-cost saving (number of stable-storage replicas
versus full Paxos).

## Exercise 2 [P]: reconfiguration trigger

Sketch the timeout-based reconfiguration trigger: when the
leader fails to collect `f + 1` accepteds within `T` ticks, it
runs a configuration-change Synod to swap an auxiliary in for
the suspected-failed main acceptor.

## Exercise 3 [F]: two-level safety

State and prove the two-level safety theorem: data SMR safety
+ configuration SMR safety together imply Cheap Paxos safety.

## Exercise 4 [V]: configuration as public input

For a verifiable Cheap Paxos, the configuration is a public
input to the verifier. Discuss how the verifier checks that
the proven decision was made under a valid configuration.
