#!/usr/bin/env bash
./build.release.sh
sudo dpkg --remove carusbaad
sudo dpkg -i target/debian/carusbaad_0.1.0_amd64.deb
sudo udevadm control --reload-rules
