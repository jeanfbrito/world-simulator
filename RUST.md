# Rust Project Organization Best Practices

Rust provides a well-defined structure for organizing projects that promotes maintainability, clarity, and scalability. Following established conventions will make your codebase easier to navigate and collaborate on.

## **Standard Project Layout**

The canonical Rust project structure follows Cargo's conventions[1][2]:

```
my_project/
├── Cargo.lock
├── Cargo.toml
├── src/
│   ├── lib.rs           # Library crate root
│   ├── main.rs          # Binary crate root
│   └── bin/
│       ├── tool1.rs     # Additional executables
│       └── tool2/
│           ├── main.rs
│           └── utils.rs
├── tests/               # Integration tests
│   └── integration_test.rs
├── benches/             # Benchmarks
│   └── my_benchmark.rs
└── examples/            # Usage examples
    └── example.rs
```

### **Source Code Organization**

- **`src/main.rs`**: Entry point for binary crates (executables)
- **`src/lib.rs`**: Entry point for library crates
- **`src/bin/`**: Additional executables beyond the main binary
- Files should use `snake_case` naming[3]

## **Module System Best Practices**

### **Module Declaration**

Keep your entry points (`main.rs` or `lib.rs`) clean and focused on high-level orchestration[4]:

```rust
// src/main.rs
mod config;
mod server;
mod database;

use config::load_config;
use server::start_server;

fn main() {
    let config = load_config("config.toml").expect("Failed to load config");
    start_server(config);
}
```

### **Module Organization Strategies**

**Single Responsibility**: Each module should have a clear, focused purpose[5]. Avoid large "utility" modules that handle multiple unrelated concerns.

**Semantic Grouping**: Organize modules based on domain concepts rather than technical layers[6]. Instead of separating all models, views, and controllers, group related functionality together:

```
src/
├── user/
│   ├── mod.rs
│   ├── model.rs
│   └── handlers.rs
└── orders/
    ├── mod.rs
    ├── model.rs
    └── handlers.rs
```

### **Visibility and Re-exports**

Use `pub` keywords strategically to control your API surface[5]. Create clean module interfaces with re-exports in `mod.rs` files:

```rust
// src/user/mod.rs
mod model;
mod handlers;

pub use model::User;
pub use handlers::{create_user, get_user};
```

## **Workspaces for Multi-Crate Projects**

For larger projects, use Cargo workspaces to manage multiple related crates[7][8]:

```
my_workspace/
├── Cargo.toml          # Workspace root
├── Cargo.lock
└── crates/
    ├── core/           # Shared library
    │   ├── Cargo.toml
    │   └── src/
    ├── cli/            # Command-line tool
    │   ├── Cargo.toml
    │   └── src/
    └── server/         # Web server
        ├── Cargo.toml
        └── src/
```

Workspace root `Cargo.toml`:
```toml
[workspace]
resolver = "2"
members = ["crates/core", "crates/cli", "crates/server"]
```

### **Benefits of Workspaces**

- **Shared dependencies**: Single `Cargo.lock` ensures consistent versions[9]
- **Unified builds**: Build all crates together with `cargo build`
- **Code reuse**: Share common functionality between crates
- **Team organization**: Different teams can own different crates[7]

## **Testing Organization**

### **Unit Tests**

Place unit tests in the same file as the code they test[10][11]:

```rust
// src/math.rs
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(2, 3), 5);
    }
}
```

### **Integration Tests**

Place integration tests in the `tests/` directory[12][10]:

```
tests/
├── integration_test.rs
└── common/
    └── mod.rs          # Shared test utilities
```

Integration tests access your crate's public API like any external user would.

## **Error Handling Organization**

Structure errors systematically for maintainability[13]:

### **Library Crates**
Use `thiserror` for defining custom error types:

```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DatabaseError {
    #[error("Connection failed: {0}")]
    Connection(String),
    #[error("Query failed: {query}")]
    Query { query: String },
}
```

### **Application Crates**
Use `anyhow` for application-level error handling:

```rust
use anyhow::{Context, Result};

fn process_file(path: &str) -> Result<()> {
    std::fs::read_to_string(path)
        .context("Failed to read config file")?;
    Ok(())
}
```

## **Documentation Standards**

### **Crate-Level Documentation**

Document your crate's purpose and usage in `lib.rs`[14]:

```rust
//! # My Crate
//!
//! A brief description of what this crate does.
//!
//! ## Examples
//!
//! ```
//! use my_crate::MyStruct;
//! let instance = MyStruct::new();
//! ```
```

### **Function Documentation**

Follow standard documentation patterns[15]:

```rust
/// Calculates the sum of two numbers.
///
/// # Arguments
///
/// * `a` - The first number
/// * `b` - The second number
///
/// # Examples
///
/// ```
/// let result = add(2, 3);
/// assert_eq!(result, 5);
/// ```
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}
```

## **Naming Conventions**

Follow Rust's established naming patterns[16]:

- **Modules & files**: `snake_case`
- **Functions & variables**: `snake_case`
- **Structs, Enums, Traits**: `PascalCase`
- **Constants**: `SCREAMING_SNAKE_CASE`

## **Scalability Guidelines**

### **When to Split Modules**

- **File size**: When a file becomes unwieldy (typically 300-500 lines)
- **Functionality**: When distinct responsibilities emerge
- **Testing**: When you need focused unit tests for specific components

### **When to Use Workspaces**

- **Multiple binaries**: CLI tools, servers, and libraries in one project
- **Team boundaries**: Different teams maintaining different components
- **Compilation optimization**: Reduce rebuild times for large projects[17]

### **Progressive Organization**

Start simple and refactor as needed[6]. Begin with everything in `main.rs` or `lib.rs`, then extract modules as complexity grows. This approach prevents over-engineering early in development while maintaining clean organization as the project scales.

By following these practices, you'll create Rust projects that are not only functional but also maintainable, testable, and collaborative. The key is to leverage Rust's module system and tooling to create clear boundaries between components while maintaining flexibility for future growth.

Sources
[1] Project Layout - The Cargo Book https://web.mit.edu/rust-lang_v1.25/arch/amd64_ubuntu1404/share/doc/rust/html/cargo/guide/project-layout.html
[2] Package Layout - The Cargo Book https://doc.rust-lang.org/cargo/guide/project-layout.html
[3] Rust Project Structure and Best Practices for Clean ... https://www.djamware.com/post/68b2c7c451ce620c6f5efc56/rust-project-structure-and-best-practices-for-clean-scalable-code
[4] How to Structure a Rust Project Idiomatically https://dev.to/sgchris/how-to-structure-a-rust-project-idiomatically-500k
[5] Modular Design in Rust Best Practices and Importance https://moldstud.com/articles/p-modular-design-in-rust-best-practices-and-importance
[6] What folder structure do you maintain on Rust projects? https://www.reddit.com/r/learnrust/comments/1645w1n/what_folder_structure_do_you_maintain_on_rust/
[7] How to Organize a Large-Scale Rust Project Effectively https://leapcell.io/blog/how-to-organize-a-large-scale-rust-project-effectively
[8] Cargo Workspaces - The Rust Programming Language https://doc.rust-lang.org/book/ch14-03-cargo-workspaces.html
[9] Rust Workspaces: A guide to managing your code better https://fullstackwriter.dev/post/rust-workspaces-a-guide-to-managing-your-code-better?category=rust
[10] Test Organization - The Rust Programming Language https://doc.rust-lang.org/book/ch11-03-test-organization.html
[11] Test Organization - The Rust Programming Language https://web.mit.edu/rust-lang_v1.25/arch/amd64_ubuntu1404/share/doc/rust/html/book/second-edition/ch11-03-test-organization.html
[12] Writing Tests in Rust: Familiar and Fast https://www.woodruff.dev/writing-tests-in-rust-familiar-and-fast/
[13] Error Handling for Large Rust Projects - Best Practice in ... https://greptime.com/blogs/2024-05-07-error-rust
[14] Rust Documentation Conventions - Project Mu https://microsoft.github.io/mu/CodeDevelopment/rust_documentation_conventions/
[15] Documentation - The Rust Programming Language https://web.mit.edu/rust-lang_v1.25/arch/amd64_ubuntu1404/share/doc/rust/html/book/first-edition/documentation.html
[16] Rust Coding Conventions and Learning Resources https://buildsoftwaresystems.com/post/rust-coding-conventions-learning-resources/
[17] Workspace - Rust Project Primer https://rustprojectprimer.com/organization/workspace.html
[18] Cargo, Crates and Basic Project Structure - Learning Rust https://learning-rust.github.io/docs/cargo-crates-and-basic-project-structure/
[19] Organizing code & project structure https://rust-classes.com/chapter_4_3
[20] Best way to organize structure / modules in project - help https://users.rust-lang.org/t/best-way-to-organize-structure-modules-in-project/114883
[21] Hello, Cargo! - The Rust Programming Language https://doc.rust-lang.org/book/ch01-03-hello-cargo.html
[22] Project structure in Rust https://www.reddit.com/r/rust/comments/185pdyr/project_structure_in_rust/
[23] Workspace Setup - Rust Full Stack Workshop - BcnRust https://bcnrust.github.io/devbcn-workshop/backend/01_workspace_setup.html
[24] Rust: Project structure example step by step https://dev.to/ghost/rust-project-structure-example-step-by-step-3ee
[25] Managing Growing Projects with Packages, Crates, and ... https://doc.rust-lang.org/book/ch07-00-managing-growing-projects-with-packages-crates-and-modules.html
[26] Workspaces best practices, code organization : r/rust https://www.reddit.com/r/rust/comments/nva157/workspaces_best_practices_code_organization/
[27] Project Structure https://v2.tauri.app/start/project-structure/
[28] Best practices around organising code bases? - help https://users.rust-lang.org/t/best-practices-around-organising-code-bases/88694
[29] Workspaces - The Cargo Book https://doc.rust-lang.org/cargo/reference/workspaces.html
[30] rust folder structure organisation https://stackoverflow.com/questions/77786227/rust-folder-structure-organisation
[31] Rust modules confusion when there is main.rs and lib.rs https://stackoverflow.com/questions/57756927/rust-modules-confusion-when-there-is-main-rs-and-lib-rs
[32] Rust Module System Encourages Poor Practices ... https://dmitryfrank.com/articles/rust_module_system_encourages_bad_practices
[33] Main.rs and lib.rs at same level - help https://users.rust-lang.org/t/main-rs-and-lib-rs-at-same-level/42499
[34] Rust's modules and project organization: best practices ... https://www.reddit.com/r/rust/comments/alsph9/rusts_modules_and_project_organization_best/
[35] Crate Layout Best Practices: lib.rs, mod.rs, and src/bin https://dev.to/sgchris/crate-layout-best-practices-librs-modrs-and-srcbin-4abd
[36] How to define a separate lib.rs in the same folder with main.rs https://users.rust-lang.org/t/how-to-define-a-separate-lib-rs-in-the-same-folder-with-main-rs/106449
[37] Rust's Module System Explained: A Complete Guide to ... https://dev.to/ajtech0001/rusts-module-system-explained-a-complete-guide-to-organizing-your-code-3i8i
[38] thoughts on the src/main.rs and src/lib.rs pattern #167 https://github.com/rust-lang/api-guidelines/discussions/167
[39] Rust Project Folder Structure (Lib/Tests/etc.) - code review https://users.rust-lang.org/t/rust-project-folder-structure-lib-tests-etc/70067
[40] What goes into lib.rs? : r/learnrust https://www.reddit.com/r/learnrust/comments/10wgz9t/what_goes_into_librs/
[41] How It Works: Rust's Module System Finally Explained https://confidence.sh/blog/rust-module-system-explained/
[42] Can the project's src/bin directory contain subdirectories? https://users.rust-lang.org/t/can-the-projects-src-bin-directory-contain-subdirectories/122800
[43] Rust Error Handling - Best Practices https://www.youtube.com/watch?v=j-VQCYP7wyw
[44] Error handling in Rust: A comprehensive tutorial https://blog.logrocket.com/error-handling-rust/
[45] Test Organization - The Rust Programming Language https://doc.rust-lang.org/stable/book/ch11-03-test-organization.html?highlight=import+a+crate
[46] Error Handling - The Rust Programming Language https://doc.rust-lang.org/book/ch09-00-error-handling.html
[47] are there established conventions? What do you use? : r/rust https://www.reddit.com/r/rust/comments/tjeoc0/standard_documentation_format_are_there/
[48] Error handling - good/best practices : r/rust https://www.reddit.com/r/rust/comments/1bb7dco/error_handling_goodbest_practices/
[49] Test Integration and organizing tests - Full Crash Rust ... https://www.youtube.com/watch?v=MiLp0InGOkI
[50] Making Great Docs with Rustdoc - Tangram Visions Blog https://www.tangramvision.com/blog/making-great-docs-with-rustdoc
[51] Practical guide to Error Handling in Rust - Dev State https://dev-state.com/posts/error_handling/
[52] Complete Guide To Testing Code In Rust https://zerotomastery.io/blog/complete-guide-to-testing-code-in-rust/
[53] Documentation and testing - Rust for the Polyglot ... https://www.chiark.greenend.org.uk/~ianmdlvl/rust-polyglot/rustdoc.html
[54] Understanding Best Practices for propagating different ... https://users.rust-lang.org/t/understanding-best-practices-for-propagating-different-errors-in-libraries/120269
[55] Best way to organise tests in Rust https://www.reddit.com/r/rust/comments/qk77iu/best_way_to_organise_tests_in_rust/
