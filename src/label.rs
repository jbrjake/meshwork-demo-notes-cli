const MINUTE: u64 = 60_000;
const HOUR: u64 = 60 * MINUTE;
const DAY: u64 = 24 * HOUR;

/// When a note was edited, relative to `now_ms`: "edited just now",
/// "edited 5 minutes ago", "edited 3 hours ago", "edited 2 days ago". A time
/// ahead of `now_ms`, which another device's fast clock can produce, reads
/// "edited in 5 minutes".
pub fn edited_ago(now_ms: u64, at_ms: u64) -> String {
    if at_ms > now_ms {
        let ahead = at_ms - now_ms;
        if ahead < MINUTE {
            return "edited just now".to_string();
        }
        return format!("edited in {}", span(ahead));
    }
    let ago = now_ms - at_ms;
    if ago < MINUTE {
        return "edited just now".to_string();
    }
    format!("edited {} ago", span(ago))
}

fn span(ms: u64) -> String {
    let (count, unit) = if ms < HOUR {
        (ms / MINUTE, "minute")
    } else if ms < DAY {
        (ms / HOUR, "hour")
    } else {
        (ms / DAY, "day")
    };
    if count == 1 {
        format!("1 {unit}")
    } else {
        format!("{count} {unit}s")
    }
}
