FROM fedora:latest

RUN dnf update -y

RUN dnf install -y glib-devel \
    git \
    meson \
    gcc \
    gobject-introspection-devel \
    wget \
    graphene-devel \
    gtk4-devel \
    xz \
    gcc-c++ \
    gtk-doc \
    vulkan-loader-devel \
    cups-devel \
    gstreamer1-devel \
    gstreamer1-plugins-base-devel \
    gstreamer1-plugins-bad-free-devel \
    diffutils \
    librsvg2-devel \
    xorg-x11-server-Xvfb \
    procps-ng

# build gtk4 from the latest release
ADD https://download.gnome.org/sources/gtk/4.0/gtk-4.0.3.tar.xz /tmp/gtk-4.0.3.tar.xz
RUN tar -xf /tmp/gtk-4.0.3.tar.xz --directory /tmp
WORKDIR /tmp/gtk-4.0.3
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