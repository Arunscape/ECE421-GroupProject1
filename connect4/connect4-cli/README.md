# Connect 4 like games in rust

## AI
A lot of the AI stuff is based off this
http://blog.gamesolver.org/solving-connect-four/01-introduction/

## Server API
/signin: takes username and password, returns JWT

/refresh: takes in JWT returns new JWT

/creategame: takes in description of game, and JWT, returns gameid

/playmove: takes in description of move, gameid, and JWT, returns new gamestate

/getgame: takes in gameid, JWT, and returns gamestate
