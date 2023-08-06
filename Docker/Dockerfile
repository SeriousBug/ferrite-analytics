FROM scratch

EXPOSE 3000/tcp
VOLUME []

ARG TARGETPLATFORM

ADD binaries/$TARGETPLATFORM/ferrite-analytics /

WORKDIR /

ENTRYPOINT ["/ferrite-analytics"]