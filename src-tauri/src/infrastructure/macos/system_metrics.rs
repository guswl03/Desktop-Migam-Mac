use std::{
    sync::Mutex,
    time::{Duration, Instant},
};

use sysinfo::{CpuRefreshKind, MemoryRefreshKind, RefreshKind, System};

#[derive(Clone, Copy, Debug, Default)]
pub struct SystemMetricsSnapshot {
    pub cpu_percent: u8,
    pub memory_percent: u8,
}

struct SamplingState {
    system: System,
    smoothed_cpu: f64,
    last_sample_at: Option<Instant>,
    snapshot: SystemMetricsSnapshot,
}

pub struct SystemMetricsMonitor {
    state: Mutex<SamplingState>,
}

impl Default for SystemMetricsMonitor {
    fn default() -> Self {
        Self {
            state: Mutex::new(SamplingState {
                system: System::new_with_specifics(
                    RefreshKind::nothing()
                        .with_cpu(CpuRefreshKind::everything())
                        .with_memory(MemoryRefreshKind::everything()),
                ),
                smoothed_cpu: 0.0,
                last_sample_at: None,
                snapshot: SystemMetricsSnapshot::default(),
            }),
        }
    }
}

impl SystemMetricsMonitor {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn poll(&self) -> Result<SystemMetricsSnapshot, String> {
        let mut state = self
            .state
            .lock()
            .map_err(|_| "system metrics state is unavailable".to_owned())?;
        let now = Instant::now();
        if state
            .last_sample_at
            .is_some_and(|sampled| now.duration_since(sampled) < Duration::from_millis(750))
        {
            return Ok(state.snapshot);
        }
        state.system.refresh_cpu_usage();
        state.system.refresh_memory();
        let measured = f64::from(state.system.global_cpu_usage());
        state.smoothed_cpu = if state.last_sample_at.is_some() {
            state.smoothed_cpu * 0.65 + measured * 0.35
        } else {
            measured
        };
        let total = state.system.total_memory();
        let memory = if total == 0 {
            0
        } else {
            ((state.system.used_memory() as f64 / total as f64) * 100.0).round() as u8
        };
        state.snapshot = SystemMetricsSnapshot {
            cpu_percent: state.smoothed_cpu.round().clamp(0.0, 100.0) as u8,
            memory_percent: memory.min(100),
        };
        state.last_sample_at = Some(now);
        Ok(state.snapshot)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn metrics_are_bounded() {
        let snapshot = SystemMetricsMonitor::new().poll().unwrap();
        assert!(snapshot.cpu_percent <= 100);
        assert!(snapshot.memory_percent <= 100);
    }
}
