## How to get it running:

### The easy way (If you have docker and docker-compose):

    cd connect4
    JWT_KEY=secureserverencryptionkey docker-compose up -d

By default, this method will attempt to run on port 80. Feel free to modify the ports if you cannot run on port 80

If you just want to try it out without installing anything,
try visiting http://connect4.woosaree.xyz


### If you don't have docker,
You'll need to have an instance of mongodb running.
You will also need wasm-pack, and rollup

    cd connect4/connect4-web

    # install external dependencies
    npm -i -g rollup

    rustup override set nightly
    rustup update

    cargo install wasm-pack
    mongod

    SERVER_URL=localhost:8000 wasm-pack build --target web
    rollup ./main.js --format iife --file ./pkg/bundle.js
    JWT_KEY=jkey DB_URL=mongodb://localhost:27017 cargo run --release --manifest-path=../connect4-server/Cargo.toml


Also, the last three of those steps are contained within a script called web

By default, this method will run mongodb on port 27017, and the rust
rocket web server on port 8000. If you change either port number, be
sure to also make sure that the environment variables SERVER\_URL and
DB\_URL reflect this.

## How to use it:
Great care was taken to ensure the application is as easy to use as
possible. When the user first loads up the application, they are
greeted with a friendly main screen.

Several things were taken into consideration to ensure this great user
experience. First, the site was designed to support the browsers back
button. To many sites break the back button, which is just
annoying. Secondly, we wanted users to be able to play from whenever
where ever, so it supports desktops, mobile devices, as well as the
ability to start a game and continue it from anywhere.

### Main Screen
From here, if the user is not signed in, they will be able to either
sign in, or create a game in offline mode, or access the settings
menu.

If the user is signed in, they will also have the settings button, but
they will also have a statistics button, a view current games, a view
past games, and a create game button.

### Settings
In here the user can toggle color blind mode, or go back to the main
menu. Color blind mode draws a charactor over the chip to indicate the
color.

### Statistics
From this screen, users can view their stats. It shows their wins,
losses, draws, and games played as well as games ongoing. The user can
also go back to the main menu.

### Past Games
From here, users can see a list of every game they have completed,
allowing them to analyse the board and learn from their mistakes. Or
just revel in their victory.

### Current Games
From here, users can see all games they have ongoing and they can
click on a game to continue playing.

### Create a game
From here, users will be presented with a series of menus to help them
create the game they want. They will select if it is a local or remote
multiplayer, or if they are playing against the AI. It also has them
select the game type of connect4/toot and otto. The menu options will
look slightly different if the user is not signed in, as there will be
no remote multiplayer option. When the user is done, it will send them
to the playing a game screen.

Note: For online multiplayer games, the first player will create a
game. The second player will join the game. The second player will
need to enter the game code, this can be found in the url of the
playing game. The url will look like
`domain.xyz/game/CODE?otherinformation` where the CODE is the game
code.


### Playing the game
From the playing game screen. The user will play games of connect4 or
toot and otto. The user clicks or touches a column to play a piece in
that column. For games where the user has the choice of multiple
pieces, the user can select a chip using the keyboard and typing the
letter displayed on the chip. Or by clicking or tapping on the chip
from the chip options part of the screen.

If the user is signed in. All moves will be sent to the server, this
means you can stop playing and resume later on any device as long as
you sign in.
