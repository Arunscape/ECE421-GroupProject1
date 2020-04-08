use connect_game::games::*;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut game = if args.len() > 1 {
        match args[1].as_ref() {
            "toto" => toto(),
            "toto_ai" => toto_ai(),
            "3" => connect4_3player(),
            "ai" => connect4_ai(),
            "aibig" => connect4_large_ai(),
            "ai2" => connect4_ai_p2(),
            _ => connect4(),
        }
    } else {
        connect4()
    };
    connect_game::play(&mut game, connect_game::io::TermIO::new())
}
