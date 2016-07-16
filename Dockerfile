FROM ubuntu:latest

WORKDIR /root

ENV DEBIAN_FRONTEND=noninteractive

RUN apt-get update && \
	apt-get upgrade -y && \
	apt-get install --no-install-recommends -y \
		build-essential \
		ca-certificates \
		curl \
		file \
		xutils-dev

RUN curl https://sh.rustup.rs -sSf | \
	sh -s -- --default-toolchain stable -y

ENV PATH=/root/.cargo/bin:$PATH

RUN \
	apt-get -y install \
		krb5-admin-server \
		krb5-kdc \
		krb5-multidev \
		krb5-user \
		libkrb5-dev \
		pkg-config \
		python-pip

RUN pip install k5test

RUN mkdir -p /kerberos
WORKDIR /rust-gssapi

CMD /rust-gssapi/tests/cmd.sh

COPY tests/krb5.conf /etc/krb5.conf
COPY tests/kdc.conf /etc/krb5kdc/kdc.conf


COPY . /rust-gssapi