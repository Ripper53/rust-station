use std::collections::VecDeque;

pub fn create_command_channel<Command>() -> (CommandSender<Command>, CommandReceiver<Command>) {
    let commands = std::rc::Rc::new(std::cell::RefCell::new(VecDeque::new()));
    (
        CommandSender {
            commands: std::rc::Rc::clone(&commands),
        },
        CommandReceiver { commands },
    )
}

#[derive(Debug)]
pub struct CommandSender<Command> {
    commands: std::rc::Rc<std::cell::RefCell<VecDeque<Command>>>,
}

impl<Command> Clone for CommandSender<Command> {
    fn clone(&self) -> Self {
        CommandSender {
            commands: std::rc::Rc::clone(&self.commands),
        }
    }
}

impl<Command> CommandSender<Command> {
    pub fn send(&self, command: Command) {
        self.commands.borrow_mut().push_back(command);
    }
}

pub struct CommandReceiver<Command> {
    commands: std::rc::Rc<std::cell::RefCell<VecDeque<Command>>>,
}

impl<Command> CommandReceiver<Command> {
    pub fn receive(&self) -> Option<Command> {
        self.commands.borrow_mut().pop_front()
    }
}
