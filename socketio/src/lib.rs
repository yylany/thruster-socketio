#[macro_use]
extern crate lazy_static;
extern crate log;
#[macro_use]
extern crate serde_derive;

pub use proc::*;
pub use rooms::{get_rooms_count, get_sockets_for_room, get_sockets_number_for_room, print_sockets_for_room};
pub use socketio::{adapter, broadcast, SocketIOAdapter, SocketIOSocket as SocketIO};
pub use socketio_context::SocketIOContext;
pub use socketio_upgrade::{handle_io, handle_io_with_capacity};

mod rooms;
mod sid;
mod socketio;
mod socketio_context;
mod socketio_message;
// mod socketio_parser;
mod socketio_upgrade;

