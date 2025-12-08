import childProcess from "node:child_process";
import fs from "node:fs/promises";

const proc = childProcess.spawnSync("wasm-as", [
  "--enable-multimemory",
  "-o",
  "-",
  "./day06.wast",
]);

if (proc.status) {
  console.error(proc.stderr.toString());
  process.exit(1);
}

const fileMemory = new WebAssembly.Memory({ initial: 1 });
const view = new Int32Array(fileMemory.buffer);
const file = await fs.readFile("../data/inputs/06.txt");
file.copy(view);

const wasmModule = await WebAssembly.instantiate(proc.stdout, {
  js: {
    log: console.log,
    file: fileMemory,
  },
});
const instance = wasmModule.instance;

const { part1, part2 } = instance.exports;
console.log({ part1: part1(), part2: part2() });
