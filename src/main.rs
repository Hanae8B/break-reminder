use notify_rust::Notification;
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    let tips = [
        "👀 20-20-20 Rule: Every 20 minutes, look 20 feet away for 20 seconds.",
        "🧘 Stretch your back, arms, and neck.",
        "💧 Drink some water!",
        "🦶 Walk or stand for 1–2 minutes.",
        "🖐️ Stretch your fingers and wrists.",
    ];

    let mut idx = 0;

    loop {
        Notification::new()
            .summary("Time for a Health Break 🕒")
            .body(tips[idx])
            .show()
            .unwrap();

        idx = (idx + 1) % tips.len();

        sleep(Duration::from_secs(20 * 60)).await;
    }
}
