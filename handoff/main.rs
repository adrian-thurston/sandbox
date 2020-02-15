#[derive(Copy, Clone)]
struct Token
{
    i: i32,
    j: i32,
}

impl Default for Token
{
    fn default() -> Token
    {
        Token{ i: 1, j: 2 }
    }
}

struct Scanner
{
    optional: Option<Token>,
    other: Token,
}

impl Default for Scanner
{
    fn default() -> Scanner
    {
        Scanner{ optional: None, other: Default::default()  }
    }
}

impl Scanner
{
    fn peek( &mut self ) -> Token
    {
        match &self.optional {
            Some( token ) => { *token }
            None => {
                //let token = Token{ i: 2, j: 3 };
                //self.optional = Some( token );
                self.other
            }
        }
    }

    fn destroy( &mut self )
    {
        self.optional = None;
    }
}

fn main() {
    let mut s = Scanner{ optional: Some( Token{ i: 1, j: 2 } ), other: Token{ i: 5, j: 6 } };
    let t = s.peek();
    s.destroy();

    println!( "{} {}", t.i, t.j );
}
