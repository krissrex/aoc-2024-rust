# Template for solving Advent of Code puzzles in Rust with RustRover

Read the [blog post](https://blog.jetbrains.com/rust/2024/11/29/advent-of-code-in-rust-for-the-rest-of-us/) that explains the structure and rationale behind this template.

## Usage

1. Create a new project from the template repository:
   - Using GitHub’s templating feature: Simply click the Use this template [button](https://github.com/new?template_name=advent-of-code-rust-template&template_owner=bravit) on the repository page, create a new repository, and then open it in [RustRover](https://www.jetbrains.com/rust/) by selecting *File | New | Project From Version Control…*.
   -  Adding the template to RustRover: You can integrate the template directly into RustRover and use the regular New Project wizard.

2. Whenever you're ready to start solving a new day's puzzle:
   - Open the `bin` folder, copy and paste the `NN.rs` file into it, and give it the corresponding name (`01.rs`, `02.rs`, etc.).
   - In the `input` folder, create and fill the input data file (`01.txt`, `02.txt`, etc.).
   - Fill in the `DAY` constant in the freshly created file.
   - Run the current day's solution to check if it compiles (you can use the gutter icon next to the `main` function).
   - Fill in `<TEST-INPUT>`.
   - Write the expected answer for the test data in the `assert_eq` statement in *Part 1*.
   - Now you're ready to write your solution in the `part1` function (inside `main`).
   - Use `Shift+F10` (Win/Linux) or `Ctrl-R` (macOS) to re-run the same program.

3. When you're done with the first part of the puzzle, use folding to hide *Part 1*.

4. Uncomment *Part 2*, fill in the test data assertion, and start solving it.


## Use external libraries for the boring stuffCopy heading link
https://blog.jetbrains.com/rust/2024/11/29/advent-of-code-in-rust-for-the-rest-of-us/#use-external-libraries-for-the-boring-stuff

Using external libraries for the less exciting parts of the challenge allows you to focus your energy on solving the actual puzzles. Many Advent of Code tasks involve repetitive or tedious operations, like parsing input or handling edge cases. Instead of reinventing the wheel, leverage crates from Rust’s rich ecosystem to handle these tasks efficiently. Libraries like regex or itertools can simplify your code and save valuable time. The less time you spend on boilerplate, the more time you can dedicate to crafting elegant solutions.

Iterators are, in general, a very powerful concept and highly suitable for Advent of Code puzzles. I recommend reading through my three-part series on iterators to gain a deeper understanding beyond the basics, learn best practices for structuring code with iterators, and discover some lesser-known iterator functionalities that can be incredibly useful for the tasks at hand. The itertools crate adds even more functionality for processing serial data.

As for regular expressions, RustRover recognizes them in your code and makes it easy to compose and verify them. Simply select your regex, then press Alt+Enter (Windows/Linux) or ⌥↵ (macOS) to access helpful tools for editing and testing:



In some puzzles, you might need a simulation or an algorithm that relies on randomized input, making the rand crate a valuable addition to your Cargo.toml.

If your Advent of Code solutions involve advanced number manipulation, the num crate is your go-to solution. It extends Rust’s standard numeric capabilities with features like big integers, rationals, complex numbers, and more. Using num saves you from reinventing the wheel, so you can focus on solving the puzzle instead of building number utilities from scratch.

For puzzles involving linear algebra, matrices, or vector computations, the nalgebra crate is a lifesaver. Instead of writing custom code for mathematical operations, you can leverage this powerful library to handle even the most complex calculations. It’s perfect for tasks requiring geometric transformations or higher-dimensional data structures, allowing you to concentrate on the logic rather than the math.

When a problem involves graph structures – such as navigating nodes, finding shortest paths, or detecting cycles – the petgraph crate is a must. It provides efficient data structures and algorithms for working with graphs, eliminating the need to build your own graph utilities. With petgraph, you can tackle even the trickiest graph-related puzzles with ease. You can even visualize your graphs, which can be extremely helpful for finding solutions in some cases.

Want to bring a new dimension to your Advent of Code solutions? The bevy crate allows you to visualize solutions with a powerful game engine, making them not only functional but also visually engaging. Alternatively, for a lightweight and terminal-based approach, ratatui offers a simple way to build interactive user interfaces in the terminal. Whether you’re crafting stunning visualizations or enhancing interactivity, these crates add flair and fun to your solutions.

## Results

### 1
```text
src/bin/01.rs:52 took 232.5µs.
Result = 1506483
```

