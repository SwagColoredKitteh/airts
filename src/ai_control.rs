use command::Command;
use game::GameState;

use std::sync::mpsc::{channel, Sender, Receiver};
use std::sync::{Arc, Mutex, Barrier, Weak};
use std::io::prelude::*;
use std::io;
use std::thread;

struct Request {
    game: Arc<GameState>,
    barrier: Arc<Barrier>,
    commands: Weak<Mutex<Vec<Command>>>
}

pub struct AIControl {
    submit: Sender<Request>
}

impl AIControl {
    pub fn new(read: Box<BufRead + Send>, write: Box<Write + Send>, initial: Arc<GameState>) -> AIControl {
        let (tx, rx) = channel();
        AIChild::start(read, write, initial, rx);
        AIControl {
            submit: tx
        }
    }

    pub fn run(&mut self, state: Arc<GameState>) -> Result<Vec<Command>, ()> {
        let barrier = Arc::new(Barrier::new(2));
        let commands = Arc::new(Mutex::new(Vec::new()));
        try!(self.submit.send(Request {
            game: state,
            barrier: barrier.clone(),
            commands: Arc::downgrade(&commands)
        }).map_err(|_| ()));
        barrier.wait();
        Ok(try!(try!(Arc::try_unwrap(commands).map_err(|e| ())).into_inner().map_err(|e| ())))
    }
}

struct AIChild {
    read: Box<BufRead>,
    write: Box<Write>
}

impl AIChild {
    fn start(read: Box<BufRead + Send>, write: Box<Write + Send>, initial: Arc<GameState>, submit: Receiver<Request>) {
        thread::spawn(move || { // TODO: better error handling
            let mut me = AIChild {
                read: read,
                write: write
            };
            me.init(initial.as_ref()).unwrap();
            loop {
                let req = submit.recv().unwrap();
                {
                    let temp_arc = req.commands.upgrade().unwrap();
                    let mut guard = temp_arc.lock().unwrap();
                    me.run(req.game.as_ref(), &mut guard).unwrap(); // TODO: termination, somehow
                }
                req.barrier.wait();
            }
        });
    }

    fn init(&mut self, game: &GameState) -> Result<(), io::Error> {
        game.map.dump_state(&mut self.write)
    }

    fn run(&mut self, game: &GameState, cmds_out: &mut Vec<Command>) -> Result<(), io::Error> { // TODO: better error handling
        game.dump_state(&mut self.write);
        let mut buf = String::new();
        try!(self.read.read_line(&mut buf));
        let cmd_count: usize = try!(buf.trim().parse().map_err(|e| io::Error::new(io::ErrorKind::InvalidData, "expected command count")));
        for _ in 0..cmd_count {
            buf.clear();
            try!(self.read.read_line(&mut buf));
            let cmd: Command = try!(buf.trim().parse().map_err(|e| io::Error::new(io::ErrorKind::InvalidData, "invalid command")));
            cmds_out.push(cmd);
        }
        Ok(())
    }
}