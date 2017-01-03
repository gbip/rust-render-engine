#Building the project
Building the project is very simple:
`cd awesome_project_dir
git clone https://github.com/gbip/rust-render-engine
cargo build`
If you don't have a nightly rust compiler, the compiler might complain because of the Serde crates that uses experimental features.
You can install a nightly compiler by entering the following command : `curl -s https://static.rust-lang.org/rustup.sh | sh -s -- --channel=nightly`

#Running the test
Running the test is as simple as typing `cargo test`.

