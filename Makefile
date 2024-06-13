gnome-autodark:
	cargo build
install:
	sudo cp target/debug/gnome-autodark /usr/bin/gnome-autodark
	sudo cp gnome-autodark.service /etc/systemd/user
	sudo cp gnome-autodark.timer /etc/systemd/user
	systemctl --user daemon-reload
	systemctl --user enable --now gnome-autodark.timer
	systemctl --user enable --now gnome-autodark.service
	echo "Done!"
	
