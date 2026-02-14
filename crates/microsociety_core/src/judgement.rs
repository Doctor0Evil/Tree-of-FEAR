use serde::{Deserialize, Serialize};

use crate::deed::{Deed, DeedKind};
use crate::model::{GlobalConstraints, SiteSnapshot};

/// Per-deed moral evaluation, rule-based and auditable.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Judgment {
    pub harmscore: f64,
    pub opportunity_cost_score: f64,
    pub responsibility_score: f64,
    pub fairness_score: f64,
    pub overall_moral_score: f64,
    pub rationale: String,
}

/// Parameters for the deed–judgement engine.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JudgmentConfig {
    pub high_load_threshold: f64,
    pub vulnerable_fear_band: (f64, f64),
}

fn average_bioload(snapshots: &[SiteSnapshot]) -> f64 {
    if snapshots.is_empty() {
        return 0.0;
    }
    let sum: f64 = snapshots.iter().map(|s| s.bio.bioload).sum();
    sum / snapshots.len() as f64
}

fn average_power(snapshots: &[SiteSnapshot]) -> f64 {
    if snapshots.is_empty() {
        return 0.0;
    }
    let sum: f64 = snapshots.iter().map(|s| s.tokens.power).sum();
    sum / snapshots.len() as f64
}

fn average_church(snapshots: &[SiteSnapshot]) -> f64 {
    if snapshots.is_empty() {
        return 0.0;
    }
    let sum: f64 = snapshots.iter().map(|s| s.tokens.church).sum();
    sum / snapshots.len() as f64
}

/// Compute responsibility weighting from POWER, TECH, and CHURCH (duty-of-care).
fn compute_responsibility(pre: &[SiteSnapshot]) -> f64 {
    if pre.is_empty() {
        return 0.0;
    }
    let mut acc = 0.0;
    for s in pre {
        let p = s.tokens.power;
        let t = s.tokens.tech;
        let c = s.tokens.church;
        let duty = 0.4 * p + 0.3 * t + 0.3 * c;
        acc += duty;
    }
    acc / pre.len() as f64
}

/// Core judgement function: pure, deterministic mapping Deed → Judgment.
pub fn judge_deed(
    deed: &Deed,
    cfg: &JudgmentConfig,
    global: &GlobalConstraints,
) -> Judgment {
    // Context: average bioload before and after.
    let pre_b = average_bioload(&deed.pre);
    let post_b = average_bioload(&deed.post);
    let delta_b = post_b - pre_b;

    // Context: POWER–CHURCH ratio after deed (captures stewardship).
    let post_power = average_power(&deed.post);
    let post_church = average_church(&deed.post);
    let ratio_pc = if post_church > 0.0 {
        post_power / post_church
    } else {
        0.0
    };

    // Harm score: bioload increases in already high-load context are penalized.
    let mut harmscore = 0.0;
    if pre_b >= cfg.high_load_threshold && delta_b > 0.0 {
        harmscore += 1.5 * delta_b;
    } else if delta_b > 0.0 {
        harmscore += delta_b;
    }

    // Extra harm if deed exceeds POWER ≤ k·CHURCH corridor.
    if ratio_pc > global.power_church_k {
        harmscore += 0.5 * (ratio_pc - global.power_church_k);
    }

    // Opportunity cost: failure to use restorative deeds when available.
    let mut opportunity_cost_score = 0.0;
    match deed.kind {
        DeedKind::EmitPollution | DeedKind::Conflict => {
            if pre_b >= cfg.high_load_threshold {
                opportunity_cost_score += 0.5;
            }
        }
        _ => {}
    }

    // Responsibility gradient: higher POWER/TECH/CHURCH implies higher duty.
    let responsibility_score = compute_responsibility(&deed.pre);

    // Fairness score: simple proxy using pollution/exposure change and load context.
    let mut fairness_score = 0.0;
    if matches!(deed.kind, DeedKind::EmitPollution) && delta_b > 0.0 {
        fairness_score -= delta_b;
    }

    // Aggregate moral score (bounded but interpretable).
    let overall_moral_score = 1.0
        - harmscore.clamp(0.0, 1.0)
        - opportunity_cost_score.clamp(0.0, 1.0)
        - 0.5 * responsibility_score.clamp(0.0, 1.0)
        + fairness_score.clamp(-1.0, 0.0);

    let rationale = format!(
        "kind={:?}, pre_b={:.3}, post_b={:.3}, delta_b={:.3}, ratio_pc={:.3}",
        deed.kind, pre_b, post_b, delta_b, ratio_pc
    );

    Judgment {
        harmscore,
        opportunity_cost_score,
        responsibility_score,
        fairness_score,
        overall_moral_score,
        rationale,
    }
}
