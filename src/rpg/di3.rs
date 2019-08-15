use std::cell::RefCell;
use std::io::{Stdout, Write};

/*
    Idea: can a service take self and impls use & or &mut ?
        A: No because Rust will think self is moved/consumed after each call

*/


//trait Logger {
//    fn info(self, msg: &str);
//}
//
//trait Service1 {
//    fn do_something1(self, i: i64) -> String;
//}
//
//struct Service1Impl<'a, L:Logger> {
//    logger: L
//}
//
//// doesn't allow for alternate implementations
//impl<'a,T:HasLogger<'a>> Service1 for T {
//    fn do_something1(self, i: i64) -> String {
//        self.logger.info("do stuff 1");
//        // this won't work
//        self.logger.info("do stuff 1");
//        i.to_string()
//    }
//}

