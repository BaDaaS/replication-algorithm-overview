# Module 0037 Solutions

## Solution 1 [T]: external consistency

Suppose `T_1` commits at `t_1` and acknowledges to client.
By commit-wait, `TT.after(t_1) = true` at the coordinator,
i.e. `t_1 < TT.earliest`. Hence `t_1 < real-time-clock` at
all observers (TrueTime soundness).

`T_2` starts after `T_1`'s ack, so its real-time start is
after `t_1`. By commit-wait, `t_2 > TT.latest` at `T_2`'s
coordinator. By TrueTime soundness, `TT.latest >=
real-time-clock`. So `t_2 > t_1`.

External consistency: real-time order T_1 -> T_2 implies
timestamp order t_1 < t_2.

## Solution 2 [P]: epsilon tuning

| epsilon | commit-wait | use case                |
| ------- | ----------- | ----------------------- |
| 1ms     | 1ms         | atomic clock + GPS      |
| 7ms     | 7ms         | Google's TrueTime       |
| 50ms    | 50ms        | NTP-bounded systems     |
| 500ms   | 500ms       | uncalibrated clocks     |

The 7ms is Google's calibrated production value. NTP-only
systems pay 50-100ms. The trade-off: clock-quality
investment vs per-commit latency.

CockroachDB defaults to 250ms uncertainty as a safe upper
bound for NTP-tuned clocks.

## Solution 3 [F]: pseudo-Lean TrueTime

```text
class TrueTime where
  now      : Time × Time  -- (earliest, latest)
  after    : Time -> Bool
  before   : Time -> Bool
  soundness :
    forall (t : Time),
    after t = true -> t < real_clock /\
    before t = true -> t > real_clock

theorem spanner_external_consistency
    [TT : TrueTime] :
    forall (T_1 T_2 : Transaction),
    Committed T_1 t_1 -> StartedAfter T_2 T_1 ->
    Committed T_2 t_2 -> t_1 < t_2 := by
  sorry
```

## Solution 4 [V]: VDF-based timestamping

A VDF takes time `T` to compute; the output is unbiasable.
Use as a verifiable timestamp source:

- Each block's "timestamp" is a VDF evaluation on the
  previous block's hash.
- The VDF proof attests to the elapsed time.
- The verifier checks the VDF proof and the timestamp
  ordering.

Per commit:

- VDF evaluation: T = ~30s real time (typical for Filecoin-
  style VDFs).
- VDF proof: ~200k constraints.
- BLS quorum cert: ~10^6 constraints.

Total: ~1.2 * 10^6 per commit. The VDF dominates real-time
latency but adds modest constraint cost.

Production: Filecoin's chain randomness uses VDF; Ethereum's
RANDAO+VDF roadmap is similar.
