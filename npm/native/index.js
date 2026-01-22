import { spawnSync } from "node:child_process";

export function format(code, lang) {
  const result = spawnSync(
    "roshformatter-core",
    [lang],
    { input: code, encoding: "utf8" }
  );

  if (result.status !== 0) {
    throw new Error(result.stderr);
  }

  return result.stdout;
}
