function gg {
  eval "in dir jump $1"
  local target=/tmp/inspection-bashmarks-jump-target
  if test -f $target; then
    cd "$(cat $target)"
    ls -pG
  fi
}