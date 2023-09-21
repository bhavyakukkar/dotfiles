# aliases
if [ -e $HOME/.aliases ]; then
	source $HOME/.aliases
fi

setopt PROMPT_SUBST
PROMPT='
╭─(%F{green}%n%f) in [%F{blue}%~%f] at %F{yellow}%*%f
╰─$ '

# pywal color palette
(cat ~/.cache/wal/sequences &)
