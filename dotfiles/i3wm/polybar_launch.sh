#!/usr/bin/env bash

# Terminate already running bar instances
# If all your bars have ipc enabled, you can use 
polybar-msg cmd quit
# Otherwise you can use the nuclear option:
# killall -q polybar

# Launch bar1 and bar2
echo "---" | tee -a /tmp/polybar.log

PRIMARY_MON="$(xrandr --query | grep " connected" | grep "primary" | cut -d ' ' -f 1)"

echo "Launching bar for primary monitor: $PRIMARY_MON"

MONITOR=$PRIMARY_MON TRAY_POS=right polybar & disown


if type "xrandr"; then
  for m in $(xrandr --query | grep " connected" | grep -v "primary" | cut -d ' ' -f 1); do
    echo "Starting bar for monitor $m"
    MONITOR=$m polybar & disown
  done
else
  polybar --reload &
fi

echo "Bars launched..."
