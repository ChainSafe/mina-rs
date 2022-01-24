Contributing
Thank you for your interest in making contributions to Mina rs! Start by taking a look at this coding repo for overall information on repository workflow and standards.

Please follow standard github best practices: fork the repo, branch from the tip of develop, make some commits, and submit a pull request to develop. See the open issues for things we need help with!


Looking for a good place to start contributing? How about checking out some good first issues

Forking

For instance, to create a fork and work on a branch of it, One would:

-
-
-
-
-


To pull in updates from the origin repo, run

* `git fetch upstream`
* `git rebase upstream/develop` (or whatever branch you want)
Do not make Pull Requests to master, they will not be considered.


Dependencies


-
-
-
-


Since some dependencies are not under our control, a third party may break our build, in which case we can fall back on dep ensure (or make deps). Even for dependencies under our control, dep helps us to keep multiple repos in sync as they evolve. Anything with an executable, such as apps, tools, and the core, should use dep.

Run dep status to get a list of vendor dependencies that may not be up-to-date.

Testing
All repos should be hooked up to CircleCI





Libraries need not follow the model strictly, but would be wise to.


Development Procedure:

- the latest state of development is on develop
- develop must never fail make test
- develop should not fail make lint
- no --force onto develop (except when reverting a broken commit, which should seldom happen)
- create a development branch either on github.com/cosmos/ethermint, or your fork (using git remote add origin)
- squash your commits into an individual commit
- before submitting a pull request, begin git rebase on top of develop



Pull Merge Procedure:

- ensure pull branch is rebased on develop
- squash your commits into an individual commit
- run make test and make test-cli to ensure that all tests pass
- merge pull request

Release Procedure:

- start on develop
- prepare changelog/release issue
- bump versions
- push to releas to ------ run CI
- merge to master
- merge master back to develop
