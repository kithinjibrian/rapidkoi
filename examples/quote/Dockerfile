# The quote demo's server in a container (behind a TLS proxy such as
# Traefik). Build context: a folder holding the `quote-server` binary from
#   lughac build examples/quote/server.la -o quote-server
# It's linked on Ubuntu 26.04 against libgc and OpenSSL 3, so the base matches.
# The signed site is mounted read-only at /srv/site; set QUOTE_HOST to the
# app's host.
FROM ubuntu:26.04
RUN apt-get update \
    && apt-get install -y --no-install-recommends libgc1 libssl3t64 ca-certificates \
    && rm -rf /var/lib/apt/lists/* \
    && useradd --system --no-create-home quote
COPY quote-server /usr/local/bin/quote-server
USER quote
EXPOSE 8080
ENV QUOTE_HOST=localhost:8080
CMD ["sh", "-c", "exec quote-server --site /srv/site --host \"$QUOTE_HOST\" --listen 0.0.0.0 --port 8080"]
