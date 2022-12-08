# region-cov-example

An example for Rust region coverage

In the [single source file](https://flying-sheep.github.io/region-cov-example/coverage/src/lib.rs.html),
all lines are covered, but one `if` branch is not:

![grafik](https://user-images.githubusercontent.com/291575/206483176-8d51afbe-4581-4076-a0eb-a8145a799d2f.png)

This is only captured in the “regions” metric in the [summary](https://flying-sheep.github.io/region-cov-example/):

![grafik](https://user-images.githubusercontent.com/291575/206483330-c2ef57e1-32c0-4198-aba4-b5e1847aadc0.png)

([the branch metric is broken](https://github.com/rust-lang/rust/issues/79649),
and I’m not sure what it even means: Shouldn’t “regions” be “branches, but actually correct and not ignoring in-line branchings”?)
