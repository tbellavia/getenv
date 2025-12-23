# 🌱 getenv - Your Environment Variable Utility 🌟

[![Language: Rust](https://img.shields.io/badge/language-Rust-%23dea584?logo=rust)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

Welcome to **getenv**! 🚀  
A simple program written in **Rust** 🦀 to help you effortlessly retrieve values from environment files.

---

## 🎯 Key Features

1. 📂 **Handles `.env` files**: Getenv reads and processes `.env` files seamlessly.
2. 🔑 **Search by key**: Locate the value of an environment variable via its key.
3. 📋 **Copy to clipboard**: Automatically copies the variable's value to the clipboard for convenience.
4. ⚙️ **Custom file support**: Use a custom `.env` file or the default one in the working directory.

---

## 🛠️ Usage

### Command-line Interface

`getenv` searches for an environment variable in a `.env` file and copies its value to your system clipboard for easy reuse.

Here's how you can use it:

```bash
getenv [OPTIONS] --key <KEY>
```

### Options:
- **`-f, --filepath <FILEPATH>`**  
   Specify the path to the environment file containing the key.  
   *Default*: `.env`

- **`-k, --key <KEY>`**  
   The key to search for within the environment file. *(Required)*

- **`-h, --help`**  
   Display a help message summarizing the program's usage.

- **`-V, --version`**  
   Display the program version.

---

### Examples:

#### Default `.env` file:
If your working directory contains a file `.env`:
```
API_KEY=12345ABCDEF
DATABASE_URL=postgres://user:password@localhost:5432/dbname
```

You can retrieve the `API_KEY` value with:
```bash
getenv --key API_KEY
```
This copies `12345ABCDEF` to your system clipboard! 🎉

#### Custom environment file:
If the desired environment variables are stored in a file `config.env`, you can specify it explicitly:
```bash
getenv --filepath config.env --key DATABASE_URL
```
This copies the value of `DATABASE_URL` to the clipboard.

---

## 🚀 Installation and Build

### Prerequisites
- **Rust** must be installed. Install it [here](https://www.rust-lang.org/tools/install) if needed.

### Installation
Clone the repository and build the project:
```bash
git clone https://github.com/tbellavia/getenv.git
cd getenv
cargo build --release
```

---

## 📦 Contributions

Contributions are always welcome! 🛠 Feel free to open an **issue** or a **pull request** if you have ideas or improvements.

---

## 📄 License

This project is distributed under the [MIT License](LICENSE). Feel free to use, modify, and distribute it.
