Running instance of the server [*here*](https://server.siriusmart.repl.co).

```
Siriusmart's server

Paths:
`/`: This landing page.
`/ping`: Used by UptimeRobot to keep the server running on replit, special path to prevent making the number in stats inaccurate.

Api V1 (`/api/v1`):
`/stats`: Stats page

    Request proxy (`/request-proxy`):
    `/normal`: Send a request to a webpage and returns the result (params: `url` the url to request)
    `/html`: Returns the result by sending a `window.parent.postMessage` to enable sites to send cross origin requests from client side, the content is html escaped (params: `url` the url to request`, any other params will be returned in the `postMessage` object)

    Accounts (`/accounts`):
    `/create/{username}/{password}`: Creates an account
    `/login/{username or id}/{password}`: Check if you got the correct password
    `/delete/{username or id}/{password}`: Deletes an account
    `/changepassword/{username or id}/{old password}/{new password}`: Changes to password of an account
    `/rename/{username or id}/{password}/{new username}`: Changes username of an account

All account info are encrypted
A hash and salt is used in passwords

`.env` should contain:
USERID_SALT=SALT_OF_ANY_LENGTH
USERNAME_SALT=SALT_OF_ANY_LENGTH
PASSWORD_SALT=SALT_OF_ANY_LENGTH
ACCOUNTS_FILES_KEY=64_CHARS_LONG_HASH
USERNAME_FILES_KEY=64_CHARS_LONG_HASH```
