## How to get it running:

### The easy way (If you have docker and docker-compose):

    cd connect4
    docker-compose up -d

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
    
    DB_URL=mongodb://localhost:27017 cargo run --release --manifest-path=../connect4-server/Cargo.toml

By default, this method will run mongodb on port 27017, and the rust rocket web server on port 8000. If you change either port number, be sure to also make sure that the environment variables SERVER_URL and DB_URL reflect this.





