use lazy_static::lazy_static;
use user_agent_parser::UserAgentParser;

pub struct UserAgent {
    pub platform: Option<String>,
    pub browser: Option<String>,
}

impl UserAgent {
    pub fn new(user_agent: &str) -> Self {
        Self {
            platform: USER_AGENT_PARSER
                .parse_os(user_agent)
                .name
                .map(|v| v.to_string()),
            browser: USER_AGENT_PARSER
                .parse_product(user_agent)
                .name
                .map(|v| v.to_string()),
        }
    }
}

lazy_static! {
    static ref USER_AGENT_PARSER: UserAgentParser =
        UserAgentParser::from_str(include_str!("../../uap-core/regexes.yaml")).unwrap();
}
