import init, { run } from './pkg/yew_debug.js';
async function main() {
   await init('pkg/yew_debug_bg.wasm');
   run();
}
main()
