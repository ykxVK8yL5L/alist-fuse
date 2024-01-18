FROM alpine:latest
ARG TARGETARCH
ARG TARGETVARIANT
RUN apk --no-cache add ca-certificates tini fuse3
RUN apk add tzdata && \
	cp /usr/share/zoneinfo/Asia/Shanghai /etc/localtime && \
	echo "Asia/Shanghai" > /etc/timezone && \
	apk del tzdata

RUN mkdir -p /etc/alist-fuse /mnt/alistDrive
WORKDIR /root/
ADD alist-fuse-$TARGETARCH$TARGETVARIANT /usr/bin/alist-fuse

ENTRYPOINT ["/sbin/tini", "--"]
CMD ["/usr/bin/alist-fuse", "--workdir", "/etc/alist-fuse", "/mnt/alistDrive"]
