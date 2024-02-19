# .bash_aliases

# general aliases
alias ls='ls --color=auto'
alias lt='ls --human-readable --size -1 -S --classify'
alias count='find . -type f | wc -l'
alias hgrep='history | grep --color=auto'
alias sudo='sudo '
alias g='grep --color=auto'
alias die='exit'

# specific aliases
alias jupyter='cd ~; python -m notebook'
alias neofetch='/home/bhavya/programs/scripts/neofetch-cache'

# git aliases
alias gits='git status'
alias gitd='git diff'
alias gita='git add'
alias gitl='git log'

# xbps aliases
alias xi='xbps-install'
alias xq='xbps-query'
