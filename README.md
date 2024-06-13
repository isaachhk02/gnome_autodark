# GNOME Auto-Dark Mode

Change the light/dark mode in specify hour

# Build
`make`

# Install
`make install`

# Disable the service
`systemctl --user disable gnome-autodark.timer`

`systemctl --user disable gnome-autodark.service`

# Enable the service

`systemctl --user enable gnome-autodark.service`

`systemctl --user enable gnome-autodark.timer`

# How to remove?

First disable the systemd services

`systemctl --user disable --now gnome-autodark.service`

`systemctl --user disable --now gnome-autodark.timer`

And finally remove the systemd file and binary file

`sudo rm -v /etc/systemd/user/gnome-autodark.service  /etc/systemd/user/gnome-autodark.timer /usr/bin/gnome-autodark`
