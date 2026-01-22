import { execSync } from "child_process";

console.time("prettier");
execSync("prettier test.js");
console.timeEnd("prettier");

console.time("roshformatter");
execSync("roshformatter test.js js");
console.timeEnd("roshformatter");
