
            // SPDX-License-Identifier: Apache-2.0
            //
            // Copyright (c) 2025 RTVLAS contributors

            use rtvlas_core::{default_profile, evaluate_scenario, nominal_snapshot, EvidenceBundle, TrustVerdict, write_evidence_bundle};
            use std::fs;

#[test]
            fn formation_spacing_raises_signal() {
                let profile = default_profile();
                let index = profile
                    .properties
                    .iter()
                    .position(|property| property.key == "formation_spacing")
                    .expect("property present");
                let property = profile.properties[index].clone();
                let mut snapshot = nominal_snapshot();
                snapshot.trust_inputs.formation_spacing_m = 46.5;
snapshot.trust_inputs.desired_spacing_m = 40.0;
                let outcome = property.evaluate(&snapshot);
                assert!(matches!(outcome.verdict, TrustVerdict::Flag | TrustVerdict::Reject));
                assert_eq!(outcome.property_key, "formation_spacing");
            }

#[test]
fn heading_coherence_raises_signal() {
    let profile = default_profile();
    let index = profile
        .properties
        .iter()
        .position(|property| property.key == "heading_coherence")
        .expect("property present");
    let property = profile.properties[index].clone();
    let mut snapshot = nominal_snapshot();
    snapshot.trust_inputs.heading_error_rad = 0.23;
    let outcome = property.evaluate(&snapshot);
    assert!(matches!(outcome.verdict, TrustVerdict::Flag | TrustVerdict::Reject));
    assert_eq!(outcome.property_key, "heading_coherence");
}

#[test]
            fn deconfliction_margin_raises_signal() {
                let profile = default_profile();
                let index = profile
                    .properties
                    .iter()
                    .position(|property| property.key == "deconfliction_margin")
                    .expect("property present");
                let property = profile.properties[index].clone();
                let mut snapshot = nominal_snapshot();
                snapshot.trust_inputs.deconfliction_margin_m = 23.0;
snapshot.trust_inputs.min_deconfliction_margin_m = 25.0;
                let outcome = property.evaluate(&snapshot);
                assert!(matches!(outcome.verdict, TrustVerdict::Flag | TrustVerdict::Reject));
                assert_eq!(outcome.property_key, "deconfliction_margin");
            }

#[test]
fn degraded_pnt_timing_raises_signal() {
    let profile = default_profile();
    let index = profile
        .properties
        .iter()
        .position(|property| property.key == "degraded_pnt_timing")
        .expect("property present");
    let property = profile.properties[index].clone();
    let mut snapshot = nominal_snapshot();
    snapshot.trust_inputs.temporal_skew_ms = 51.0;
    let outcome = property.evaluate(&snapshot);
    assert!(matches!(outcome.verdict, TrustVerdict::Flag | TrustVerdict::Reject));
    assert_eq!(outcome.property_key, "degraded_pnt_timing");
}

#[test]
fn target_handoff_plan_validity_raises_signal() {
    let profile = default_profile();
    let index = profile
        .properties
        .iter()
        .position(|property| property.key == "target_handoff_plan_validity")
        .expect("property present");
    let property = profile.properties[index].clone();
    let mut snapshot = nominal_snapshot();
    snapshot.trust_inputs.mission_plan_valid = false;
    let outcome = property.evaluate(&snapshot);
    assert!(matches!(outcome.verdict, TrustVerdict::Flag | TrustVerdict::Reject));
    assert_eq!(outcome.property_key, "target_handoff_plan_validity");
}

            #[test]
            fn evidence_pipeline_writes_expected_files() {
                let profile = default_profile();
                let scenario_name = "test_scenario";
                let snapshots = vec![nominal_snapshot(), nominal_snapshot()];
                let (timeline, scorecard) = evaluate_scenario(profile, scenario_name, &snapshots);
                let bundle = EvidenceBundle { timeline, scorecard };
                let temp_dir = std::env::temp_dir().join("rtvlas_phase1_evidence");
                let _ = fs::remove_dir_all(&temp_dir);
                fs::create_dir_all(&temp_dir).expect("temp dir");
                let input_log = temp_dir.join("input.jsonl");
                fs::write(
                    &input_log,
                    snapshots
                        .iter()
                        .map(|snapshot| serde_json::to_string(snapshot).expect("json"))
                        .collect::<Vec<_>>()
                        .join("\n"),
                )
                .expect("input log");
                write_evidence_bundle(&temp_dir, &input_log, &snapshots, &bundle).expect("evidence bundle");
                assert!(temp_dir.join("trust_scorecard.json").exists());
                assert!(temp_dir.join("timeline.json").exists());
                assert!(temp_dir.join("proof_log.txt").exists());
                assert!(temp_dir.join("trace.svg").exists());
            }

            #[test]
            fn reject_path_drops_trust() {
                let mut snapshot = nominal_snapshot();
                snapshot.trust_inputs.formation_spacing_m = 61.0;
    snapshot.trust_inputs.desired_spacing_m = 40.0;
                let (timeline, scorecard) = evaluate_scenario(default_profile(), "reject_case", &[snapshot]);
                assert_eq!(timeline.len(), 1);
                assert!(scorecard.final_trust_score < 1.0);
                assert!(scorecard.reject_frames >= 1 || scorecard.flag_frames >= 1);
            }
