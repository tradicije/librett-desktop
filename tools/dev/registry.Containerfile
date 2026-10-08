# Copyright (C) 2026 Aleksa Dimitrijević. AGPL-3.0-or-later.
FROM registry.fedoraproject.org/fedora:44
RUN dnf -y install rust cargo rustfmt clippy gcc gcc-c++ pkgconf-pkg-config dbus-devel gtk3-devel webkit2gtk4.1-devel libappindicator-gtk3-devel librsvg2-devel openssl-devel cmake perl-core nodejs npm && dnf clean all
WORKDIR /workspace
