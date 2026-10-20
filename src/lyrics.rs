use std::fs;
use std::path::Path;
use std::time::Duration;

#[derive(Debug, Clone, Default)]
pub struct Lyrics {
    lines: Vec<LyricLine>,
}

#[derive(Debug, Clone)]
struct LyricLine {
    time: Duration,
    text: String,
}

impl Lyrics {
    pub fn load_for_track(track_path: &Path) -> Option<Self> {
        let lrc_path = track_path.with_extension("lrc");

        let lrc_path = if lrc_path.exists() {
            lrc_path
        } else {
            track_path
                .parent()?
                .join("lrc")
                .join(track_path.with_extension("lrc").file_name()?)
        };

        let content = fs::read_to_string(lrc_path).ok()?;

        Some(Self::parse(&content))
    }

    fn parse(content: &str) -> Self {
        let mut lines = Vec::new();

        for raw_line in content.lines() {
            let mut rest = raw_line;
            let mut timestamps = Vec::new();

            while let Some(start) = rest.find('[') {
                let Some(end) = rest[start..].find(']').map(|i| i + start) else {
                    break;
                };

                let tag = &rest[start + 1..end];

                let Some(time) = parse_timestamp(tag) else {
                    break;
                };

                timestamps.push(time);
                rest = &rest[end + 1..];
            }

            if timestamps.is_empty() {
                continue;
            }

            let text = rest.trim().to_string();

            for time in timestamps {
                lines.push(LyricLine {
                    time,
                    text: text.clone(),
                });
            }
        }

        lines.sort_by_key(|line| line.time);

        Self { lines }
    }

    pub fn is_empty(&self) -> bool {
        self.lines.is_empty()
    }

    pub fn line_at(&self, position: Duration) -> Option<&str> {
        self.lines
            .iter()
            .rev()
            .find(|line| line.time <= position)
            .map(|line| line.text.as_str())
    }
}

fn parse_timestamp(tag: &str) -> Option<Duration> {
    let tag = tag.trim();
    let mut parts = tag.splitn(2, ':');

    let minutes: u64 = parts.next()?.trim().parse().ok()?;
    let rest = parts.next()?.trim().replace(':', ".");
    let seconds: f64 = rest.parse().ok()?;

    Some(Duration::from_secs_f64(minutes as f64 * 60.0 + seconds))
}
