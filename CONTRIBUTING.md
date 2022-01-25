# Contributing

Thank you for your interest in making contributions to Mina-rs! Start by taking a look at this page for overall information on repository workflow and standards.

Looking for a good place to start contributing? How about checking out some [good first issues](https://github.com/ChainSafe/mina-rs/issues?q=is%3Aissue+is%3Aopen+label%3A%22good+first+issue%22)

Please follow standard github best practices: fork the repo, branch off from the tip of [main](https://github.com/ChainSafe/mina-rs/tree/main), make some commits, and submit a pull request to develop. See the open issues for things we need help with!

## Getting started

To start contributing to mina-rs, following is a cursory guideline on how to make the process of making changes more efficient for the contributer and the maintainer.

* File an issue for the change you want to make. This way we can track the why of the change. Get consensus from community for the change.
* Clone the project and perform a fresh build. Create a branch with the naming "feature/issue-number.
* Ensure that the PR only changes the parts of code which implements/solves the issue. This includes running the linter (cargo fmt) and removing any extra spaces and any formatting that accidentally were made by the code editor in use.
* If your PR has changes that should also reflect in README.md, please update that as well.
* Document non obvious changes and the why of your changes if it's unclear.
* If you are adding a public API, add the documentation as well.
* Increase the version numbers in `Cargo.toml` file if needed and the `README.md` to the new version that this Pull Request would represent. The versioning scheme we use is SemVer.
* Update the CHANGELOG.md to reflect the change if applicable.

More details: https://github.community/t/best-practices-for-pull-requests/10195

To pull in updates from the origin repo, run

* `git fetch upstream`
* `git rebase upstream/main` (or whatever branch you want)

Since some dependencies are not under our control, a third party may break our build, in which case we can fall back on dep ensure (or make deps). Even for dependencies under our control, dep helps us to keep multiple repos in sync as they evolve. Anything with an executable, such as apps, tools, and the core, should use dep.

Libraries need not follow the model strictly, but would be wise to.

## Development Procedure:

- the latest state of development is on main
- main must never fail make test
- main should not fail make lint
- no --force onto main (except when reverting a broken commit, which should seldom happen)
- create a development branch either on github.com/ChainSafe/mina-rs, or your fork (using `git remote add origin`)
- squash your commits into an individual commit
- before submitting a pull request, begin git rebase on top of main

