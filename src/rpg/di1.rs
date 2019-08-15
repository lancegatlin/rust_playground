use std::cell::RefCell;
use std::io::{Stdout, Write};

// struct ClosureMut<T> {
//   value : T
// }

// impl ClosureMut {
//     fn borrow<A,F>(&mut self, f: F) -> A where F:FnOnce(&mut T) -> A {
//         (f)(&mut self.value)
//     }
// }

trait MutLogger {
    fn info(&mut self, msg: &str);
}

impl MutLogger for Vec<String> {
    fn info(&mut self, msg: &str) {
        self.push(msg.to_string())
    }
}

impl MutLogger for Box<dyn Write> {
    fn info(&mut self, msg: &str) {
        write!(self, "{}\n", msg).unwrap()
    }
}

impl<L:MutLogger> Logger for RefCell<L> {
    fn info(&self, msg: &str) {
        self.borrow_mut().info(msg)
    }
}

trait Logger {
    fn info(&self, msg: &str);
}

//struct MutLoggerShim<L:MutLogger> {
//    logger: RefCell<L>
//}
//
//impl<L:MutLogger> Logger for MutLoggerShim<L> {
//    fn info(&self, msg: &str) {
//        self.logger.borrow_mut().info(msg)
//    }
//}
//
//trait HasLogger {
//    fn logger(&self) -> &dyn Logger;
//}

// struct StdOutLogger { }

// impl Logger for StdOutLogger {
//     fn info(&mut self, msg: &str) {
//         println!("{}", msg)
//     }
// }

//trait HasLogger<'a> {
//    // fn with_logger<A,F:FnOnce(&mut Logger) -> A>(&mut self, f: F) -> A;
//    fn logger<'b:'a>(&'a self) -> &'b mut dyn Logger;
//}
//
//struct HasLoggerImpl<L:Logger> {
//    // logger: RefCell<L>
//    logger: L
//}
//
//impl<'a,L:Logger> HasLogger<'a> for HasLoggerImpl<L> {
//    // fn with_logger<A,F:FnOnce(&mut Logger) -> A>(&mut self, f: F) -> A {
//    //     // (f)(&mut *self.logger.borrow_mut())
//    //     (f)(&mut self.logger)
//    // }
//    fn logger<'b:'a>(&'a self) -> &'b mut dyn Logger {
//        &mut self.logger
//    }
//}

trait Service1 {
    fn do_something1(&self, i: i64) -> String;
}

struct Service1Impl<'a, L:Logger> {
    logger: &'a L
}

impl<'a,L:Logger> Service1 for Service1Impl<'a, L> {
    fn do_something1(&self, i: i64) -> String {
        self.logger.info("do stuff 1");
        i.to_string()
    }
}

trait Service2 {
    fn do_something2(&self, i: i64) -> String;
}

struct Service2Impl<'a, L:Logger> {
    logger: &'a L
}

impl<'a,L:Logger> Service2 for Service2Impl<'a,L> {
    fn do_something2(&self, i: i64) -> String {
        self.logger.info("do stuff 2");
        i.to_string()
    }
}


// struct WorldImpl<
//     L:Logger,
//     S1:Service1,
//     S2:Service2
// > {
//     logger: Logger,
//     s1: S1,
//     s2: S2
// }

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn main_test() {
//        let logger = RefCell::new(Vec::<String>::new());
        let logger : RefCell<Box<dyn Write>> = RefCell::new(Box::new(std::io::stdout()));
        let service1 = Service1Impl {
            logger: &logger
        };
        let service2 = Service2Impl {
            logger: &logger
        };
        service1.do_something1(123);
        service2.do_something2(123);
        // let world = World {
        //     logger: logger
        // }
//        for msg in logger.borrow().iter() {
//            println!("{}", msg)
//        }
    }
}