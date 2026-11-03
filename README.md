# rust-e2e-fixture

Deliberately vulnerable Rust handlers for testing RubberDuck's Rust analysis end to end. Not meant to compile or run.

Expected findings in `src/main.rs`:

| Line | Flow |
|---|---|
| 7 | query parameter reaches `Command::new("sh").arg("-c")` |
| 13 | query parameter reaches `sqlx::query(&format!(..))` |
| 20 | route parameter reaches `std::fs::read_to_string` |
| 26 | `env::var` reaches `Command::new` |

Expected dependency finding in `Cargo.lock`: `smallvec` 1.6.0 (CVE-2021-25900, fixed in 1.6.1).
