import init, { run_app } from './pkg/connect_game.js';
async function main() {
   await init('/pkg/connect_game_bg.wasm');
   run_app();
}
main()
