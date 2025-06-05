# easydot
a simple dotfile manager

## installation and use
1. create an `easydot` folder (or call it whatever you like)
2. download the executable found in releases, and place it in the folder you created
3. within the easydot folder, create another folder called `dotfiles`
4. move your dotfiles into the new `dotfiles` folder
5. create a file called `filemap.toml` in the `easydot` folder
6. populate the filemap with your sources and destinations
7. run easydot
8. done!

## filemap.toml
this is the file that easydot reads to figure out which files should go where

### example structure
```toml
[zsh]
source = "zshrc"
dest = "~/.zshrc"

[dotconfig]
source = "dotconfig"
dest = "~/.config"
```

- the table names are arbitrary - they're your way to figure out what's going where, easydot doesn't use them
- the source is the file or folder name inside the `dotfiles` folder, e.g. for the `zsh` table, that source file actually exists at `./dotfiles/zshrc`
- the dest is where easydot will link your source file - if your source is a folder and the destination already exists, easydot will instead link everything inside your source to the destination, e.g.
`./dotfiles/dotconfig/nvim` will link to `~/.config/nvim`, if `~/.config` already exists

# alternatives
easydot was made mainly as a small project with which to learn rust. As such, it's probably not the most robust and certainly not the most feature complete dotfile manager.

- [chezmoi](https://chezmoi.io/)
- [dotbot](https://github.com/anishathalye/dotbot)
- [GNU stow](https://www.gnu.org/software/stow/)
