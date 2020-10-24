mod structed {
    use std::sync::mpsc;
    use std::thread;
    use std::time::Duration;

    enum InterfaceCommands {
        StopThread,
        DoSomething,
    }
    pub struct Interface {
        command_pipe: mpsc::Sender<InterfaceCommands>,
        join_handler: thread::JoinHandle::<()>,
    }

    impl Interface {
        pub fn stop_working(self){
            self.command_pipe.send(InterfaceCommands::StopThread).expect("Unable to stop_working");
            self.join_handler.join().expect("Worker thread cannot be joined somehow")
        }
        pub fn do_something(&self) {
            self.command_pipe.send(InterfaceCommands::DoSomething).expect("Unable to do_something");
        }
    }

    pub fn start_worker() -> Interface {
        let (tx, rx) = mpsc::channel::<InterfaceCommands>();
        let closure = 
            move || {loop {
                println!("from closure");
                let receieve = rx.try_recv();
                match receieve {
                    Ok(command) => {match command {
                        InterfaceCommands::DoSomething => { println!("Doing something");},
                        InterfaceCommands::StopThread => {println!("Stopping worker"); break;}
                    }},
                    Err(err) => {
                        println!("Not received any command, err is: {:?}", err);
                        thread::sleep(Duration::from_millis(500));
                    },
                };
            }
        };
        let join_handler = thread::spawn(closure);
        let interface = Interface {command_pipe: tx, join_handler: join_handler};
        interface
    }
}

fn main() {
    use std::thread;
    use std::time::Duration;

    let interface = structed::start_worker();
    thread::sleep(Duration::from_secs(2));
    interface.do_something();
    interface.do_something();
    interface.stop_working();
}
