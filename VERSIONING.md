# Versioning and Branching in Trustification

This document outlines how we handle versioning in the Trustification
organization.

Individual projects inside the Trustification organization may have more specific
rules and/or guidelines.

# Overview

Trustification follows [Semantic Versioning](https://semver.org/). This means that
a version is expressed in the format X.Y.Z, where:

- X is the major version, representing significant changes and new features
- Y is the minor version, representing new features and improvements
- Z is the patch version, representing bug fixes and minor improvements

_NOTE_: If the major release is 0, any minor release may contain breaking changes.

# Branching

In order to maintain our guarantees two types of branches will be maintained:
`main` and `release-X.Y`.

- `main` branch is where all of the development happens. It is not guaranteed
  stable and may contain breaking changes.
- `release-X.Y` where X.Y is the minor version for a release is considered
  stable. These branches will receive backport fixes for critical issues, but no
  new features will be added.

# Test multicommit

# Releases

All releases from the main branch will be marked as pre-release and be of the
form `vX.Y.0-alpha.n`, where `X.Y` represents the next minor release and `n`
follows Semantic Versioning precedence rules for pre-release. For example,
if the next minor release is going to be 2.0, a pre-release from the main
branch might be `v2.0.0-alpha.1` or `v2.0.0-alpha.2`, depending on the number of
pre-releases that have been made so far. By marking releases from the main
branch as pre-release, we aim to clearly indicate that the code may contain
breaking changes and is not yet considered stable.

Releases from `release-X.Y` branches contain stable code and always start with
`vX.Y.0`. For example, the first release on a `release-2.3` branch would be
`v2.3.0` and after critical issues are fixed + backported, the next release on
the same branch would be `v2.3.1`. Marking these as full releases, we aim to
indicate that they are stable, tested, and usable in production settings.

_NOTE_ When we say release, we are referring to a
[GitHub release](https://docs.github.com/en/repositories/releasing-projects-on-github/managing-releases-in-a-repository)
and the associated git tag.

# Pull Request (PR) Process

Every PR should be annotated with an icon indicating whether it's
a:

- Breaking change: :warning: (`:warning:`)
- Non-breaking feature: :sparkles: (`:sparkles:`)
- Patch fix: :bug: (`:bug:`)
- Docs: :book: (`:book:`)
- Infra/Tests/Other: :seedling: (`:seedling:`)
- No release note: :ghost: (`:ghost:`)

Since GitHub supports emoji aliases (ie. `:ghost:`), there is no need to include
the emoji directly in the PR title -- **please use the alias**. It used to be
the case that projects using emojis for PR typing had to include the emoji
directly because GitHub didn't render the alias. Given that `:warning:` is
easy enough to read as text, easy to parse in release tooling, and rendered in
GitHub well, we prefer to standardize on the alias.
