# github-notif-indicator

A bar module (specifically for [waybar], but should be generic) for
showing the number of notifications on GitHub and Codeberg.

This is basically the same as the [native github module] on polybar,
just a lot less configurable (and it supports Codeberg)

[waybar]: https://github.com/Alexays/Waybar
[native github module]: https://github.com/polybar/polybar/wiki/Module:-github

## Install

To install, you need to have rust and cargo installed

```sh
cargo install --git https://github.com/funnyboy-roks/github-notif-indicator.git
```

## Waybar

To use in way bar, install and then add following custom module:

```jsonc
"custom/github": {
    "exec": "github-notif-indicator <path-to-github-secret> <path-to-codeberg-secret> 10"
}
```

## Usage Note

This application is very tailored to my needs.  If you intend to use it
or want support for more software, please let me know and I may consider
making this a little more polished.
