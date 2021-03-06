FROM ghcr.io/gtk-rs/gtk4-rs/gtk4:latest

RUN dnf update -y
RUN dnf install gtk-update-icon-cache -y

# Build gtksourceview5 from the latest release
ADD https://download.gnome.org/sources/gtksourceview/5.0/gtksourceview-5.0.0.tar.xz /tmp/gtksourceview-5.0.0.tar.xz
RUN tar -xf /tmp/gtksourceview-5.0.0.tar.xz --directory /tmp
WORKDIR /tmp/gtksourceview-5.0.0
RUN meson _build --prefix=/usr -Dvapi=false
RUN ninja -C _build
RUN ninja -C _build install
