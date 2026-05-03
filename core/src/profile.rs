
        // SPDX-License-Identifier: Apache-2.0
        //
        // Copyright (c) 2025 RTVLAS contributors

        use crate::model::{AutonomySnapshot, BoolField, NumericField, PropertyKind, PropertySpec, TrustInputs};
        use crate::monitor::MonitorProfile;

        pub fn default_profile() -> MonitorProfile {
            MonitorProfile {
                topic_id: "DAF26BZ01-DV002".to_string(),
                title: "Autonomous Leader-Follower UAS Formation".to_string(),
                framing: "formation autonomy assurance layer; degraded-PNT trust monitor for multi-UAS control".to_string(),
                properties: vec![
        PropertySpec::new(
            "formation_spacing",
            "Formation Geometry Error",
            "Checks that observed leader-follower spacing remains within the allowable formation envelope for resilient single-pilot operations.",
            PropertyKind::FormationSpacing { tolerance_m: 5.0 },
            1.2,
        ),
        PropertySpec::new(
            "heading_coherence",
            "Formation Intent Coherence",
            "Bounds relative heading divergence so follower actions do not silently desynchronize from leader intent during coordinated maneuvers.",
            PropertyKind::MaxValue { field: NumericField::HeadingErrorRad, max: 0.18 },
            0.9,
        ),
        PropertySpec::new(
            "deconfliction_margin",
            "Terminal Separation Margin",
            "Preserves a minimum collision-avoidance margin even when formation commands begin to diverge toward target or terminal actions.",
            PropertyKind::MinMargin { field: NumericField::DeconflictionMarginM, reference: NumericField::MinDeconflictionMarginM },
            1.1,
        ),
        PropertySpec::new(
            "degraded_pnt_timing",
            "Contested-PNT Timing Coherence",
            "Uses timing skew as a lightweight proxy for degraded navigation confidence inside the formation loop.",
            PropertyKind::MaxValue { field: NumericField::TemporalSkewMs, max: 45.0 },
            0.8,
        ),
        PropertySpec::new(
            "target_handoff_plan_validity",
            "Target Handoff Plan Validity",
            "Detects when pilot reassignment, target designation, or terminal handoff state is no longer valid and should force reversionary logic.",
            PropertyKind::BooleanGate { field: BoolField::MissionPlanValid, reject_on_false: true },
            1.0,
        )
                ],
            }
        }

        pub fn nominal_snapshot() -> AutonomySnapshot {
            AutonomySnapshot {
    timestamp_ms: 0,
    position_m: [0.0, 0.0, 180.0],
    velocity_mps: [22.0, 1.5, 0.0],
    heading_rad: 0.08,
    trust_inputs: TrustInputs {
        gps_valid: true,
        operator_link: true,
        autonomy_solution_feasible: true,
        mission_plan_valid: true,
        emergency_response_ready: true,
        temporal_skew_ms: 12.0,
        corridor_error_m: 8.0,
        corridor_half_width_m: 24.0,
        command_speed_mps: 26.0,
        max_safe_speed_mps: 38.0,
        deconfliction_margin_m: 55.0,
        min_deconfliction_margin_m: 25.0,
        formation_spacing_m: 40.0,
        desired_spacing_m: 40.0,
        heading_error_rad: 0.05,
        threat_distance_m: 76.0,
        threat_min_distance_m: 46.0,
        wez_exposure: 0.18,
        route_efficiency: 0.91,
        decision_latency_ms: 140.0,
        operator_intent_alignment: 0.94,
        evidence_completeness: 0.97,
        hazard_distance_m: 74.0,
        min_hazard_distance_m: 42.0,
        safe_altitude_margin_m: 48.0,
        recovery_zone_distance_m: 920.0,
        max_recovery_zone_distance_m: 1600.0,
        autonomy_solution_optimality: 0.91,
    },
}
        }
