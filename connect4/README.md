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
From this screen, users
