
/*
    Idea: can a typeclass be used simply to "route" a call to a DI trait object?
        A: YES, though this might not really help?
*/

trait Logger {
    fn info(&self, msg: &str);
}

trait Parser<I,O> {
    fn parse(&self, from: I) -> Result<O,String>;
}

impl<I,O,F> Parser<I,O> for F where F:Fn(I) -> Result<O,String> {
    fn parse(&self, item: I) -> Result<O, String> {
        (self)(item)
    }
}


trait ParseFrom<I> : Sized {
    fn parse_from<P:Parser<I,Self>+?Sized>(parser: &P, from: I) -> Result<Self,String> {
        parser.parse(from)
    }
}

impl<I,O> ParseFrom<I> for O { }

enum IntegerParseType {
    DECIMAL,
    HEX,
    OCT,
}

struct I64Parser {
    integer_parse_type: IntegerParseType
}

fn parse_i64_from_string(
    integer_parse_type: &IntegerParseType,
    from: String
) -> Result<i64,String> {
    let temp =
        match integer_parse_type {
            IntegerParseType::DECIMAL =>
                i64::from_str_radix(&from, 10),
            IntegerParseType::HEX =>
                i64::from_str_radix(&from, 16),
            IntegerParseType::OCT =>
                i64::from_str_radix(&from, 8),
        };

    temp.map_err(|err|err.to_string())
}

fn parse_i64_from_decimal_string(
    from: String
) -> Result<i64,String> {
    i64::from_str_radix(&from, 10)
        .map_err(|err|err.to_string())
}


impl Parser<String,i64> for I64Parser {
    fn parse(&self, from: String) -> Result<i64,String> {
        parse_i64_from_string(&self.integer_parse_type, from)
    }
}

struct A {
    i1: i64,
    i2: i64
}

// super parser? module parser?
struct AParser<'a> {
    logger: &'a dyn Logger,
    integer_parse_type: IntegerParseType,
    i64_parser: &'a dyn Parser<String, i64>,
    u8_parser: &'a dyn Parser<String, u8>
}

impl<'a> Parser<String,i64> for AParser<'a> {
    fn parse(&self, from: String) -> Result<i64, String> {
        parse_i64_from_string(&self.integer_parse_type, from)
//        self.i64_parser.parse(from)
    }
}

impl<'a> Parser<String,u8> for AParser<'a> {
    fn parse(&self, from: String) -> Result<u8, String> {
        self.u8_parser.parse(from)
    }
}


impl<'a> Parser<String,A> for AParser<'a> {
    fn parse(&self, from: String) -> Result<A, String> {
        let mut iter = from.split(',');
        let s1 = iter.next().unwrap();
        let s2 = iter.next().unwrap();
        // why not just self.parse(s1) ?

//        let i1 = i64::parse_from(self, s1.to_string())?;

        // notice type inference static dispatches to correct impl!
        let i1 = self.parse(s1.to_string())?;
        let u1 : u8 = self.parse(s1.to_string())?;
        let u2  = self.u8_parser.parse(s1.to_string())?;

        // this works, but not needed, and not really helpful???
        // it is regular?
        // but isn't self.parse regular too?
        let i2 = i64::parse_from(self, s2.to_string())?;

        let parser = |from| {
            parse_i64_from_string(&self.integer_parse_type, from)
        };
        let i3 = i64::parse_from(&parser, s2.to_string())?;

        let i4 = i64::parse_from(&|from| {
            parse_i64_from_string(&self.integer_parse_type, from)
        }, s2.to_string())?;

        let i5 = i64::parse_from(&parse_i64_from_decimal_string, s2.to_string())?;


        Ok(A {
            i1,
            i2
        })
    }
}