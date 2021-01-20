use rocket::{Request, Outcome};

//traits signature implementation
trait CookieId {
    fn cookie_id()-> &'static str;   
}

//*****************************************************//
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdministratorCookie {
    pub userid: u32,
    pub username: String,
    pub display: Option<String>
}
impl CookieId for AdministratorCookie {
    fn cookie_id()-> &'static str {
        "unique id"
    }
} 