use once_cell::sync::Lazy;
use rand::RngCore;
use std::sync::Mutex;
use std::time::Duration;

const SNOWFLAKE_EPOCH_MS: i64 = 1_704_067_200_000; // 2024-01-01T00:00:00Z
const SNOWFLAKE_NODE_BITS: u16 = 10;
const SNOWFLAKE_SEQUENCE_BITS: u16 = 12;
const SNOWFLAKE_MAX_SEQUENCE: u16 = (1 << SNOWFLAKE_SEQUENCE_BITS) - 1;

struct SnowflakeGenerator {
    node_id: i64,
    last_timestamp_ms: i64,
    sequence: u16,
}

impl SnowflakeGenerator {
    fn new() -> Self {
        let node_mask = (1 << SNOWFLAKE_NODE_BITS) - 1;
        let node_id = std::env::var("SNOWFLAKE_NODE_ID")
            .ok()
            .and_then(|value| value.parse::<u16>().ok())
            .unwrap_or_else(|| {
                let mut bytes = [0u8; 2];
                rand::thread_rng().fill_bytes(&mut bytes);
                u16::from_be_bytes(bytes)
            })
            & node_mask;

        Self {
            node_id: i64::from(node_id),
            last_timestamp_ms: -1,
            sequence: 0,
        }
    }

    fn next_id(&mut self) -> i64 {
        let mut timestamp_ms = current_snowflake_timestamp_ms();
        if timestamp_ms < self.last_timestamp_ms {
            timestamp_ms = self.last_timestamp_ms;
        }

        if timestamp_ms == self.last_timestamp_ms {
            self.sequence = (self.sequence + 1) & SNOWFLAKE_MAX_SEQUENCE;
            if self.sequence == 0 {
                timestamp_ms = wait_next_millis(self.last_timestamp_ms);
            }
        } else {
            self.sequence = 0;
        }

        self.last_timestamp_ms = timestamp_ms;

        (timestamp_ms << (SNOWFLAKE_NODE_BITS + SNOWFLAKE_SEQUENCE_BITS))
            | (self.node_id << SNOWFLAKE_SEQUENCE_BITS)
            | i64::from(self.sequence)
    }
}

static SNOWFLAKE_GENERATOR: Lazy<Mutex<SnowflakeGenerator>> =
    Lazy::new(|| Mutex::new(SnowflakeGenerator::new()));

fn current_snowflake_timestamp_ms() -> i64 {
    (chrono::Utc::now().timestamp_millis() - SNOWFLAKE_EPOCH_MS).max(0)
}

fn wait_next_millis(last_timestamp_ms: i64) -> i64 {
    loop {
        let timestamp_ms = current_snowflake_timestamp_ms();
        if timestamp_ms > last_timestamp_ms {
            return timestamp_ms;
        }
        std::thread::sleep(Duration::from_millis(1));
    }
}

pub fn generate_snowflake_id() -> i64 {
    SNOWFLAKE_GENERATOR.lock().unwrap().next_id()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn snowflake_ids_are_positive_and_unique() {
        let first = generate_snowflake_id();
        let second = generate_snowflake_id();

        assert!(first > 0);
        assert!(second > 0);
        assert_ne!(first, second);
    }
}
