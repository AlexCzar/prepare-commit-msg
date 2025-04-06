# prepare-commit-msg
A simple git hook that will parse the branch name to prefix the commit message with the ticket number.

## How?

Build the binary:
```sh
git clone https://github.com/AlexCzar/prepare-commit-msg
cd prepare-commit-msg
cargo build --release
```

Put the resulting binary `target/release/prepare-commit-msg` into your git hooks folder (the name of the binary matters!).

> [!NOTE]
> You can set this up globablly, by configuring a shared hooks directory (`git config --global core.hooksPath /path/to/hooks`), and placing the file there.

The hook picks up prefixes formatted as TKT-0123 - which is the Jira style ticket ID, also widely used by other tools.

To have the hook pick it up, prefix your feature branch with TKT-0123, followed by either `_` or `-`.
The case in the branch name does not matter, the hook will always uppercase it.

## Credits
This is a RIIR of [the hook](https://github.com/robatwilliams/git-ticket-number-prefix-hook) by @robatwilliams.

