
use std::cell::RefCell;
use std::io::{Stdout, Write};

/*
    Idea: can a service take &self and impl &mut?
        A: no
*/


//trait Logger {
//    fn info(&self, msg: &str);
//}
//
//impl Logger for &mut Vec<String> {
////    fn info(&self, msg: &str) {
////        self.push(msg.to_string())
////    }
//
////    fn info(&&mut self, msg: &str) {
////        self.push(msg.to_string())
////    }
//}
//
//trait Service1 {
//    fn do_something1(&self, i: i64) -> String;
//}
//
//struct Service1Impl<L:Logger> {
//    logger: L
//}
//
//// doesn't allow for alternate implementations
//impl<L:Logger> Service1 for Service1Impl<L> {
//    fn do_something1(&self, i: i64) -> String {
//        self.logger.info("do stuff 1");
//        i.to_string()
//    }
//}

