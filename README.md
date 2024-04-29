# Cellular Automata ACW

## Automata

The automata software is contained within the `automata` directory. The core simulations are written in rust and the visualisation are written in python.

Build the rust code with `cargo build --release` and run the simulation using each binary in the `target/release` directory.

Execute the python code with a Jupyter notebook after generating the data files.

## Report

The report folder contains the LaTeX source for the report. The report is compiled using `latexmk` with the `shell-escape` option enabled.

Compile using `latexmk -shell-escape -pdf report.tex`.