# syntax=docker/dockerfile:1.4
FROM ghcr.io/huginn/huginn-single-process:5a1509b51188e0d16868be893c983d6fcfd232a5

USER 0
RUN apt update -qq && \
    apt install -y libjemalloc2
USER default
ENV LD_PRELOAD=/usr/lib/x86_64-linux-gnu/libjemalloc.so.2

USER 0
RUN apt update -qq && \
    apt install libjemalloc2 && \
	rm -rf /var/cache
ENV LD_PRELOAD=/usr/lib/x86_64-linux-gnu/libjemalloc.so.2
USER default

# ENV ADDITIONAL_GEMS="stackprof,sentry-ruby,sentry-rails"
RUN APP_SECRET_TOKEN=secret DATABASE_ADAPTER=postgresql ON_HEROKU=true bundle install -j 4
# COPY --link sentry.rb /app/config/initializers/sentry.rb

RUN sed -e "s,1m' do,30m' do," -e "/[125]m'/d" lib/huginn_scheduler.rb -i.bak
# sed -e "s,1m' do,1h' do," -e "/m'/d" lib/huginn_scheduler.rb | diff -u lib/huginn_scheduler.rb -
