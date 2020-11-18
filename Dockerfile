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
ADD https://download.gnome.org/sources/gtk/3.99/gtk-3.99.4.tar.xz /tmp/gtk3.99.4.tar.xz
RUN tar -xf /tmp/gtk3.99.4.tar.xz --directory /tmp
WORKDIR /tmp/gtk-3.99.4
RUN meson _build --prefix=/usr
RUN ninja -C _build
RUN ninja -C _build install
WORKDIR /


# Build gtksourceview from master
RUN git clone https://gitlab.gnome.org/GNOME/gtksourceview/ /tmp/gtksourceview
WORKDIR /tmp/gtksourceview
RUN git checkout 2510db7264f853d78ec70aa2f0f66dfa5ae0adbe
RUN meson _build --prefix=/usr -Dvapi=false
RUN ninja -C _build
RUN ninja -C _build install