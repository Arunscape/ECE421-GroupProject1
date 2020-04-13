# Connect 4 like games in rust

## TODO

## Bugs in web client
 - Holding down the mouse will result in 60 clicks per second (about)
 - Local AI moves are not shown as animated, as the delta is so long for the first frame, they hit the bottom
 - Local AI doesn't render immediately when page loads

## Refactoring Game Object
 - Only do work when necessary (not continuously rendering)
 - Make it easy to use with asycn functions (pass through callbacks or have an async handler or...)
 - Separate the game component and the game object.
    - _game object_ (deals with the canvas directly)
    - _game component_ we throw into yew (wraps around game object)
        - creates the canvas 
        - by default take 100% available space
        - basically is just the canvas
    - _game screen_ which contains game component


## User stories
- As a connect4 developer I would like a reusable component to display games
    - display several canvases/games at once (different ids)
    - screen that is nice for playing a game. (The entire screen for one canvas)
    - not use all my CPU because we're rendering 60 times a second doing nothing (i.e. only re-render when things change)
    - not to block the rendering when busy functions like the AI thinking, or network requests are happening
    - be able to communicate with the network, but not fail for local games that are offline without a network.
        - i.e. ai game should work without access to the broad internet
    - game should be configurable so the component we make, the game description can be passed as props 
        - props: 
            - game id
            - game object
            - whether or not it is an active game, or an image of a game. (so a user can preview an in-progress game, and game history)
    - loading spinner when waiting for user input

# Todo (arun)
    - create games and join games menu
    when we click remote
    menu: join/create
    join: txtfield: game id
    create: send create request to server