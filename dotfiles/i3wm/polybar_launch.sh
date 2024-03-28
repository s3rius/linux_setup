#!/usr/bin/env bash

# Terminate already running bar instances
# If all your bars have ipc enabled, you can use 
killall -q polybar || echo "Already dead"
# Otherwise you can use the nuclear option:
# polybar-msg cmd quit

# Launch bar1 and bar2
echo "---" | tee -a /tmp/polybar.log
QUERY="$(xrandr --query)"

PRIMARY_MON="$(echo "$QUERY" | grep " connected" | grep "primary" | cut -d ' ' -f 1)"

MONITOR=$PRIMARY_MON TRAY_POS=right polybar -q &


if type "xrandr"; then
  for m in $(echo "$QUERY" | grep " connected" | grep -v "primary" | cut -d ' ' -f 1); do
    echo "Starting bar for monitor $m"
    MONITOR=$m polybar -q & 
  done
else
  polybar -q --reload & 
fi

echo "Bars launched..."
