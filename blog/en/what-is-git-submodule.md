# What is Git Submodule?

A feature that lets you include another Git repository inside your own, **pinned to a specific commit**.

## Why use it

When you need external library source code, copying it directly into your repo means:

- You lose the connection to the original repository
- You can't track upstream updates
- You end up with duplicated code

With submodules, only a **reference (pointer)** is stored — "include this commit from this repository here."

## What actually gets stored

- `.gitmodules` — maps submodule URLs to paths
- A **commit hash** that the submodule points to, tracked internally by Git

It doesn't copy the entire source into your repository. It just records which commit to reference.

## Usage

### Adding a submodule

```bash
git submodule add https://github.com/example/lib.git path/to/lib
```

### Cloning

```bash
# clone with submodules in one step
git clone --recurse-submodules <repo-url>

# fetch submodules after a regular clone
git submodule update --init --recursive
```

### Updating

To bump a submodule to a newer commit:

```bash
cd path/to/lib
git pull origin main
cd ..
git add path/to/lib
git commit -m "update submodule"
```

## Things to watch out for

- A plain `git clone` leaves the submodule directory **empty**. You must either use `--recurse-submodules` or run `git submodule update --init --recursive` after cloning.
- Submodules are pinned to a specific commit. They don't automatically follow upstream updates — you have to update them explicitly.
