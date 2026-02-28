#!/bin/sh

mkdir -p ~/.local/share/mime/packages
mkdir -p ~/.local/share/applications

cp ./mime.xml ~/.local/share/mime/packages/3a-mime.xml
cp ./aaa.desktop ~/.local/share/applications/aaa.desktop

update-mime-database ~/.local/share/mime
update-desktop-database ~/.local/share/applications

echo "Checking"
gio info ./art/apple.3a | grep "standard::content-type"
xdg-mime query default application/x-3a

