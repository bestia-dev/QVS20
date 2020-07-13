# EXAMPLES AND DEVELOPMENT

## examples

Use this examples to learn how to use the qvs20 format and library.  

[comment]: # (lmake_md_to_doc_comments start A)

### 1. naive csv replacement

We start with the naive replacement of csv. No library involved.  
Super easy manually written code to write and read qvs20, but not "full standard".  
Remember that the goal of qvs20 is to import/export data from database tables.  
This naive approach can satisfy 99% of use-cases. It is extremely easily to write in any language.  
Most of the data is Strings or simple decimals. No complicated data types like datetime or hierarchical SubTable. They can always be represented as Strings. Just look at the JSON standard.  
Writing qvs20 is super easy. Not much to say here.  
Reading is a bit more complicated because the text needs to be parsed.  
The first approach is with the find() method.  
The second approach is with regex.  
`clear; cargo run --example example_01_naive_no_lib`  

[comment]: # (lmake_md_to_doc_comments end A)

[comment]: # (lmake_md_to_doc_comments start B)

### 2. using reader and writer library

It is wise to put repetitive code in a library. The first library is low level. Just read and write. Usage of libraries makes it possible to gradually enhance it with more complex data-types and methods as needed. The library encapsulate the code to parse and escape/unescape.  
`clear; cargo run --example example_02_reader_writer`  

[comment]: # (lmake_md_to_doc_comments end B)

[comment]: # (lmake_md_to_doc_comments start C)

### 3. using the qvs20_Table library

The qvs20 Table is in memory struct that contains all the data and their properties for the qvs20 format: table schema and table rows.  
The table structure is very flexible, because it is defined in runtime.
A lot of times the table is used just as an intermediary, and don't need a fixed Rust struct in compile time.  
It means that sometimes a change in the table does not dictate change in source code and consequently Rust slow compiling.  
I am sure it is not the most efficient way of working with large amounts of data, but the flexibility is sometimes more important.  
`clear; cargo run --example example_03_qvs20_table`  

[comment]: # (lmake_md_to_doc_comments end C)

### 4. Manually write an implementation for a struct

Having a struct it is fairly easy to write an implementation that reads and writes data to the qvs20 format. Most of the code is just boilerplate.  

### 5. Derive macro

Rust can codegen (code generation) the implementation in compile time, so the developer don't need to do it manually. It is achieved by a procedural derive macro in a separate crate.  

### Try it

```bash
# examples:
clear; cargo run --example example_01_read_to_table table01_simple_strings.qvs20
clear; cargo run --example example_01_read_to_table table02_int_decimal_float.qvs20
clear; cargo run --example example_01_read_to_table table03_sub_table.qvs20

clear; cargo run --example example_02_write_from_table table01_simple_strings.qvs20
clear; cargo run --example example_02_write_from_table table02_int_decimal_float.qvs20
clear; cargo run --example example_02_write_from_table table03_sub_table.qvs20

clear; cargo run --example example_03_json_population
clear; cargo run --example example_04_csv_customers
clear; cargo run --example example_05_qvs20_to_struct

clear; cargo run --example example_06_derive_manual
clear; cargo run --example example_07_derive_macro

# expand derive macro:
clear; cargo expand --example example_07_derive_macro
```

## examples, bin

```bash
# bin:
clear; cargo run --bin bin1_derive_manual
clear; cargo run --bin bin2_derive_macro

# examples:
clear; cargo run --example example_01_read_to_table table01_simple_strings.qvs20
clear; cargo run --example example_01_read_to_table table02_int_decimal_float.qvs20
clear; cargo run --example example_01_read_to_table table03_sub_table.qvs20

clear; cargo run --example example_02_write_from_table table01_simple_strings.qvs20
clear; cargo run --example example_02_write_from_table table02_int_decimal_float.qvs20
clear; cargo run --example example_02_write_from_table table03_sub_table.qvs20

clear; cargo run --example example_03_json_population
clear; cargo run --example example_04_csv_customers
clear; cargo run --example example_05_qvs20_to_struct
```

## cargo make - for non-trivial or multi-line commands

Cargo-make is a utility to write simple "scripts" to use in development.  
I use it to store in one place all the commands that are frequently in use in development.  <https://github.com/sagiegurari/cargo-make>

```bash
# install cargo plugin
cargo install --force cargo-make
# reads the Makefile.toml and shows the prepared scripts:
clear; cargo make
#for example
clear; cargo make publish_to_web - can have many steps to copy, upload, tag, stop/start server,...
```

## git

Git is the legendary distributed version-control system that is the base for github.  

```bash
# use ssh for git remote
# start the ssh-agent in the background and add your private key
eval $(ssh-agent -s)
ssh-add ~/.ssh/my_key
ssh-add -l

# clone or pull new commits
clear; git clone git@github.com:LucianoBestia/QVS20.git
clear; git pull

# locally always start development with a clear short term goal
# on a new branch
clear; git checkout -b name_your_goal
git status

# commit
clear; git add .; git commit -a -m your_msg
# when the change is small, no need for a new commit, just amend to the last one
clear; git add .;git commit --amend

# squash the branch to just one commit and merge to master
# we are on the branch
clear; git merge master
# resolve conflicts
clear; git checkout master
clear; git merge --squash your_branch_name
clear; git commit -m your_msg
clear; git push
clear; git branch -d your_branch_name

# tag before publish
git tag -f -a v${CARGO_MAKE_CRATE_VERSION} -m version_${CARGO_MAKE_CRATE_VERSION}
```

## cargo build, test

Cargo is the build tool for Rust and more. All the utilities around development are built in cargo or as plugins for the cargo command. Things like building, testing, running, benchmarking, formatting,...  


```bash
# build:
clear; cargo check
clear; cargo fmt
clear; cargo build
clear; cargo build --release

# tests:
clear; cargo test
clear; cargo test test_for_qvs20.rs

# expand derive macro:
clear; cargo run --bin bin1_derive_manual
clear; cargo expand --bin bin1_derive_manual

```

## other cargo plugins or linux commands

Cargo is enhanced by plugins that can be written by third party.  

```bash
cargo doc - create docs from doc comments
clear; cargo audit - alert problematic dependencies
clear; cargo tree - view the complete tree of transitive dependencies
clear; cargo crev verify - how good are your dependencies reviewed
# write a crev review
cargo crev crate review -u --skip-activity-check crate_name
# sync to local folders
rsync -a --info=progress2 --delete-after target/doc/ docs/ 
# sync to remote folders over ssh
rsync -e ssh -a --info=progress2 --delete-after /folder/ luciano_bestia@bestia.dev:/folder/
# run a remote sh script
ssh -tt -i ~/.ssh/ssh_key username@computer /folder/publish.sh

```

## my utilities for rust building

I wrote some utilities that I need for repeated steps in development.  
Than I write scripts in cargo-make that call this utilities.  

```bash
lmake_readme - copy data from cargo.toml to readme.md, then include text from readme.md into *.rs doc comments
lmake_semver --increment=minor - increment semver version in Cargo.toml for libraries
lmake_semver --increment=patch - increment semver version in Cargo.toml for libraries
lmake_version_from_date - change version in Cargo.toml to a "date/time version" (non semver) for binaries
lmake_lines_of_code - count lines of: code, comments, doc comments, tests, examples and include in README.md as shield badges
codetour_export_to_md - exports codetour to md files
```

## VSCode editor

I use the VSCode editor in Win10. It is an "microsoft open source" coding editor.  
The Rust project is in Debian "Buster" on WSL-1.  
In the WSL bash I cd to the Rust project folder and start VSCode with:  `code .`  
I am using these VSCode extensions:

- Remote development - (microsoft) <ms-vscode-remote.vscode-remote-extensionpack>
- Remote WSL - (microsoft) to have the VSCode GUI in Win10, but the project is in Debian <ms-vscode-remote.remote-wsl>
- crates - in Cargo.toml shows crate versions, has links to crate documentation and crev reviews <https://github.com/serayuzgur/crates> <serayuzgur.crates>
- rust-analyzer - for Rust autocompletion <matklad.rust-analyzer>
- Bracket Pair Colorizer - <CoenraadS.bracket-pair-colorizer>
- Better TOML - <bungcip.better-toml>
- Code Spell Checker - <streetsidesoftware.code-spell-checker>
- markdownlint - <DavidAnson.vscode-markdownlint>
- Bookmarks - <alefragnani.Bookmarks>
- vscode-icons - <vscode-icons-team.vscode-icons>
- CodeTour - a work in progress, but a nice promise <vsls-contrib.codetour>

## Win10 utilities

Some of my favorite windows utilities:  

- Total commander - best file manager for windows <https://www.ghisler.com/>
- Clipboard History - <https://www.outertech.com/en/clipboard-history>
- paint.net - for small work on bitmaps
- notepad++ - for text editing
- Inkscape - for vector graphics

## Debian utilities

Some of my favorite Debian/Linux utilities:  

- mc - midnight commander - terminal file manager
- rsync - file-copying tool
