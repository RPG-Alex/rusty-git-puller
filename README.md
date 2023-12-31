# Git Repo Updater
<a name="readme-top"></a>
<details>
    <summary>Contents</summary>
     <ol>
        <li>
            <a href="#introduction">Introduction</a>
        </li>
        <li>
            <a href="#usage">Usage</a>
        </li>
        <li>
       		<a href="#explanation">Explanation</a>
        </li>
        <li>
            <a href="#building">Building</a>
       	</li>
		<li>
            <a href="#contributing">Contributing</a>
       	</li>
       	<li>
            <a href="#license">License</a>
       	</li>
    </ol>
</details>


## Introduction
This is a simple Rust CLI program that recursively updates all git repositories in a specified directory up to a specified depth.

## Usage

```bash
cargo run <directory> <depth>
```
Replace <directory> with the directory you want to start searching from, and <depth> with the maximum depth you want to search.

For example, if you want to update all git repositories in ~/Projects up to a depth of 2 directories, you would run:

```bash
cargo run ~/Projects 2
```

## Explanation

The program starts by reading all the directories in the specified directory. For each directory, it checks if a .git directory exists within it. If a .git directory exists, it runs the git pull command in that directory. This process is done recursively up to the specified depth.
## Building

To build the program, you need to have Rust installed. You can download Rust from the official website.

Once you have Rust installed, you can build the program with:

```bash
cargo build --release
```

This will create an executable in the target/release directory.

## Contributing

Pull requests are welcome. For major changes, please open an issue first to discuss what you would like to change.

# License

MIT

<p align="center">[<a href="#readme-top">RETURN TO TOP</a>]</p>
