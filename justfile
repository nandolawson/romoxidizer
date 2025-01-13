#>> Settings

set allow-duplicate-recipes := false
set allow-duplicate-variables := false
set dotenv-filename := ""
set dotenv-load := false
set dotenv-path := ""
set dotenv-required := false
set export := false
set fallback := false
set ignore-comments := false
set positional-arguments := false
set quiet := true
set unstable := false
set working-directory := ""

#>> Variables

NAME := `cargo pkgid | sed -rn 's|^.*://.*/([^/]+)#.*$|\1|p'`
VERSION := `cargo pkgid | sed -rn s'/^.*#(.*)$/\1/p'`


#>> Aliases

alias b := build
alias c := clean
alias f := format
alias n := name
alias u := upgrade
alias v := version

#>> Recipes

[doc('List all recipes')]
[private]
default:
    just --list

[doc('Build the project [PROFILE can be "debug" or "release"]')]
build PROFILE="debug":
    #!/usr/bin/env sh
    echo "Create {{ PROFILE }} build..."
    if [ -z "{{ PROFILE }}" ] || [ "{{ PROFILE }}" = "debug" ]
    then
        cargo build --quiet; \
    elif [ "{{ PROFILE }}" = "release" ]
    then
        echo "Linux (x86) build"
        cargo build --target x86_64-unknown-linux-gnu --release --quiet
        echo "Windows (x86) build"
        cargo build --target x86_64-pc-windows-gnu --release --quiet
    fi && \
    echo "Done!"

[doc('Remove target directory')]
clean:
    echo "Remove target directory..."
    cargo clean --quiet && \
    echo "Done!"

[doc('Generate documentation')]
doc:
    echo "Generate documentation..."
    cargo doc --no-deps --quiet && \
    echo "Done!"

[doc('Format code of all files')]
format:
    echo "Format the project..."
    cargo fmt && \
    just --fmt --unstable --quiet && \
    echo "Done!"

[doc('Show name of the project')]
name:
    echo "{{ NAME }}"

[doc('Upgrade dependencies')]
upgrade:
    echo "Upgrade all dependencies..."
    cargo upgrade > /dev/null && \ # Not quiet yet
    echo "Done!"

[doc('Show version of the project')]
version:
    echo "{{ VERSION }}"
