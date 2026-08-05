# github-notif-indicator

A bar module (specifically for [waybar], but should be generic) for
showing the number of notifications on GitHub.

This is basically the same as the [native github module] on polybar,
just a lot less configurable

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
    "exec": "github-notif-indicator <path-to-secret> 10"
}
```
