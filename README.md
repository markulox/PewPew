# PewPew
A CLI tool for sending multiple API requests simultaneously. Since the current tools for testing API calls can not perform a heavy request calling (plus, with the opportunity to practice my Rust programming skills), I developed the command line tools to simultaneously send requests or multiple requests by spawning threads to execute each parallel API request. 

Some features that I've implemented are
- Obviously, perform multiple API requests in parallel to simulate numerous users calling the API.
- Send a body as raw text or form with the API request.
- Send additional headers.
- Record the latency time of each API request and export it as a graph.

Not yet available, but it's going to be soon...
- Cookies management.
- Generate random delay time from Normal Distribution.

## Installation
### Windows
1. Download a compiled binary from here: 
2. Run using the command prompt by calling `pewpew.exe <URL>` from the directory where you place the binary.

If you want to make this command available anywhere, please proceed with the following steps.
3. Opening Settings > About > Advance system settings >  Environment Variables
4. Click "Path" on the User variables one (Upper section) and click "Edit."
5. Click "New" and enter the path that stores pewpew.exe. (Note: The directory path needs to start from 'C:\') Then click OK
6. Re-open the command prompt, and you are ready to go.

### Linux
1. Simply download and copy `pewpew` to `~/.local/bin` if you want to install for you only
2. For the System-wide installation, you can copy to `/usr/bin` instead.

### macOS
1. Download a compiled binary from here: 
2. Run using the terminal by calling `./pewpew <URL>` from the directory where you place the binary.

If you want to make this command available anywhere, run the following script below
3. Opening Settings > About > Advance system settings >  Environment Variables
4. Click "Path" on the User variables one (Upper section) and click "Edit."
5. Click "New" and enter the path that stores pewpew.exe. (Note: The directory path needs to start from 'C:\') Then click OK
6. Re-open the command prompt, and you are ready to go.

I'm planning for making some simple installation script for this platform for easier installation.

## Manual Build from Source
```bash
$ git clone https://github.com/nopphonyel/PewPew.git
$ cd PewPew
$ cargo install --path . 
$ cargo build -r
```
After the build process, the binary is located at `./target/release`