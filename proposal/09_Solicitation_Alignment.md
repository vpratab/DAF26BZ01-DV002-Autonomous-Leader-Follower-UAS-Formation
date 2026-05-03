# Solicitation Alignment

## Topic Basis

- **Track posture:** Direct-to-Phase-II / pre-release topic (confirm final DSIP release details at submission time)
- **Source basis:** Public pre-release mirrors for DAF26BZ01-DV002; validate final wording against the official DSIP topic PDF before submission.
- **Objective summary:** Support resilient leader-follower UAS teaming that reduces operator workload while preserving formation coherence, target handoff, and continuity under degraded GPS or communications.

## What This Repository Intentionally Covers

- leader-follower geometry, separation, and intent coherence
- degraded GPS / PNT resilience inside formation logic
- control or task handoff validity during pilot reassignment or target designation changes
- safe reversion, stasis, or terminal-action integrity when the formation chain degrades

## How The Repository Maps To The Topic

| Solicitation Need | Repository Response |
| --- | --- |
| Topic-specific runtime checks | `core/src/profile.rs` encodes five topic-shaped trust properties tied to this mission area. |
| Repeatable proof and replay | `tooling/replay`, `tooling/eval`, `evidence/`, and `package_manifest.json` provide deterministic reproduction. |
| Integration path | `bindings/include/rt_vlas.h` and `bindings/src/lib.rs` define the C ABI boundary for autonomy-stack insertion. |
| Reviewer-verifiable evidence | `evidence/scorecard_summary.md`, `proof_log.txt`, `timeline.json`, and `trace.svg` make the behavior inspectable. |
| Clear scope discipline | This repository is scoped as: This repository focuses on runtime integrity and supervisory checks around leader-follower behaviors rather than replacing the formation flight controller itself. |

## What The Package Is Not Claiming

- it is not a replacement for the underlying autonomy stack
- it is not a certification package
- it is not based on classified program data
- it is not claiming operational fielding approval

## Why The Current Shape Is Credible

The strongest near-term value of RTVLAS is the ability to make autonomy behavior observable,
explainable, and rejectable when it drifts outside mission or safety expectations. That is the
thread this repository follows for this specific topic.
