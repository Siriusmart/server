use actix_web::get;

const MESSAGE: &str = r#"Siriusmart's server

Paths:
`/`: This landing page.
`/ping`: Used by UptimeRobot to keep the server running on replit, special path to prevent making the number in stats inaccurate.

User API (`/api/user`)
    Accounts (`/accounts`):
    `/create/{username}/{password}/email`: Creates an account
    `/login/{identifier}/{password}`: Check if you got the correct password
    `/delete/{identifier}/{password}`: Deletes an account
    `/changepassword/{identifier}/{old password}/{new password}`: Changes to password of an account
    `/rename/{identifier}/{password}/{new username}`: Changes username of an account

The user API responds with text message, while the standard API responds with JSON

API V1 (`/api/v1`):
`/stats`: Stats page

    Request proxy (`/request-proxy`):
    `/normal`: Send a request to a webpage and returns the result (params: `url` the url to request)
    `/html`: Returns the result by sending a `window.parent.postMessage` to enable sites to send cross origin requests from client side, the content is html escaped (params: `url` the url to request`, any other params will be returned in the `postMessage` object)

    Accounts (`/accounts`):
    `/create/{username}/{password}/email`: Creates an account
    `/login/{identifier}/{password}`: Check if you got the correct password
    `/delete/{identifier}/{password}`: Deletes an account
    `/changepassword/{identifier}/{old password}/{new password}`: Changes to password of an account
    `/rename/{identifier}/{password}/{new username}`: Changes username of an account

---

Identifiers can be the user id, username or email addressusername or id

---

All account info are encrypted
A hash and salt is used in passwords

---

`.env` should contain:
USERID_SALT=string-any-length
USERNAME_SALT=string-any-length
PASSWORD_SALT=string-any-length
EMAIL_SALT=string-any-length
VERIFICATION_CODE_SALT=string-any-length
VERIFICATION_NOUNCE_SALT=string-any-length
ACCOUNTS_FILES_KEY=64-length-hex-string
USERNAME_FILES_KEY=64-length-hex-string
EMAIL_FILES_KEY=64-length-hex-string
VERIFICATION_FILES_KEY=64-length-hex-string

EMAIL_USERNAME=username@gmail.com (or other provider)
EMAIL_PASSWORD=password
SMTP_RELAY=smtp.gmail.com (or other smtp relay address)
SELF_ADDRESS=http://localhost:8080 (The server's address - localhost for dev)"#;

#[get("/")]
pub async fn root() -> &'static str {
    MESSAGE
}

#[get("/ping")]
pub async fn ping() -> &'static str {
    "Pong"
}
