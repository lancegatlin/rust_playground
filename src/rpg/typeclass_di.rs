
/*
    Idea: can a typeclass have its own config type and then have that propagate?
        A: NO because associated types can't have lifetime parameters
            both in declaration or in impl

    RFC would add this capability
    https://github.com/rust-lang/rfcs/blob/master/text/1598-generic_associated_types.md
*/

//trait Logger {
//    fn info(&self, msg: &str);
//}
//
//trait ParseFrom<I> {
//    type Config;
//
//    // doesn't work
////    type Config<'a>;
//
//    fn parse_from(config: &Self::Config, from: I) -> Result<Self,String>;
//}
//
//enum IntegerParseType {
//    DECIMAL,
//    HEX,
//    OCT,
//}
//
//impl ParseFrom<String> for i64 {
//    type Config = IntegerParseType;
//
//    fn parse_from(config: &Self::Config, from: String) -> Result<i64, String> {
//        match config {
//            IntegerParseType::DECIMAL =>
//                i64::from_str_radix(from, 10),
//            IntegerParseType::HEX =>
//                i64::from_str_radix(from, 16),
//            IntegerParseType::OCT =>
//                i64::from_str_radix(from, 8),
//        }
//    }
//}
//
//struct A {
//    i1: i64,
//    i2: i64
//}
//
//struct ParseConfig<'a> {
//    logger: &'a dyn Logger,
//    integer_parse_type: IntegerParseType
//}
//
//impl ParseFrom<String> for A {
//    // doesn't work since no <'a>
////    type Config = ParseConfig;
//    // doesn't work either
////    type Config = ParseConfig<'a>;
//
//    fn parse_from(config: &Self::Config, from: String) -> Result<A, String> {
//        let iter = from.split(',');
//        let s1 = iter.next().unwrap();
//        let s2 = iter.next().unwrap();
//        let i1 = i64::parse_from(&config.integer_parse_type, s1);
//        let i2 = i64::parse_from(&config.integer_parse_type, s2);
//        A {
//            i1,
//            i2
//        }
//    }
//}