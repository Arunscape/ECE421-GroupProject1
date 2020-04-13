# Connect 4 like games in rust

## AI
A lot of the AI stuff is based off this
http://blog.gamesolver.org/solving-connect-four/01-introduction/

## /api/

### /api/refresh
- takes in:
-    jwt token from authentication header
- returns:
-     comms Refresh object with new valid JWT token


### /api/signin/u/p
- takes in:
-     username from api path u
-     password from api path p
- returns:
-     comms Signin object with new JWT token

### /api/creategame
- takes in:
-     jwt token from authentication header
-     lib game object from the request body
- returns:
-     comms GameDataResponse


### /api/playmove
- takes in:
-     jwt token from authentication header
-     comms PlayMove object from the request body
-         has the room id in it
-         has the column in it
-         has the lib chip description in it
- returns:
-     comms GameDataResponse


### /api/getgame/id
- takes in:
-     jwt token from authentication header
-     game id from the api path
- returns:
-     comms GameDataResponse

### /api/joingame/id
- takes in:
-     jwt token from authenticaion header
-     game id from the api path
-     a vector of players to register from the body
- returns:
-     a vector of possible player numbers, if the server failed to register a player it will have None as the player number in the repsonse

