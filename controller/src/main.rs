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

use chers::ParsedMove;

use robochess::RoboChess;

fn main() {
    let mut controller = RoboChess::new();
    let server = Server::bind("0.0.0.0:8080").unwrap();

    for request in server.filter_map(Result::ok) {
        if !request.protocols().contains(&"rust-websocket".to_string()) {
            request.reject().unwrap();
            return;
        }

        let client = request.use_protocol("rust-websocket").accept().unwrap();

        let ip = client.peer_addr().unwrap();

        println!("Connection from {}", ip);

        let (mut receiver, mut sender) = client.split().unwrap();

        for message in receiver.incoming_messages() {
            let message = message.unwrap();

            match message {
                OwnedMessage::Close(_) => {
                    let message = OwnedMessage::Close(None);
                    sender.send_message(&message).unwrap();
                    println!("Client {} disconnected", ip);
                    return;
                }
                OwnedMessage::Ping(ping) => {
                    let message = OwnedMessage::Pong(ping);
                    sender.send_message(&message).unwrap();
                }
                OwnedMessage::Text(data) => {
                    let m = ParsedMove::from_coordinate_notation(&data).unwrap();
                    let answer = if controller.make_move(m) {
                        OwnedMessage::Text("OK".to_string())
                    } else {
                        OwnedMessage::Text("Error: Invalid move".to_string())
                    };
                    sender.send_message(&answer).unwrap();
                }
                _ => {
                    println!("Unexpected message: {:?}", message);
                }
            }
        }
    }
}
