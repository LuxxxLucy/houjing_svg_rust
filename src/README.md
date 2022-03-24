# Houjing

Change name to Sinothesis? Sytin?

Change a 竖版的侯景两个字。

This is heavily influenced by a lot of works, 
as well as a lot if parametric design.

such as Basbalt](), [Obsidian](https://eli.sohl.com/2020/04/14/obsidian.html), [loop-free](https://fitzgeraldnick.com/2020/01/13/synthesizing-loop-free-programs.html#program-representation).

## Disclaimer

Sometimes inevitably I will need to do some lower-level handling without a proper Rust library to use, and I will search online for a possible solution. Such cases are all denoted with a comment start with `DISCLAIMER`. Do a repo-wise text search will do. But there are not so many such cases in this repo.

## Structure of the code

-- **houjing**
    - **core**
        - geometry.rs: geometries, include the representation of number, shape
    - **synthesis**
        - solver.rs: warp the program and constraints and solve it by z3. (all z3 related work)
    - main.rs