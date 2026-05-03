# Quad Chart

| Quadrant | Content |
| --- | --- |
| Objective | Support resilient leader-follower UAS teaming that reduces operator workload while preserving formation coherence, target handoff, and continuity under degraded GPS or communications. |
| Technical Challenge / Key Innovation | The challenge is to evaluate autonomy behavior without replacing the autonomy stack. The innovation is a property-based Rust monitor with deterministic replay, structured evidence, and an integrable C ABI shaped to this topic’s specific mission checks. |
| Approach | Build and test a topic-specific property set, generate deterministic scenarios, export scorecards/traces/proof logs, and package the entire capability as a reviewer-verifiable software prototype aligned to the topic’s evaluation posture. |
| Expected Impact / Transition | Reduce risk for autonomy adoption by adding a trust layer that can feed recovery logic, operator review, and next-phase integration on representative platforms. |
