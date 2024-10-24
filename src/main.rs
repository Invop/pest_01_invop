use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "./grammar.pest"]
pub struct Grammar;


fn main() -> anyhow::Result<()> {
    let got = Grammar::parse(Rule::file,"-123.89,-78\n")?;
    println!("{:?}",got);
    Ok(())
}
