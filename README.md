# A Command Line Tool for Programming Contests

## About

This is a program to automate some of the annoying things you need to do while taking part in a programming contest, so you can focus on the fun part: solving the problems!

## Installation
- Installation requires the Rust package manager, `cargo`. To install, follow the directions [here](https://www.rust-lang.org/tools/install).
- `git clone https://github.com/Fr0benius/contest-ninja.git`
- `cd contest-ninja`
- `cargo install --path .`
- `contest-ninja -h` (to make sure executable is visible)
- If your shell can't find the executable, make sure `~/.cargo/bin` is in your `PATH`.
- Remove the source repo (or keep it, if you want to mess around with the code!): `cd ..; rm -rf contest-ninja`
- The test case downloader requires the [Competitive Companion](https://github.com/jmerle/competitive-companion) browser extension. If you want to automatically download problem tests, install it in your favorite browser.


## Usage
### Downloading test cases
- In your contest directory, run `contest-ninja download`
- Load the contest page in a browser. Click on the [Competitive Companion](https://github.com/jmerle/competitive-companion) button (it looks like a green plus sign).
- If that fails, the reason is that the extension doesn't support the website. In that case, try doing the same thing for the particular problem instead.
- You should see a log of downloaded problems in the terminal. Press Ctrl-C to exit the tool.
- The format is `<problem_name>-<count>.in` ond  `<problem_name>-<count>.out`
- NOTE: This tool will mercilessly overwrite any existing files with the same name!
- The the way the tool figures out the short problem name is by taking the first word in the "name" field. This works well for sides like Codeforces and AtCoder, which have single-letter problem names. For others, YMMV.

### Running tests
- The format is `contest-ninja test path/to/executable problem_name`
- For example, if you are testing problem `A`, with test-cases `A-1.in` and `A-2.in`, and the compiled executable is `run`, the invocation is `contest-ninja ./run A`
- You may add extra `A-*.in` and `A-*.out` pairs and the tool will test them as well.

## TODOS
This tool is in an extremely early stage. Some ideas for other features/improvements:

- Intelligently come up with short problem names for contests like GCJ.
- Allow configuration options:
  - Instead of dumping files in the current directory, add the option to find the directory for the particular contest
  - Configurable default for executable name
  - Put test case inputs and outputs in a single file. This would make it easier to add new tests.
- Instead of depending on the Competitive Companion extension, directly query the contest servers.
- Tool can compile problem solutions instead of requiring an existing executable
- Stress testing: given a wrong solution, slow solution, and a generator - find a failing test case (and add it to the problem's test suite)
- Library inlining
