use std::marker::PhantomData;

fn main() {
    println!("Hello, world!");
}

pub trait Parser<A> {
    fn parse(&self, str: String) -> Result<(A, String), String>;

    fn seq<That, B>(self, other: That) -> Seq<Self, That> where Self:Sized, That: Parser<B> {
        Seq { first: self, second: other }
    }

    fn map<B, F>(self, map: F) -> Map<Self, F, A, B> where Self: Sized, F: Fn(A) -> B {
        Map { parser: self, map, a: PhantomData, b: PhantomData }
    }
}

pub struct Succeed<A> {
    value: A,
}
impl <A: Clone> Parser<A> for Succeed<A> {
    fn parse(&self, str: String) -> Result<(A, String), String> {
        Ok((self.value.clone(), str))
    }
}

pub struct Fail<A> {
    error: String,
    phantom: PhantomData<A>,
}
impl <A> Parser<A> for Fail<A> {
    fn parse(&self, _str: String) -> Result<(A, String), String> {
      Err(self.error.clone())  
    }
}

pub struct Seq<L, R> {
    first: L,
    second: R,
}
impl <L, R, A, B> Parser<(A, B)> for Seq<L, R> where L: Parser<A>, R: Parser<B> {
    fn parse(&self, str: String) -> Result<((A, B), String), String> {
        let (a, str2) = self.first.parse(str)?;
        let (b, str3) = self.second.parse(str2)?;

        Ok(((a, b), str3))
    }
}

pub struct Map<P, F, A, B> where P: Parser<A>, F: Fn(A) -> B {
    parser: P,
    map: F,
    a: PhantomData<A>,
    b: PhantomData<B>,
}
impl <P, F, A, B> Parser<B> for Map<P, F, A, B> where P: Parser<A>, F: Fn(A) -> B {
    fn parse(&self, str: String) -> Result<(B, String), String> {
        let (a, str2) = self.parser.parse(str)?;

        Ok(((self.map)(a), str2))
    }
}

pub fn succeed<A>(value: A) -> Succeed<A> {
    Succeed { value }
}

pub fn fail<A>(error: String) -> Fail<A> {
    Fail { error, phantom: PhantomData }
}

pub fn map<P, F, A, B>(parser: P, map: F) -> impl Parser<B> where P: Parser<A>, F: Fn(A) -> B {
    Map { parser, map, a: PhantomData, b: PhantomData }
}

pub fn example() -> () {
    let s = succeed(42);
    let t = succeed(24);

    let u = s.seq(t);
    
    let v = u.map(|x| -> i32 { x.0 + x.1 });

    let w: (i32, String) = v.parse("example".to_string()).unwrap();

    println!("{}", w.0);
}
