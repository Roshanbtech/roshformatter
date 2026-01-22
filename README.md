# roshformatter

A deterministic, security-first code formatter designed for CI and regulated environments.

## Why roshformatter?
- No eval
- No plugins
- No network access
- Deterministic output
- Rust-based core
- Safe on untrusted input

## Installation

```bash
npm install -g roshformatter

Usage
Format JavaScript
roshformatter src/index.js js

Format JSON
roshformatter config.json json

Example

Input

function  test( ){console.log("hi")}


Output

function test() {
  console.log("hi");
}

CI Usage
roshformatter src/**/*.js js
git diff --exit-code

Supported Languages

JavaScript

JSON

Security

See SECURITY.md

License

MIT


This alone raises credibility **massively**.

---
