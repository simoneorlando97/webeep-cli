pub use http_client::HttpClient;
pub use html_parser::*;

mod http_client;
mod html_parser;

pub const WE_BEEP_URL : &str = "https://webeep.polimi.it/auth/shibboleth/index.php";
pub const AUNICA_URL : &str = "https://aunicalogin.polimi.it";
pub const HIDDEN_POST_URL : &str = "https://webeep.polimi.it/Shibboleth.sso/SAML2/POST";
pub const WE_BEEP_COURSE_URL : &str = "	https://webeep.polimi.it/course/view.php";





