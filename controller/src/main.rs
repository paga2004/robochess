//!  This is the controller for the chess board which runs on a raspberry pi zero. The crate
//!  [`rust_gpiozero`](https://crates.io/crates/rust_gpiozero) is used for controlling the gpio
//!  pins. There is a really good python library called
//!  [`gpiozero`](https://gpiozero.readthedocs.io/en/stable/).
//!  [`rust_gpiozero`](https://crates.io/crates/rust_gpiozero) tries to replicate the interface and
//!  some of its functionality. It's not really idomatic and there are some bugs
//!  which require workarounds (cf. [stepper::StepperMotor::turn_steps]). But it works and it's
//!  more high level than the alternatives.

mod hbot;
mod robochess;
mod stepper;

use websocket::sync::Server;
use websocket::OwnedMessage;

use chers::{Color, ParsedMove, Position};

use robochess::RoboChess;

const SUB_PROTOCOL: &'static str = "robochess-websocket";

fn main() {
    let mut controller = RoboChess::new();
    // controller.position =
    //     Position::from_fen("r3k2r/pp1bqppp/2pp1n2/4p3/1bBPPB1N/2N5/PPP1QPPP/R3K2R b KQkq - 6 13")
    //         .unwrap();
    // controller.make_move(ParsedMove::from_coordinate_notation("e8c8").unwrap());
    // return;
    let server = Server::bind("0.0.0.0:8080").unwrap();

    'outer: for request in server.filter_map(Result::ok) {
        if !request.protocols().contains(&SUB_PROTOCOL.to_string()) {
            println!("Invalid subprotocols: {:?}", request.protocols());
            request.reject().unwrap();
            return;
        }

        let client = request.use_protocol(SUB_PROTOCOL).accept().unwrap();

        let ip = client.peer_addr().unwrap();

        println!("Connection from {}", ip);

        let (mut receiver, mut sender) = client.split().unwrap();

        let message = OwnedMessage::Text(format!("!set {}", controller.position.to_fen()));
        if sender.send_message(&message).is_err() {
            continue 'outer;
        }

        for message in receiver.incoming_messages() {
            if let Ok(message) = message {
                match message {
                    OwnedMessage::Close(_) => {
                        let message = OwnedMessage::Close(None);
                        if sender.send_message(&message).is_err() {
                            continue 'outer;
                        }
                        println!("Client {} disconnected", ip);
                        continue 'outer;
                    }
                    OwnedMessage::Ping(ping) => {
                        let message = OwnedMessage::Pong(ping);
                        println!("pong!");
                        if sender.send_message(&message).is_err() {
                            continue 'outer;
                        }
                    }
                    OwnedMessage::Text(data) => {
                        if data.starts_with("!") {
                            match data.as_str() {
                                "!calibrate" => {
                                    controller.controller.init_sequence();
                                }
                                s if s.starts_with("!fen") => {
                                    if let Ok(pos) = Position::from_fen(&s[5..]) {
                                        controller.position = pos;
                                        controller.captured_pieces_white.clear();
                                        controller.captured_pieces_black.clear();
                                    }

                                    let message = OwnedMessage::Text(format!(
                                        "!set {}",
                                        controller.position.to_fen()
                                    ));
                                    if sender.send_message(&message).is_err() {
                                        continue 'outer;
                                    }
                                    let message = if controller.position.is_checkmate() {
                                        OwnedMessage::Text("!checkmate".to_string())
                                    } else if controller.position.is_draw() {
                                        OwnedMessage::Text("!draw".to_string())
                                    } else if controller.position.side_to_move() == Color::WHITE {
                                        OwnedMessage::Text("!white".to_string())
                                    } else {
                                        OwnedMessage::Text("!black".to_string())
                                    };
                                    if sender.send_message(&message).is_err() {
                                        continue 'outer;
                                    }
                                }
                                _ => println!("Invalid command {}", data),
                            }
                        } else {
                            if let Ok(m) = ParsedMove::from_coordinate_notation(&data) {
                                if !controller.make_move(m) {
                                    let message = OwnedMessage::Text(format!(
                                        "!set {}",
                                        controller.position.to_fen()
                                    ));
                                    if sender.send_message(&message).is_err() {
                                        continue 'outer;
                                    }
                                }

                                let message = if controller.position.is_checkmate() {
                                    OwnedMessage::Text("!checkmate".to_string())
                                } else if controller.position.is_draw() {
                                    OwnedMessage::Text("!draw".to_string())
                                } else if controller.position.side_to_move() == Color::WHITE {
                                    OwnedMessage::Text("!white".to_string())
                                } else {
                                    OwnedMessage::Text("!black".to_string())
                                };
                                if sender.send_message(&message).is_err() {
                                    continue 'outer;
                                }
                            }
                        }
                    }
                    _ => {
                        println!("Unexpected message: {:?}", message);
                    }
                }
            }
        }
    }
}
