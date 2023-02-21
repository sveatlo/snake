use snake::game::Game;

#[tokio::main]
async fn main() {
    let mut game = Game::new().unwrap();
    game.run().await.unwrap();
}
