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
                librsvg2-devel

# build gtk4 from the latest release
ADD https://download.gnome.org/sources/gtk/3.99/gtk-3.99.3.tar.xz /tmp/gtk3.99.3.tar.xz
RUN tar -xf /tmp/gtk3.99.3.tar.xz --directory /tmp
WORKDIR /tmp/gtk-3.99.3
RUN meson _build --prefix=/usr
RUN ninja -C _build
RUN ninja -C _build install
WORKDIR /

# Build gtksourceview from master
RUN git clone https://gitlab.gnome.org/GNOME/gtksourceview/ /tmp/gtksourceview
WORKDIR /tmp/gtksourceview
RUN git checkout bf870ac02f3dbbc499a740ab9c99501d173b13b5
RUN meson _build --prefix=/usr -Dvapi=false
RUN ninja -C _build
RUN ninja -C _build install