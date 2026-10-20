# Contributing to NaOnDB

We welcome developers, researchers, students, and people building useful personal
projects. Code, tests, ontology examples, documentation, reproducible benchmarks,
bug reports, and thoughtful design feedback are all valuable contributions.

**You do not need to buy a commercial license to collaborate on NaOnDB.**
Permitted personal and noncommercial use is free. The project is source-available,
not OSI open source: commercial exploitation requires a separate paid license.
Read [LICENSE](LICENSE) and [COMMERCIAL-LICENSING.md](COMMERCIAL-LICENSING.md).

## Find a focused task

Read [AGENTS.md](AGENTS.md), [the architecture](docs/architecture.md), and the
[implementation backlog](planning/ISSUES.md). Discuss the relevant live issue
before starting a large change. Preserve the universal ontology-native core,
redb's single transaction boundary, and the distinction between claims and facts.
A small, demonstrated vertical improvement is preferable to speculative frameworks.

Fork the repository, make a focused branch, and open a pull request explaining
what changed, which issue it addresses, what was verified, and what remains.
Private forks and research experiments do not have to be submitted upstream.

## Checks

```bash
cargo fmt --all -- --check
cargo clippy --all-targets -- -D warnings
cargo test --all-targets
cargo run --example embedded
```

Add focused regression checks for changed behavior. Do not use paid services,
private user data, or external LLM calls in tests. Do not claim planned database
features are implemented merely because a type or skeleton exists.

## Rights and commercial licensing

Before an external contribution is merged, each relevant rights holder must
explicitly accept [Contributor Agreement 1.0](CONTRIBUTOR-AGREEMENT.md) for the
identified PR and commit, or supply a separately accepted equivalent agreement.
Post the agreement's acceptance statement yourself; include an immutable link
to the agreement version and obtain employer/university/client authorization
where needed. Maintainers must recheck coverage after additional commits.
A bot-generated checkbox or a DCO sign-off alone is not the required permission.

You keep your copyright. You expressly allow Christian Klose to include your
accepted work in the community version and separate paid commercial licenses.
Participation promises neither payment nor a share of licensing revenue; any
compensation must be agreed separately, subject to mandatory statutory rights.
The current commercial license price is EUR 20,000 for scope agreed in writing.

No agreement is required merely to ask a question or report a bug. Material
copied from such a report into the distributed project still needs appropriate
rights clearance. Do not paste third-party code without its source and license.

## Maintainer acceptance checklist

Review technical evidence, attribution, third-party compatibility, and the
explicit contributor permission before merging. Link the recorded acceptance in
the PR and verify that all included authors and the final commit are covered.
If rights are unclear, leave the contribution unmerged until resolved.
Do not retroactively treat existing work as covered by a newly added agreement.
This process is manual; branch protection or automated CLA enforcement has not
been configured by these documents.
