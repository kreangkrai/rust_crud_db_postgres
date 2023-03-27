pub struct DB{
    pub url: &'static str,
}
impl DB {
    pub fn url() ->Self{
        DB {url:"postgres://postgres:Meeci50026@localhost:5432/Mee"}
    }
}