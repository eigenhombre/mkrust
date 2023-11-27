use chrono::Datelike;
use std::env;
use std::process::{exit, Command};

fn main() {
    // Get the project name from command line arguments
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: {} <project_name>", args[0]);
        exit(1);
    }

    let project_name = &args[1];
    let rustpath = rustpath();

    // Construct the full path for the new project
    let project_path = projpath(&rustpath, &project_name);
    // Run cargo new in the specified directory:
    runcargo(project_name, rustpath, &project_path);
    write_sample_test(&project_path);
    write_gitignore(project_name, &project_path);
    write_readme(project_name, &project_path);
    write_makefile(project_name, &project_path);
    write_license(&project_path);
    run_git_init(&project_path);
}

fn write_sample_test(project_path: &String) {
    // Construct the full path for the sample test file
    let test_path = format!("{}/tests/test.rs", project_path);

    // Write the sample test file
    let test = format!(
        "#[cfg(test)]

mod tests {{
    #[test]
    fn internal() {{
        assert_eq!(1, 1);
    }}
}}
"
    );
    write_file_or_die(test_path, test, "tests/test.rs");
}

fn run_git_init(project_path: &String) {
    // Run git init in the specified directory
    let output = Command::new("git")
        .arg("init")
        .current_dir(&project_path)
        .output();

    // Check if git init was successful
    match output {
        Ok(_) => println!("Initialized git repository in '{}'.", project_path),
        Err(err) => {
            eprintln!("Error initializing git repository: {:?}", err);
            exit(1);
        }
    }
}

fn license_text(year: u16) -> String {
    format!(
        "Copyright © {}, John Jacobsen. MIT License.

# Disclaimer

THE SOFTWARE IS PROVIDED 'AS IS,' WITHOUT WARRANTY OF ANY KIND,
EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES
OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT
HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY,
WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
OTHER DEALINGS IN THE SOFTWARE.
",
        year
    )
}

fn create_parent_dir(file_path: &String) {
    // Get the parent directory of the file
    let parent_dir = std::path::Path::new(&file_path)
        .parent()
        .unwrap()
        .to_str()
        .unwrap();

    // Create the parent directory if it doesn't exist
    if !std::path::Path::new(&parent_dir).exists() {
        match std::fs::create_dir_all(&parent_dir) {
            Ok(_) => (),
            Err(err) => {
                eprintln!("Error creating directory '{}': {:?}", parent_dir, err);
                exit(1);
            }
        }
    }
}

fn write_file_or_die(file_path: String, contents: String, file_name: &str) {
    create_parent_dir(&file_path);
    match std::fs::write(&file_path, &contents) {
        Ok(_) => println!("Created {} file.", file_name),
        Err(err) => {
            eprintln!("Error creating {} file: {:?}", file_name, err);
            exit(1);
        }
    }
}

fn write_license(project_path: &String) {
    // Construct the license text
    let license = license_text(year());
    let license_path = format!("{}/LICENSE", project_path);
    write_file_or_die(license_path, license, "LICENSE");
}

fn write_makefile(project_name: &String, project_path: &String) {
    // Construct the full path for the Makefile
    let makefile_path = format!("{}/Makefile", project_path);

    // Write the Makefile
    let makefile = format!(
        "PROJECT_NAME := {}
# If RUSTBIN is set, use it as the rust binary directory:
ifdef RUSTBIN
\tBINDIR := ${{RUSTBIN}}
else
\tBINDIR := ${{HOME}}/bin
endif

.PHONY: all
all: test build

.PHONY: build
build:
\tcargo build

.PHONY: run
run:
\tcargo run

.PHONY: test
test:
\tcargo test

.PHONY: clean
clean:
\tcargo clean

.PHONY: doc
doc:
\tcargo doc --no-deps --open

.PHONY: fmt
fmt:
\tcargo fmt

.PHONY: deps
deps:
\tcargo fetch

fast: build test doc fmt

RELEASED := target/release/${{PROJECT_NAME}}

${{RELEASED}}: src/*.rs
\tcargo build --release

.PHONY: release
release: ${{RELEASED}}

.PHONY: install
install: release
# Use cp rather than mv to avoid signing errors on Mac:
\tmv ${{RELEASED}} ${{BINDIR}}/${{PROJECT_NAME}}
",
        project_name
    );
    write_file_or_die(makefile_path, makefile, "Makefile");
}

fn write_readme(project_name: &String, project_path: &String) {
    // Construct the full path for the README.md file
    let readme_path = format!("{}/README.md", project_path);

    // Write the README.md file
    let readme = format!(
        "# {}

<!--- <img src='/{}.jpg' width='400'> --->

![build](https://github.com/eigenhombre/{}/actions/workflows/build.yml/badge.svg)

# Introduction

FIXME

# License
{}
",
        project_name,
        project_name,
        project_name,
        license_text(year())
    );
    write_file_or_die(readme_path, readme, "README.md");
}

fn write_gitignore(project_name: &String, project_path: &String) {
    // Construct the full path for the .gitignore file
    let gitignore_path = format!("{}/.gitignore", project_path);

    // Write the .gitignore file
    let gitignore = format!("target\n{}.rs.bk\n", project_name);
    write_file_or_die(gitignore_path, gitignore, ".gitignore");
}

fn runcargo(project_name: &String, rustpath: String, project_path: &String) {
    // Run cargo new in the specified directory
    let output = Command::new("cargo")
        .arg("new")
        .arg(project_name)
        .current_dir(&rustpath)
        .output();

    // Check if cargo new was successful
    match output {
        Ok(_) => println!("Project '{}' created in '{}'.", project_name, project_path),
        Err(err) => {
            eprintln!("Error creating project: {:?}", err);
            exit(1);
        }
    }
}

fn rustpath() -> String {
    // Get the RUSTPATH environment variable
    let rustpath = match env::var("RUSTPATH") {
        Ok(path) => path,
        Err(_) => {
            eprintln!("Error: RUSTPATH environment variable not set.");
            exit(1);
        }
    };
    rustpath
}

fn projpath(rustpath: &String, project_name: &String) -> String {
    // Get the RUSTPATH environment variable
    // let rustpath = rustpath();

    // Construct the full path for the new project
    let project_path = format!("{}/{}", rustpath, project_name);
    project_path
}

// Return year as integer:
fn year() -> u16 {
    let now = chrono::Local::now();
    now.year() as u16
}
