#Building the project
Building the project is very simple:

```bash
cd awesome_project_dir
git clone https://github.com/gbip/rust-render-engine
cargo build
```



If you don't have a nightly rust compiler, the compiler might complain because of the Serde crates that uses experimental features.
You can install a nightly compiler by entering the following command :
`curl -s https://static.rust-lang.org/rustup.sh | sh -s -- --channel=nightly`


##About the compilation warning
It is totally normal because we are still basically running a hello world program, so the compiler is complaining that I have wrotten a bunch of useless files, but they are not. Please make abstraction of them, they will be cleaned in their due time.


#Running the test
Running the test is as simple as typing `cargo test`.

#Example 
```
target/debug/render-engine -r models/scene1.json -w test.png
```
