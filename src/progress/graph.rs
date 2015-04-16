use std::cell::RefCell;
use progress::{Timestamp, Scope, Subgraph};
use progress::nested::{Source, Target};
use communication::Communicator;

pub trait Graph {
    type Timestamp : Timestamp;
    type Communicator : Communicator;

    fn connect(&mut self, source: Source, target: Target);
    fn add_boxed_scope(&mut self, scope: Box<Scope<Self::Timestamp>>) -> u64;
    fn add_scope<SC: Scope<Self::Timestamp>+'static>(&mut self, scope: SC) -> u64 { self.add_boxed_scope(Box::new(scope)) }
    fn new_subgraph<T: Timestamp>(&mut self) -> Subgraph<Self::Timestamp, T>;
    fn communicator(&mut self) -> &mut Self::Communicator;

    fn builder(&mut self) -> RefCell<&mut Self>;
}
