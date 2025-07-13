pub mod commands;

pub struct Command<const N: usize> {
    identifier: commands::Identifier,
    payload: Option<[u8;N]>,
    carriage_returns: u8,
}

pub fn encode_command<const N: usize, C: Into<Command<N>>> (cmd: C) {

}

