use std::time::Duration;
use std::fmt;

#[derive(Debug, Clone)]
pub struct Timer {
    pub label: String,
    pub original: Duration,
    pub left: Duration,
    pub description: String,
}

impl Timer {
    pub fn new(label: impl Into<String>, dur: Duration) -> Self {
        Self {
            label: label.into(),
            original: dur,
            left: dur,
            description: String::new(),
        }
    }

    /// Parse a string like "5m", "30s", "1:30", or "25" (minutes) into a Timer.
    pub fn parse(s: &str) -> Result<Self, &'static str> {
        if let Some(dur) = parse_duration(s) {
            Ok(Timer::new(s.trim(), dur))
        } else {
            Err("invalid time format")
        }
    }
}

impl fmt::Display for Timer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} - {}", self.label, fmt_dur(self.left))
    }
}

pub fn parse_duration(s: &str) -> Option<Duration> {
    if s.contains(':') {
        let mut parts = s.split(':');
        let m = parts.next()?.parse::<u64>().ok()?;
        let sec = parts.next().unwrap_or("0").parse::<u64>().ok()?;
        return Some(Duration::from_secs(m * 60 + sec));
    }
    let s = s.to_lowercase();
    if s.ends_with('m') {
        let num = s.trim_end_matches('m').parse::<u64>().ok()?;
        return Some(Duration::from_secs(num * 60));
    }
    if s.ends_with('s') {
        let num = s.trim_end_matches('s').parse::<u64>().ok()?;
        return Some(Duration::from_secs(num));
    }
    if let Ok(n) = s.parse::<u64>() {
        return Some(Duration::from_secs(n * 60));
    }
    None
}

pub fn fmt_dur(d: Duration) -> String {
    let total = d.as_secs();
    let m = total / 60;
    let s = total % 60;
    format!("{:02}:{:02}", m, s)
}
