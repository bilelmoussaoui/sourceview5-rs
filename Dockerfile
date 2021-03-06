FROM fedora:latest

RUN dnf update -y
RUN dnf install gtk-update-icon-cache git 'dnf-command(builddep)' -y
RUN dnf builddep gtk4 -y

# build gtk4 from the latest release
ADD https://download.gnome.org/sources/gtk/4.1/gtk-4.1.1.tar.xz /tmp/gtk-4.1.1.tar.xz
RUN tar -xf /tmp/gtk-4.1.1.tar.xz --directory /tmp
WORKDIR /tmp/gtk-4.1.1
RUN meson _build --prefix=/usr
RUN ninja -C _build
RUN ninja -C _build install
WORKDIR /

# Build gtksourceview5 from the latest release
ADD https://download.gnome.org/sources/gtksourceview/4.99/gtksourceview-4.99.0.tar.xz /tmp/gtksourceview-4.99.0.tar.xz
RUN tar -xf /tmp/gtksourceview-4.99.0.tar.xz --directory /tmp
WORKDIR /tmp/gtksourceview-4.99.0
RUN meson _build --prefix=/usr -Dvapi=false
RUN ninja -C _build
RUN ninja -C _build install
