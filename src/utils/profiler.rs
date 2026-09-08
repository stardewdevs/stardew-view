use std::collections::HashMap;
use std::time::{Duration, Instant};

pub struct Profiler {
    sections: HashMap<String, ProfilerSection>,
    active: Option<String>,
}

struct ProfilerSection {
    total: Duration,
    calls: u32,
    last_start: Option<Instant>,
}

impl Profiler {
    pub fn new() -> Self {
        Self {
            sections: HashMap::new(),
            active: None,
        }
    }

    pub fn begin(&mut self, name: &str) {
        self.end_current();
        let section = self
            .sections
            .entry(name.to_string())
            .or_insert_with(|| ProfilerSection {
                total: Duration::ZERO,
                calls: 0,
                last_start: None,
            });
        section.last_start = Some(Instant::now());
        self.active = Some(name.to_string());
    }

    pub fn end(&mut self, name: &str) {
        if let Some(section) = self.sections.get_mut(name) {
            if let Some(start) = section.last_start.take() {
                section.total += start.elapsed();
                section.calls += 1;
            }
        }
        self.active = None;
    }

    pub fn end_current(&mut self) {
        if let Some(name) = self.active.take() {
            if let Some(section) = self.sections.get_mut(&name) {
                if let Some(start) = section.last_start.take() {
                    section.total += start.elapsed();
                    section.calls += 1;
                }
            }
        }
    }

    pub fn get_stats(&self, name: &str) -> Option<(Duration, u32)> {
        self.sections.get(name).map(|s| (s.total, s.calls))
    }

    pub fn get_all_stats(&self) -> Vec<(&str, Duration, u32)> {
        self.sections
            .iter()
            .map(|(name, s)| (name.as_str(), s.total, s.calls))
            .collect()
    }

    pub fn reset(&mut self) {
        self.sections.clear();
        self.active = None;
    }
}
