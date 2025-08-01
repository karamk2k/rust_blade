
# Blade Test (WIP)

⚠️ **This is not a final project** — it's just an experimental prototype.

I'm currently exploring how to build a simple template engine in Rust.  
I'm **not a Rust developer** — this project is part of my learning journey to understand Rust better and try out ideas.

## 🧠 Why?

I wanted to:
- Learn Rust by building something practical.
- Explore how a template engine (like Blade in Laravel) might be structured.
- Practice working with strings, macros, and ownership in Rust.

## 🚧 Current Status

- Basic variable replacement using `{{ ... }}` works.
- Still experimenting with:
  - Filters like `{{ name | upper }}`
  - Syntax parsing
  - Better architecture
  - Error handling and edge cases
  - ✅ Expression evaluation with `+` now works:
  - Numbers: `{{ x + y }} → 30`

## 📌 Notes

- Expect messy or temporary code — this is just a personal playground for Rust experiments.
- I'm figuring things out as I go.
- Pull requests or suggestions are welcome, but keep in mind this is not meant for production use (yet? 😄).

---

Made with ❤️ by someone who still loves PHP.
