//! API endpoints relating to the leaderboard

use acm::models::User;
use actix_web::{get, web::Json, Responder};
use rand::{thread_rng, Rng};

#[get("/leaderboard")]
async fn leaderboard() -> impl Responder {
    // TODO: implement actual leaderboard logic. How do we want this to be implemented?

    let mut data = ["Miles", "Aidan", "Alex", "Evan", "Meher", "Kevin", "Reema"]
        .iter()
        .map(|name| User {
            name: name.to_string(),
            username: name.to_string().to_lowercase(),
            star_count: thread_rng().gen_range(0..20),
            ..Default::default()
        })
        .collect::<Vec<User>>();

    data.sort_unstable_by(|a, b| b.star_count.cmp(&a.star_count));

    Json(data)
}
