enum Direction{
    N,
    E,
    S,
    W
}
enum PlayerAction {
    Move {
        direction: Direction,
        speed: u8 },
    Wait,
    Attack(Direction)
}
fn main() {
    let simulated_player_action = PlayerAction::Move {
        direction: Direction::N,
        speed: 2,
    };
    match simulated_player_action {
        PlayerAction::Wait =>println!("Player want to wait"),
        PlayerAction::Move {direction,speed}=>{
            println!("Player want to move direction {:#?} with speed{}",direction,speed)
        }
        PlayerAction::Attack(direction)=>{
            println!("Player want to attack direction {:#?}",direction)
        }
        _ => {}
    };
}

