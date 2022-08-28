# Siriusmart's server

> ### Paths:
> * `/`: This landing page.
> * `/ping`: Used by UptimeRobot to keep the server running on replit, special path to prevent making the number in stats inaccurate.

> ### Api (v1):
> * `/api/v1/stats`: Stats page
> * `/api/v1/utils/request-proxy/normal`, `/api/v1/utils/request-proxy/html`: Sends a request on behalve of you, /normal returns the response recieved "as is" in `object.content` and /html sends a "message" in html to pass the content out of the iframe. Url params will be included in the response.
