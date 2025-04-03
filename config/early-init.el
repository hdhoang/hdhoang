;; Startup speed, annoyance suppression
(setq gc-cons-threshold most-positive-fixnum)
(add-hook 'emacs-startup-hook
          (lambda () (setq gc-cons-threshold (* 50 1024 1024))))

(setq byte-compile-warnings '(not obsolete))
(setq warning-suppress-log-types '((comp) (bytecomp)))
(setq native-comp-async-report-warnings-errors 'silent)
(setq native-comp-speed 2)

(setq inhibit-startup-screen t)
(setq inhibit-startup-echo-area-message (user-login-name))
(tool-bar-mode -1)
