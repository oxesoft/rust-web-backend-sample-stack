Rust Web Backend Sample Stack
=====================================
This project aims to speed up the development of a web backend written in [Rust](https://www.rust-lang.org/) using [Rocket](https://rocket.rs/) + [Diesel](http://diesel.rs/), with the more common needs addressed. Both the Web Framework and ORM chosen are the most popular at the time of this implementation.

Notes
-----
1. This sample code uses SQLite as database backend however Diesel also supports MySQL and PostgreSQL;
2. Using this project as starting point *absolutely not* exempt you of reading [all](https://www.rust-lang.org/learn) of the material that you can before start coding using Rust;
3. VSCode is the suggested code editor and its launch configuration is included. You just need to install the extensions Rust + CodeLLDB from the Marketplace;
4. Because of Rocket, you need to use the nightly version of Rust using `rustup default nightly`;
5. In case of you need to debug you code using VSCode is highly recomended that you install Rust source code using the command `rustup component add rust-src` and make sure of setting "sourceMap" of [.vscode/launch.json](.vscode/launch.json) to map the original source location to the output of `$(rustc --print sysroot)/lib/rustlib/src/rust`. See the [CodeLLDB manual](https://github.com/vadimcn/vscode-lldb/blob/master/MANUAL.md#source-path-remapping);

Setup
-----
1. Assuming that Rust is installed in your system, follow the note 4 above;
2. Install Diesel CLI with `cargo install diesel_cli`;
3. Define an environment variable DATABASE_URL pointing to a file to be created (our SQLite database);
4. Call `diesel migration run`;
5. Call `cargo run` and open in your browser the URL shown at the console.
