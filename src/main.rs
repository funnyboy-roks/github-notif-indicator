use std::time::{Duration, Instant};

use octocrab::Octocrab;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let mut args = std::env::args();
    let program = args.next().expect("arg0 is always set");
    let (Some(token_path), Some(interval)) = (args.next(), args.next()) else {
        eprintln!("Usage: {} <token-path> <interval (seconds)>", program);
        std::process::exit(1);
    };
    let interval = Duration::from_secs(interval.parse().unwrap());

    let token = std::fs::read_to_string(token_path).unwrap();

    let octocrab = Octocrab::builder()
        .personal_token(token.trim())
        .set_connect_timeout(Some(Duration::from_secs(1)))
        .set_read_timeout(Some(Duration::from_secs(1)))
        .set_write_timeout(Some(Duration::from_secs(1)))
        .build()
        .unwrap();

    loop {
        let start = Instant::now();
        match octocrab
            .activity()
            .notifications()
            .list()
            .per_page(100)
            .send()
            .await
        {
            Ok(notifs) => {
                let notifs = notifs.into_iter().count();
                if notifs == 0 {
                    println!();
                } else {
                    println!(" {}", notifs);
                }
            }
            Err(e) => {
                eprintln!("{:?}", e);
            }
        }

        let elapsed = start.elapsed();
        tokio::time::sleep(interval.saturating_sub(elapsed)).await;
    }
}
