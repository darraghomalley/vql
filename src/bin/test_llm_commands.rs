use anyhow::Result;
use vql::tests::test_llm_ar_add;

fn main() -> Result<()> {
    println!("Testing LLM commands...");
    test_llm_ar_add()?;
    println!("Tests completed.");
    Ok(())
}