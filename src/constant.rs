pub const INIT: &str = "init";
pub const AUTH: &str = "auth";
pub const LIST: &str = "list";
pub const LOG: &str = "log";
pub const ENV: &str = "env";
pub const STATUS: &str = "status";
pub const START: &str = "start";
pub const DELETE: &str = "delete";
pub const PROJECT: &str = "project";
pub const ENVIRONMENT: &str = "environment";
pub const BRANCH: &str = "branch";
pub const APPLICATION: &str = "application";

pub const COL_NAME: &str = "NAME";
pub const COL_APPLICATION_NAME: &str = "APPLICATION NAME";
pub const COL_CREATED_AT: &str = "CREATED AT";
pub const COL_BRANCH: &str = "BRANCH";
pub const COL_STATUS: &str = "STATUS";
pub const COL_ENDPOINTS: &str = "ENDPOINTS";
pub const COL_ENDPOINT: &str = "ENDPOINT";
pub const COL_REGION: &str = "REGION";
pub const COL_APPLICATIONS: &str = "APPLICATIONS";
pub const COL_DATABASES: &str = "DATABASES";

pub const OUT_NONE: &str = "none";
pub const OUT_UNKNOWN: &str = "unknown";


pub const ASCII_NAME: &str = r#"
 .d88888b.  .d88888b. 888     88888888888888888888b.Y88b   d88P
d88P" "Y88bd88P" "Y88b888     888888       888   Y88bY88b d88P
888     888888     888888     888888       888    888 Y88o88P
888     888888     888Y88b   d88P8888888   888   d88P  Y888P
888     888888     888 Y88b d88P 888       8888888P"    888
888 Y8b 888888     888  Y88o88P  888       888 T88b     888
Y88b.Y8b88PY88b. .d88P   Y888P   888       888  T88b    888
 "Y888888"  "Y88888P"     Y8P    8888888888888   T88b   888
       Y8b
"#;

pub const AUTH_RESPONSE: &str = r#"
<script type="text/javascript" charset="utf-8">
    var hash = window.location.hash.split("=")[1].split("&")[0];
    var xmlHttp = new XMLHttpRequest();
    xmlHttp.open("GET", "http://localhost:10999/authorization/valid?access_token=" + hash, false);
    xmlHttp.send(null);
    xmlHttp.responseText;
</script>
"#;

pub const AUTH_URL: &str = "https://auth.qovery.com/login?client=MJ2SJpu12PxIzgmc5z5Y7N8m5MnaF7Y0\
&protocol=oauth2&response_type=id_token%20token&audience=https%3A%2F%2Fcore.qovery.com\
&redirect_uri=http://localhost:10999/authorization";
