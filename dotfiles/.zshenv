typeset -U path
PYENV_ROOT=~/.pyenv/
EDITOR=/bin/vim
LC_ALL="$LANG"
path=($path[@] ~/.poetry/bin/ ~/.local/bin/ ~/.pyenv/bin ~/.cargo/bin/)
fpath=($fpath[@] ~/.zfunc)