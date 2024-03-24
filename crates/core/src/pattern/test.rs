trait Context<'a> {
    fn name(&'a self) -> &'a str;
}

type Func<'a, C: Context<'a>> = dyn Fn(C);

pub struct BuiltInFunction<'a, C: Context<'a>> {
    pub(crate) func: Box<Func<'a, C>>,
}

fn main() {}
