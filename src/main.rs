use std::time::{Duration, Instant};

use forgejo_api::Forgejo;
use futures_util::StreamExt;
use octocrab::Octocrab;

pub async fn github_notifs(octocrab: &Octocrab) -> usize {
    match octocrab
        .activity()
        .notifications()
        .list()
        .per_page(100)
        .send()
        .await
    {
        Ok(notifs) => notifs.into_stream(octocrab).count().await,
        Err(e) => {
            eprintln!("{:?}", e);
            0
        }
    }
}

pub async fn codeberg_notifs(fj: &Forgejo) -> usize {
    fj.notify_new_available()
        .await
        .unwrap()
        .new
        .unwrap_or_default() as _
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let mut args = std::env::args();
    let program = args.next().expect("arg0 is always set");
    let (Some(gh_token), Some(cb_token), Some(interval)) = (args.next(), args.next(), args.next())
    else {
        eprintln!(
            "Usage: {} <gh-token-path> <cb-token-path> <interval (seconds)>",
            program
        );
        std::process::exit(1);
    };
    let interval = Duration::from_secs(interval.parse().unwrap());

    let gh_token = std::fs::read_to_string(gh_token).unwrap();
    let cb_token = std::fs::read_to_string(cb_token).unwrap();

    let octocrab = Octocrab::builder()
        .personal_token(gh_token.trim())
        .set_connect_timeout(Some(Duration::from_secs(1)))
        .set_read_timeout(Some(Duration::from_secs(1)))
        .set_write_timeout(Some(Duration::from_secs(1)))
        .build()
        .unwrap();

    let codeberg = Forgejo::new(
        forgejo_api::Auth::Token(cb_token.trim()),
        url::Url::parse("https://codeberg.org").unwrap(),
    )
    .unwrap();

    loop {
        let start = Instant::now();

        const GITHUB_ICON: &str = "";
        const CODEBERG_ICON: &str = " ";

        let github = github_notifs(&octocrab).await;
        if github > 0 {
            print!("{} {}", GITHUB_ICON, github);
        }

        let codeberg = codeberg_notifs(&codeberg).await;
        if codeberg > 0 {
            if github > 0 {
                print!("  ");
            }
            print!("{} {}", CODEBERG_ICON, codeberg);
        }

        println!();

        let elapsed = start.elapsed();
        tokio::time::sleep(interval.saturating_sub(elapsed)).await;
    }
}
