# projlyzer
A CLI for analyzing the programming languages and how much code written in a project.\
New features are on the way...
## Example Screenshot
<img src="https://i.ibb.co/9H5fwvK/projlyzer-example.png" alt="Example screenshot" />

## Build and use
Note: The commmands below are for Unixish systems. I don't know the Windows way to do that since I'm a MacOS user and Linux lover. (Please add the Windows way by opening a PR, you Windows user!)
```bash
# Clone the repo and enter to the directory
$ git clone https://github.com/electromeow/projlyzer.git
$ cd projlyzer/
# Compile
$ cargo build --release
# Optionally add to the path.
$ sudo mv ./target/release/projlyzer /usr/local/bin/projlyzer
$ projlyzer /path/to/project/
# Or use without adding to the path
$ ./target/release/projlyzer /path/to/project/
```
## LICENSE
This project is distributed under MIT license.