FROM rust:1.100.0-trixie

ARG DEV_USERNAME=dev
ARG DEV_GROUPNAME=dev
ARG DEV_UID=1000
ARG DEV_GID=1000

# ユーザーとグループを作成する
RUN groupadd -g ${DEV_GID} ${DEV_GROUPNAME}
RUN useradd -m -s /bin/bash -u ${DEV_UID} -g ${DEV_GID} ${DEV_USERNAME}
