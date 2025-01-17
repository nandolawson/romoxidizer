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
    #!/usr/bin/bash
    if [ -z "{{ PROFILE }}" ] || [ "{{ PROFILE }}" = "debug" ]
    then
        echo "Compiling debug build..."
        cargo build
    elif [ "{{ PROFILE }}" = "release" ]
    then
        targets=("x86_64-unknown-linux-gnu" "aarch64-unknown-linux-gnu" "x86_64-pc-windows-gnu")
        for target in "${targets[@]}"
        do
            echo "Compiling build for target: $(case "$target" in
                "x86_64-unknown-linux-gnu") echo "Linux (x86)";;
                "aarch64-unknown-linux-gnu") echo "Linux (arm64)";;
                "x86_64-pc-windows-gnu") echo "Windows (x86)";;
                *);;
            esac)..."
            cross build --target $target --release || {
                echo "Build failed!" >&2
                exit 1
            }
        done
    fi
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
    cargo upgrade >/dev/null 2>&1 && \
    echo "Done!"

[doc('Show version of the project')]
version:
    echo "{{ VERSION }}"
