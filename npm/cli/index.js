#!/usr/bin/env node
import fs from "fs";
import { format } from "../native/index.js";

const [,, file, lang] = process.argv;

if (!file || !lang) {
  console.error("Usage: roshformatter <file> <lang>");
  process.exit(1);
}

const code = fs.readFileSync(file, "utf8");
process.stdout.write(format(code, lang));
