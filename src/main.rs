use std::io;
use std::process::ExitCode;

use rs_last_bytes::line2last::LAST_BYTES_DEFAULT;

fn sub() -> Result<(), io::Error> {
    let cnt: usize = std::env::var("ENV_LAST_BYTES_CNT")
        .ok()
        .and_then(|s| str::parse(s.as_str()).ok())
        .unwrap_or(LAST_BYTES_DEFAULT);
    rs_last_bytes::line2last::stdin2stdout_default(cnt)
}

fn main() -> ExitCode {
    sub().map(|_| ExitCode::SUCCESS).unwrap_or_else(|e| {
        eprintln!("{e}");
        ExitCode::FAILURE
    })
}
