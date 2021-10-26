use result::H3mResult;

pub mod result;

pub struct H3m {}

impl H3m {
    pub fn load() -> H3mResult<H3m> {
        Ok(H3m {})
    }

    pub fn save(&self) -> H3mResult<()> {
        Ok(())
    }
}
