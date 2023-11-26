# mkrust

<img src="/mkrust.jpg" width="400">

![build](https://github.com/eigenhombre/mkrust/actions/workflows/build.yml/badge.svg)

Early version of my work-in-progress Rust project generator.

Probably don't use this.  I'm not very good at Rust yet.

Since I work in several languages, and since various project tasks such as building,
releasing, etc. are conceptually the same, I like to give `make` targets to common operations,
and this template reflects that approach.

# Building

1. Install Rust and Cargo
2. Clone this repo
3. `make install`

# Usage

1. Define `RUSTPATH` in your environment to point to the directory
   where you want to create your Rust projects.
2. `mkrust <project-name>`
3. `cd $RUSTPATH/<project-name>`
4. `make`
5. Try it out with `./target/debug/<project-name>`
6. `make install` to install the binary in `$HOME/bin`.
7. After adding any unit tests, `make test` to run them.

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
