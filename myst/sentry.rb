Sentry.init do |config|
  config.breadcrumbs_logger = [:active_support_logger, :http_logger]

  config.traces_sample_rate = 0.1
  config.profiles_sample_rate = 1.0
end
