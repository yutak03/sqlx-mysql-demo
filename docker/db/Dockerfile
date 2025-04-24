ARG MYSQL_VERSION=8.0
FROM mysql:${MYSQL_VERSION}-debian

# Install packages
RUN apt-get update && \
    apt-get install -y wget gnupg procps && \
    apt-get clean && \
    rm -rf /var/lib/apt/lists/*

# Set locale
RUN apt-get update && apt-get install -y locales locales-all
RUN sed -i -E 's/# (ja\_JP.UTF-8)/\\1/' /etc/locale.gen && locale-gen
RUN locale-gen ja_JP.UTF-8

ENV LANG ja\_JP.UTF-8