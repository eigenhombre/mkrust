# mkrust

<img src="/mkrust.jpg" width="400">

![build](https://github.com/eigenhombre/mkrust/actions/workflows/build.yml/badge.svg)

Early version of my work-in-progress Rust project generator.  Generated projects have:

- README with MIT license, GitHub build badge, and commented-out artwork link
- Makefile with several commonly-used targets
- Trivial `main.rs` with a `hello world` message
- Example unit test
- Dockerfile for building a containerized version of the project
- GitHub Actions workflow for building and testing the project.

Since I work in several languages, and since various project tasks
such as building, releasing, etc. are conceptually the same, I like to
give `make` targets to common operations, and this template reflects
that approach.


# Building

1. Install Rust and Cargo
1. Define `RUSTPATH` in your environment to point to the directory
   where you want to create your Rust projects.
1. `cd` to that directory.
3. If you want newly-created binaries to go somewhere other than `$HOME/bin`,
   define `RUSTBIN` in your environment.  In either case, the directory must exist
   and be in your `$PATH`.
2. Clone this repo
3. `cd mkrust`
4. Optional: `make`  if you want to build a debug version first
5. `make install` to install the release binary in `$RUSTBIN`.

# Usage

1. `mkrust <project-name>`
2. `cd $RUSTPATH/<project-name>`
3. `make`
4. Try it out with `./target/debug/<project-name>`
5. `make install` to install the binary in `$RUSTBIN`.
6. After adding any unit tests, `make test` to run them.

# Docs

`make doc` will generate docs and open them in your browser.

# License

Copyright © 2023, John Jacobsen. MIT License.

# Disclaimer

THE SOFTWARE IS PROVIDED 'AS IS,' WITHOUT WARRANTY OF ANY KIND,
EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES
OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT
HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY,
WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
OTHER DEALINGS IN THE SOFTWARE.
