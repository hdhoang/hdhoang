# syntax=docker/dockerfile:1.10
FROM ghcr.io/huginn/huginn-single-process:087bcdd8f21ead2dd4e587b554e8248e5ffd4b99

USER 0
RUN apt update -qq && \
    apt install -y libjemalloc2

RUN sed -i.bak -e "s,1m' do,30m' do," -e "/[125]m'/d" lib/huginn_scheduler.rb && \
    sed -i.bak -e "/twitter/d" -e "/jabber/d" /app/lib/agent_runner.rb && \
    sed -i.bak -e "/twilio|-ftp|eather|mqtt|xmpp|slack|pdf|hipchat|weibo/d" \
               -e "/Optional lib/,/google-oauth2/d" \
               -e "/bsd|BSD/d" Gemfile && \
    rm -v app/models/agents/hipchat* app/models/agents/dropbox* app/models/agents/tumblr* app/models/agents/twilio* app/models/agents/telegram* app/models/agents/slack* app/models/agents/s3* app/models/agents/twitter* app/models/agents/google* app/models/agents/pdf* app/models/agents/jabber* app/models/agents/jira* app/models/agents/imap* app/models/agents/ftp* \
    app/concerns/twitter*
RUN gem install bundler:2.5.22

USER default
ENV LD_PRELOAD=/usr/lib/x86_64-linux-gnu/libjemalloc.so.2

# ENV ADDITIONAL_GEMS="stackprof,sentry-ruby,sentry-rails,mini_racer"
ENV ADDITIONAL_GEMS=mini_racer
RUN APP_SECRET_TOKEN=secret DATABASE_ADAPTER=postgresql bundler install -j 4
# COPY --link sentry.rb /app/config/initializers/sentry.rb
