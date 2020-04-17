# Connect 4 Lib

## ![Connect 4 Lib](https://github.com/Arunscape/ECE421-GroupProject1/workflows/Connect%204%20Lib/badge.svg)

## AI
A lot of the AI stuff is based off this
http://blog.gamesolver.org/solving-connect-four/01-introduction/

## Server API
/signin: takes username and password, returns JWT

/refresh: takes in JWT returns new JWT

/creategame: takes in description of game, and JWT, returns gameid

/playmove: takes in description of move, gameid, and JWT, returns new gamestate

/getgame: takes in gameid, JWT, and returns gamestate
