# Evidence Scorecard Summary

- Topic: `DAF26BZ01-DV002 Autonomous Leader-Follower UAS Formation`
- Generated: `2026-05-03T04:54:20Z`
- Git Head: `UNCOMMITTED_SCAFFOLD`
- Scenario Pass Rate: `3/3 (100.0%)`
- Evidence Type: `deterministic synthetic autonomy traces for submission-stage feasibility review`

| Scenario | Mode | Result | Final Trust | First Reject | Scorecard |
| --- | --- | --- | --- | --- | --- |
| Nominal Single-Pilot Formation Leg | `nominal` | `PASS` | `1.000` | `None` | [scenario_01_nominal_formation_leg](scenario_01_nominal_formation_leg/trust_scorecard.json) |
| Contested-PNT Stasis Drift | `degraded` | `PASS` | `0.130` | `None` | [scenario_02_degraded_pnt_spacing](scenario_02_degraded_pnt_spacing/trust_scorecard.json) |
| Follower Divergence After Handoff | `fault` | `PASS` | `0.003` | `20` | [scenario_03_follower_divergence](scenario_03_follower_divergence/trust_scorecard.json) |

## Notes

- Nominal scenarios are expected to remain fully accepted.
- Degraded scenarios are expected to produce concern signals without hard reject behavior.
- Fault scenarios are expected to produce deterministic reject behavior.
- This summary is generated automatically from the underlying per-scenario scorecards.
