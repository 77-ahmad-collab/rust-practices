# Installation

### On Linux

```bash
 curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
```

##### Get C compiler on Mac os

```bash
xcode-select --install
```

### On Windows

```bash
https://www.rust-lang.org/tools/install
```

### Updating rust

```bash
rustup update
```

#### To check whether you have Rust installed correctly, open a shell and enter this line:

```bash
rustc --version #<--Input -->
rustc x.y.z (abcabcabc yyyy-mm-dd) #<--OUTPUT LIKE THIS WILL COME-->
```

# Basic Program Implementation

### For Linux, macOS, and PowerShell on Windows, enter this

```bash
mkdir ~/projects
cd ~/projects
mkdir hello_world
cd hello_world
```

### For Windows CMD, enter this:

```bash
mkdir projects
cd projects
mkdir basics
cd basics
```

##### Now open the main.rs file you just created and enter the code in Listing 1-1.

###### Filename: main.rs

```bash
fn main() {
    println!("Hello, world!");
}
```

## Cargo

#### check whether Cargo is installed by entering the following into your terminal:

```bash
cargo --version
```

### Creating a project with cargo

```bash
cargo new hello_cargo
cd hello_cargo
```

cargo is simply a rust build system and package manager, you can think of npm if you coming from js background
Cargo Installation
