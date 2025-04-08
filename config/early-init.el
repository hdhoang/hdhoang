;; Startup speed, annoyance suppression
(setopt gc-cons-threshold most-positive-fixnum)
(add-hook 'emacs-startup-hook
          (lambda () (setopt gc-cons-threshold (* 50 1024 1024))))

(setopt byte-compile-warnings '(not obsolete)
        warning-suppress-log-types '((comp) (bytecomp))
        native-comp-async-report-warnings-errors 'silent
        native-comp-speed 2)

(setopt inhibit-startup-screen t
        inhibit-startup-echo-area-message (user-login-name)
        tool-bar-mode nil)
