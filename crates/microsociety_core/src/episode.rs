use serde::{Deserialize, Serialize};

use crate::deed::{EventLog};
use crate::model::World;

/// Single simulation run: world trajectory + deeds + reflection hooks.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Episode {
    pub id: String,
    pub world_init: World,
    pub world_final: World,
    pub event_log: EventLog,

    /// Episode-level metrics (health-equivalent, justice-equivalent, governance).
    pub total_exposure: f64,
    pub total_bioload: f64,
    pub collapse_events: u64,
    pub gini_power: f64,
    pub gini_tech: f64,
    pub gini_church: f64,

    /// W-cycle reflections (What / So what / Now what) as structured hooks.
    pub what_reflections: Vec<String>,
    pub so_what_reflections: Vec<String>,
    pub now_what_reflections: Vec<String>,
}

impl Episode {
    pub fn new(id: String, world_init: World) -> Self {
        Self {
            id,
            world_init: world_init.clone(),
            world_final: world_init,
            event_log: EventLog::default(),
            total_exposure: 0.0,
            total_bioload: 0.0,
            collapse_events: 0,
            gini_power: 0.0,
            gini_tech: 0.0,
            gini_church: 0.0,
            what_reflections: Vec::new(),
            so_what_reflections: Vec::new(),
            now_what_reflections: Vec::new(),
        }
    }

    pub fn finalize(&mut self, final_world: World) {
        self.world_final = final_world;
        // Metric aggregation would be computed here from event_log and world_final.
    }
}
