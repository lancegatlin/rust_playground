//use std::cell::RefCell;
//use std::io::{Stdout, Write};
//
//trait MutLogger {
//    fn info(&mut self, msg: &str);
//}
//
//impl MutLogger for Vec<String> {
//    fn info(&mut self, msg: &str) {
//        self.push(msg.to_string())
//    }
//}
//
//impl MutLogger for Box<dyn Write> {
//    fn info(&mut self, msg: &str) {
//        write!(self, "{}\n", msg).unwrap()
//    }
//}
//
//impl<L:MutLogger> Logger for RefCell<L> {
//    fn info(&self, msg: &str) {
//        self.borrow_mut().info(msg)
//    }
//}
//
//trait Logger {
//    fn info(&self, msg: &str);
//}
//
//trait HasLogger<'a> {
//    // can't use generics in methods of trait objects
//    fn logger(&self) -> &'a dyn Logger;
//}
//
//trait Service1 {
//    fn do_something1(&self, i: i64) -> String;
//}
//
//struct Service1Impl<'a, T:HasLogger<'a>>(T);
//
//// doesn't allow for alternate implementations
//impl<'a,T:HasLogger<'a>> Service1 for T {
//    fn do_something1(&self, i: i64) -> String {
//        self.logger().info("do stuff 1");
//        i.to_string()
//    }
//}
//
//trait Service2 {
//    fn do_something2(&self, i: i64) -> String;
//}
//
//struct Service2Impl<'a, L:Logger> {
//    logger: &'a L
//}
//
//impl<'a,L:Logger> Service2 for Service2Impl<'a,L> {
//    fn do_something2(&self, i: i64) -> String {
//        self.logger.info("do stuff 2");
//        i.to_string()
//    }
//}
//
//struct World<'a> {
//    logger: &'a dyn Logger
//}
//
//impl<'a> HasLogger<'a> for World<'a> {
//    fn logger(&self) -> &'a Logger {
//        self.logger
//    }
//}
//
//#[cfg(test)]
//mod test {
//    use super::*;
//
//    #[test]
//    fn main_test() {
////        let logger = RefCell::new(Vec::<String>::new());
//        let logger : RefCell<Box<dyn Write>> = RefCell::new(Box::new(std::io::stdout()));
//        let world = World {
//            logger: &logger
//        };
////        let service1 = Service1Impl {
////            logger: &logger
////        };
//        let service2 = Service2Impl {
//            logger: &logger
//        };
//        world.do_something1(123);
//        service2.do_something2(123);
//        // let world = World {
//        //     logger: logger
//        // }
////        for msg in logger.borrow().iter() {
////            println!("{}", msg)
////        }
//    }
//}