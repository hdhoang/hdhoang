                                        ; bedrock early-init.el
;; Startup speed, annoyance suppression
(setq gc-cons-threshold 10000000)
(setq byte-compile-warnings '(not obsolete))
(setq warning-suppress-log-types '((comp) (bytecomp)))
(setq native-comp-async-report-warnings-errors 'silent)

;; Silence stupid startup message
(setq inhibit-startup-echo-area-message (user-login-name))
;;;

(custom-set-faces
 ;; custom-set-faces was added by Custom.
 ;; If you edit it by hand, you could mess it up, so be careful.
 ;; Your init file should contain only one such instance.
 ;; If there is more than one, they won't work right.
 )
(custom-set-variables
 ;; custom-set-variables was added by Custom.
 ;; If you edit it by hand, you could mess it up, so be careful.
 ;; Your init file should contain only one such instance.
 ;; If there is more than one, they won't work right.
 '(auth-source-save-behavior nil)
 '(auto-revert-avoid-polling t)
 '(auto-revert-check-vc-info t)
 '(auto-revert-interval 5)
 '(auto-revert-mode-text "")
 '(blink-cursor-mode nil)
 '(column-number-mode t)
 '(completion-detailed t)
 '(completion-group t)
 '(desktop-load-locked-desktop t)
 '(desktop-save-mode t)
 '(dired-hide-details-hide-symlink-targets nil)
 '(dired-vc-rename-file t)
 '(display-line-numbers-width 3)
 '(display-time-default-load-average nil)
 '(eldoc-minor-mode-string "")
 '(font-use-system-font t)
 '(indent-tabs-mode nil)
 '(indicate-buffer-boundaries 'left)
 '(initial-major-mode #'sql-mode)
 '(initial-scratch-message nil)
 '(isearch-lazy-count t)
 '(line-number-mode t)
 '(major-mode-remap-alist
   '((conf-toml-mode . toml-ts-mode)
     (go-mode . go-ts-mode)
     (js-json-mode . json-ts-mode)
     (python-mode . python-ts-mode)
     (rust-mode . rust-ts-mode)
     (sh-mode . bash-ts-mode)
     (yaml-mode . yaml-ts-mode)))
 '(menu-bar-mode t)
 '(package-selected-packages
   '(kdl-ts-mode terraform-mode hcl-mode pcre2el apheleia justl just-mode marginalia avy rustic which-key orderless fira-code-mode combobulate treesit expand-region groovy-mode jinja2-mode magit-delta markdown-mode rainbow-delimiters use-package))
 '(python-indent-offset 4)
 '(repeat-mode t)
 '(require-final-newline 't)
 '(rust-format-on-save t)
 '(safe-local-variable-values
   '((electric-pair-mode . t)
     (vc-prepare-patches-separately)
     (diff-add-log-use-relative-names . t)
     (vc-git-annotate-switches . "-w")))
 '(tool-bar-mode nil)
 '(treesit-font-lock-level 4)
 '(treesit-language-source-alist
   '((bash "https://github.com/tree-sitter/tree-sitter-bash")
     (c "https://github.com/tree-sitter/tree-sitter-c")
     (cpp "https://github.com/tree-sitter/tree-sitter-cpp")
     (dockerfile "https://github.com/camdencheek/tree-sitter-dockerfile")
     (go "https://github.com/tree-sitter/tree-sitter-go")
     (html "https://github.com/tree-sitter/tree-sitter-html")
     (javascript "https://github.com/tree-sitter/tree-sitter-javascript" "master" "src")
     (json "https://github.com/tree-sitter/tree-sitter-json")
     (kdl "https://github.com/amaanq/tree-sitter-kdl")
     (python "https://github.com/tree-sitter/tree-sitter-python")
     (rust "https://github.com/tree-sitter/tree-sitter-rust")
     (toml "https://github.com/tree-sitter/tree-sitter-toml")
     (yaml "https://github.com/ikatyang/tree-sitter-yaml")) t)
 '(warning-suppress-types '((use-package)))
 '(whitespace-style
   '(face trailing tabs missing-newline-at-eof indentation::space))
 '(x-underline-at-descent-line nil))

(global-auto-revert-mode)

(require 'package)
(add-to-list 'package-archives '("melpa" . "https://melpa.org/packages/") t)

(use-package fira-code-mode
  :ensure
  :hook fundamental-mode)
(set-face-attribute 'default nil :height 100)

(use-package which-key
  :ensure
  :custom
  (which-key-lighter nil)
  :config
  (which-key-mode))
(windmove-default-keybindings 'control)
(when (display-graphic-p)
  (context-menu-mode))

(add-to-list 'load-path (expand-file-name "~/gh/combobulate/"))
(add-to-list 'load-path (expand-file-name "~/gh/kdl-ts-mode/")) ; https://github.com/dataphract/kdl-ts-mode

(use-package kdl-ts-mode)

;; `M-x combobulate' (default: `C-c o o') to start using Combobulate
(use-package combobulate
  :preface (setq combobulate-key-prefix "C-c o")

  :hook ((python-ts-mode . combobulate-mode)
         (js-ts-mode . combobulate-mode)
         (css-ts-mode . combobulate-mode)
         (yaml-ts-mode . combobulate-mode)
         (json-ts-mode . combobulate-mode)))
                                        ; refresh grammars
                                        ; (dolist (grammar treesit-language-source-alist) (treesit-install-language-grammar (car grammar)))

(defun my-increment-number-decimal-at-point (&optional arg)
  "Increment the number forward from point by 'arg'."
  (interactive "p*")
  (save-excursion
    (save-match-data
      (let (inc-by field-width answer)
        (setq inc-by (if arg arg 1))
        (skip-chars-backward "0123456789")
        (when (re-search-forward "[0-9]+" nil t)
          (setq field-width (- (match-end 0) (match-beginning 0)))
          (setq answer (+ (string-to-number (match-string 0) 10) inc-by))
          (when (< answer 0)
            (setq answer (+ (expt 10 field-width) answer)))
          (replace-match (format (concat "%0" (int-to-string field-width) "d")
                                 answer)))))))
(global-set-key (kbd "C-x C-a") #'my-increment-number-decimal-at-point)
(global-set-key (kbd "C-x C-d") #'duplicate-dwim)
(global-set-key (kbd "C-x C-l") #'copy-from-above-command)

(if (string-equal (getenv "USER") "hdhoang")
    (load-theme 'tango-dark t)
  (load-theme 'tango t))

(use-package magit-delta
  :ensure
  :custom (magit-define-global-key-bindings 'recommended)
  :bind ("C-c M-g l" . #'magit-log-buffer-file)
  :hook ((magit-mode . magit-delta-mode)))
(setq transient-levels '((magit-pull (transient:magit-pull:--autostash . 1))))

(use-package rainbow-delimiters
  :ensure
  :hook ((prog-mode . rainbow-delimiters-mode)))
(use-package expand-region
  :ensure
  :bind ("C-=" . #'er/expand-region))

(add-hook 'fundamental-mode-hook #'whitespace-mode)
(add-hook 'fundamental-mode-hook #'follow-mode)
(add-hook 'occur-mode-hook #'next-error-follow-minor-mode)
(add-hook 'prog-mode-hook #'display-line-numbers-mode)

(use-package markdown-mode
  :ensure
  :hook ((markdown-mode . visual-line-mode)))

(global-set-key (kbd "C-x f") #'apheleia-format-buffer) ; enable apheleia on-demand
(use-package apheleia
  :ensure
  :custom
  (apheleia-mode-lighter nil)
  :hook
  ((rust-ts-mode . apheleia-mode)
                                        ; (yaml-ts-mode . apheleia-mode) ; it's rude to trample manifests
   (elisp-mode . apheleia-mode)
   (python-ts-mode . apheleia-mode))
  :config
  (add-to-list 'apheleia-mode-alist '(toml-ts-mode . dprint))
  (add-to-list 'apheleia-mode-alist '(rust-ts-mode . dprint))
  (add-to-list 'apheleia-mode-alist '(dockerfile-ts-mode . dprint))

  (add-to-list 'apheleia-mode-alist '(yaml-ts-mode . prettier-yaml))

  (add-to-list 'apheleia-mode-alist '(python-mode . ruff))
  (add-to-list 'apheleia-mode-alist '(python-ts-mode . ruff)))

(use-package groovy-mode
  :ensure)

(use-package pcre2el
  :ensure)
(use-package jinja2-mode
  :ensure)
(use-package terraform-mode
  :ensure)
(use-package hcl-mode
  :ensure)
;; (use-package ansible-doc)
;; (use-package company-ansible)

(use-package orderless
  :ensure
  :custom (completion-styles '(basic partial-completion emacs22)))
(use-package avy
  :ensure
  :bind (("C-c j" . avy-goto-line)
         ("s-j"   . avy-goto-char-timer)))
(global-set-key [remap dabbrev-expand] #'hippie-expand)

;; Marginalia: annotations for minibuffer
(use-package marginalia
  :ensure
  :config
  (marginalia-mode))

(use-package eglot
  :ensure
  :config
  (fset #'jsonrpc--log-event #'ignore)
  (add-to-list 'eglot-server-programs
               '((rust-ts-mode) .
                 ("rust-analyzer" :initializationOptions (:check (:command "clippy")))))
  :hook
  ((rust-ts-mode . eglot-ensure))
  :custom
  (eglot-send-changes-idle-time 0.1)
  (eglot-extend-to-xref t)
  )

(use-package rustic
  :ensure
  :custom
  (rustic-lsp-client 'eglot)
  :hook
  ((eglot-managed-mode . (lambda () (flymake-mode -1))))
  :bind (:map rustic-mode-map
              ("M-j" . lsp-ui-imenu)
              ("M-?" . lsp-find-references)
              ("C-c C-c l" . flycheck-list-errors)
              ("C-c C-c a" . lsp-execute-code-action)
              ("C-c C-c r" . lsp-rename)
              ("C-c C-c q" . lsp-workspace-restart)
              ("C-c C-c Q" . lsp-workspace-shutdown)
              ("C-c C-c s" . lsp-rust-analyzer-status)
              ))
(use-package just-mode
  :ensure)
(use-package justl
  :ensure)

(add-to-list 'magic-mode-alist '("^$TTL" . zone-mode))
(add-to-list 'magic-mode-alist '("^$ORIGIN" . zone-mode))
(add-to-list 'magic-mode-alist '("^# syntax=docker" . dockerfile-ts-mode))

(server-start)
(put 'narrow-to-region 'disabled nil)
