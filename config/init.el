;;; -*- lexical-binding: t -*-

;; bedrock early-init.el
;; Startup speed, annoyance suppression
(setq bedrock--initial-gc-threshold gc-cons-threshold)
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
 '(package-selected-packages
   '(standard-themes diff-hl nov devil polymode hcl-ts-mode company-ansible terraform-doc terraform-mode treesit-ispell kdl-ts-mode pcre2el apheleia justl just-mode marginalia avy rustic which-key orderless fira-code-mode combobulate treesit expand-region groovy-mode magit-delta rainbow-delimiters use-package poly-ansible poly-markdown markdown-ts-mode poly-org))

 '(auth-source-save-behavior nil)

 '(global-auto-revert-mode t)
 '(auto-revert-avoid-polling t)
 '(auto-revert-check-vc-info t)
 '(auto-revert-interval 5)
 '(auto-revert-mode-text "")

 '(blink-cursor-mode nil)
 '(column-number-mode t)
 '(completion-detailed t)
 '(completion-group t)
 '(desktop-load-locked-desktop t)

 '(dired-hide-details-hide-symlink-targets nil)
 '(dired-vc-rename-file t)
 '(dired-listing-switches "ls -alAFth")
 '(insert-directory-program "coreutils")

 '(display-battery-mode t)
 '(display-line-numbers-width 3)
 '(display-time-default-load-average nil)
 '(eldoc-minor-mode-string "")
 '(font-use-system-font t)
 '(indent-tabs-mode nil)
 '(indicate-buffer-boundaries 'left)
 '(inhibit-startup-screen t)
 '(initial-major-mode #'sql-mode)
 '(initial-scratch-message nil)
 '(isearch-lazy-count t)
 '(line-number-mode t)

 '(menu-bar-mode t)
 '(tool-bar-mode nil)
 '(global-tab-line-mode t)

 '(require-final-newline 't)
 '(whitespace-style
   '(face trailing tabs missing-newline-at-eof indentation::space))
 '(x-underline-at-descent-line nil)

 '(python-indent-offset 4)
 '(reb-re-syntax 'string)
 '(repeat-mode t)
 '(rust-format-on-save t)
 '(safe-local-variable-values
   '((electric-pair-mode . t)
     (vc-prepare-patches-separately)
     (diff-add-log-use-relative-names . t)
     (vc-git-annotate-switches . "-w")))

 '(major-mode-remap-alist
   '((conf-toml-mode . toml-ts-mode)
     (go-mode . go-ts-mode)
     (hcl-mode . hcl-ts-mode)
     (js-json-mode . json-ts-mode)
     (python-mode . python-ts-mode)
     (markdown-mode . markdown-ts-mode)
     (rust-mode . rust-ts-mode)
     (sh-mode . bash-ts-mode)
     (yaml-mode . yaml-ts-mode)))
 '(treesit-font-lock-level 4)
 '(treesit-language-source-alist
   '((bash "https://github.com/tree-sitter/tree-sitter-bash")
     (c "https://github.com/tree-sitter/tree-sitter-c")
     (cpp "https://github.com/tree-sitter/tree-sitter-cpp")
     (dockerfile "https://github.com/camdencheek/tree-sitter-dockerfile")
     (go "https://github.com/tree-sitter/tree-sitter-go")
     (gosum "https://github.com/tree-sitter-grammars/tree-sitter-go-sum")
     (hcl "https://github.com/tree-sitter-grammars/tree-sitter-hcl")
     (html "https://github.com/tree-sitter/tree-sitter-html")
     (javascript "https://github.com/tree-sitter/tree-sitter-javascript" "master" "src")
     (json "https://github.com/tree-sitter/tree-sitter-json")
     (kdl "https://github.com/tree-sitter-grammars/tree-sitter-kdl")
     (markdown "https://github.com/tree-sitter-grammars/tree-sitter-markdown" "split_parser" "tree-sitter-markdown/src")
     (markdown-inline "https://github.com/tree-sitter-grammars/tree-sitter-markdown" "split_parser" "tree-sitter-markdown-inline/src")
     (python "https://github.com/tree-sitter/tree-sitter-python")
     (sql "https://github.com/DerekStride/tree-sitter-sql")
     (rust "https://github.com/tree-sitter/tree-sitter-rust")
     (toml "https://github.com/tree-sitter/tree-sitter-toml")
     (yaml "https://github.com/tree-sitter-grammars/tree-sitter-yaml")) t)

 '(use-package-always-defer 3)
 '(use-package-always-ensure t)
 '(warning-suppress-types '((use-package))))

(put 'narrow-to-region 'disabled nil)
(global-unset-key (kbd "C-x m"))

(require 'package)
(add-to-list 'package-archives '("melpa" . "https://melpa.org/packages/") t)

(use-package devil
  :custom
  (devil-lighter "")
  :config
  (global-devil-mode)
  :bind
  ("C-," . #'global-devil-mode))

(use-package standard-themes
  :ensure t
  :demand t
  :config
  (if (string-equal (getenv "USER") "hdhoang")
      (load-theme 'standard-dark t)
    (load-theme 'standard-light t)))

(use-package fira-code-mode
  :hook ((fundamental-mode . fira-code-mode)))
(set-face-attribute 'default nil :height 100)

(use-package which-key
  :custom
  (which-key-lighter nil)
  :config
  (which-key-mode))
(windmove-default-keybindings 'control)
(when (display-graphic-p)
  (context-menu-mode))

(use-package kdl-ts-mode
  :defer 600
  ;; https://github.com/dataphract/kdl-ts-mode
  :load-path "../../gh/kdl-ts-mode/")

(use-package combobulate
  :load-path "../../gh/combobulate/"
  :preface (setq combobulate-key-prefix "C-c o")
  :hook ((python-ts-mode . combobulate-mode)
         (js-ts-mode . combobulate-mode)
         (css-ts-mode . combobulate-mode)
         (yaml-ts-mode . combobulate-mode)
         (json-ts-mode . combobulate-mode)))

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
(global-set-key (kbd "C-x C-z") #'bury-buffer)

(use-package diff-hl
  :config (global-diff-hl-mode t))
(use-package magit-delta
  :custom
  (magit-define-global-key-bindings 'recommended)
  (magit-diff-refine-hunk t)
  :bind ("C-c M-g l" . #'magit-log-buffer-file)
  :hook ((magit-mode . magit-delta-mode)))
(setq transient-levels '((magit-pull (transient:magit-pull:--autostash . 1))))

(use-package rainbow-delimiters
  :hook ((prog-mode . rainbow-delimiters-mode)))
(use-package expand-region
  :bind ("C-=" . #'er/expand-region))

(add-hook 'fundamental-mode-hook #'whitespace-mode)
(add-hook 'fundamental-mode-hook #'follow-mode)
(add-hook 'occur-mode-hook #'next-error-follow-minor-mode)
(add-hook 'prog-mode-hook #'display-line-numbers-mode)

(use-package treesit-ispell
  :defer 600
  :bind ("C-x s" . #'treesit-ispell-run-at-point))

(global-set-key (kbd "C-x f") #'apheleia-format-buffer) ; enable apheleia on-demand
(use-package apheleia
  :custom
  (apheleia-mode-lighter nil)
  :hook
  ((rust-ts-mode . apheleia-mode)
   ;; (yaml-ts-mode . apheleia-mode) ; it's rude to trample manifests
   (elisp-mode . apheleia-mode)
   (python-ts-mode . apheleia-mode))
  :config
  (add-to-list 'apheleia-mode-alist '(toml-ts-mode . dprint))
  (add-to-list 'apheleia-mode-alist '(rust-ts-mode . dprint))
  (add-to-list 'apheleia-mode-alist '(dockerfile-ts-mode . dprint))
  (add-to-list 'apheleia-mode-alist '(markdown-ts-mode . dprint))

  (add-to-list 'apheleia-mode-alist '(yaml-ts-mode . prettier-yaml))
  (add-to-list 'apheleia-mode-alist '(terraform-mode . terraform))
  (add-to-list 'apheleia-mode-alist '(poly-terraform-yaml-mode . terraform))

  (add-to-list 'apheleia-mode-alist '(python-mode . ruff))
  (add-to-list 'apheleia-mode-alist '(python-ts-mode . ruff)))

(use-package nov
  :defer 60)

(use-package hcl-ts-mode
  ;; https://github.com/arkbriar/hcl-ts-mode
  :load-path "../../gh/hcl-ts-mode/"
  )
(use-package terraform-mode
  :defer t)
(use-package terraform-doc
  :defer t)

(use-package polymode
  :config
  (pm-around-advice '(apheleia-format-buffer) #'polymode-with-current-base-buffer)
  ;; (defun poly-apheleia-format-chunk (beg end msg)
  ;;   (apheleia-format

  (define-hostmode poly-terraform-hostmode :mode #'terraform-mode)
  (define-innermode poly-yaml-terraform-innermode :mode #'yaml-ts-mode
    :adjust-face 5
    :head-matcher "<<EO\\(YAML\\|T\\)\n"
    :tail-matcher #'pm-same-indent-tail-matcher
    :head-mode 'host
    :tail-mode 'host
    )
  (define-polymode poly-terraform-yaml-mode :hostmode #'poly-terraform-hostmode
    :innermodes '(poly-yaml-terraform-innermode))

  (define-innermode poly-yaml-sh-innermode :mode #'bash-ts-mode
    :adjust-face 5
    :head-matcher "- |[+-]?\n"
    :tail-matcher #'pm-same-indent-tail-matcher
    :head-mode 'host
    :tail-mode 'host
    )
  (define-innermode poly-yaml-jinja2-innermode :mode #'jinja2-mode
    :adjust-face 5
    :head-matcher "^ .+[.]templates: [|>][1-9+-]*\n"
    :tail-matcher #'pm-same-indent-tail-matcher
    :head-mode 'host
    :tail-mode 'host
    )
  (define-innermode poly-yaml-conf-innermode :mode #'conf-space-mode
    :adjust-face 5
    :head-matcher "^ +\\(http\\|nginx[.]ingress[.]kubernetes[.]io/\\(server\\|configuration\\)\\)-snippet: |\n"
    :tail-matcher #'pm-same-indent-tail-matcher
    :head-mode 'host
    :tail-mode 'host
    )
  (define-hostmode poly-yaml-hostmode :mode #'yaml-ts-mode)
  (define-innermode poly-yaml-yaml-innermode :mode #'yaml-ts-mode
    ;; TBD: the whole chunk is still string-ly face
    :adjust-face 5
    :can-nest t
    :head-matcher "^  .+[.]ya?ml: |\n"
    :tail-matcher #'pm-same-indent-tail-matcher
    :head-mode 'host
    :tail-mode 'host
    )
  (define-innermode poly-yaml-toml-innermode :mode #'toml-ts-mode
    :adjust-face 5
    :head-matcher "^ +.+[.]toml: |\n"
    :tail-matcher #'pm-same-indent-tail-matcher
    :head-mode 'host
    :tail-mode 'host
    )
  (define-polymode poly-yaml-mode :hostmode #'poly-yaml-hostmode
    :innermodes '(poly-yaml-yaml-innermode
                  poly-yaml-conf-innermode
                  poly-yaml-toml-innermode
                  poly-yaml-jinja2-innermode
                  poly-yaml-sh-innermode)
    )

  :mode
  ("/k8s-manifest/.+[.]ya?ml\\'" . poly-yaml-mode)
  ("[.]tf\\'" . poly-terraform-yaml-mode)
  )
(use-package yaml-ts-mode
  :demand t
  :config
  (delete '("\\.ya?ml\\'" . yaml-ts-mode) auto-mode-alist)
  (add-to-list 'auto-mode-alist '("\\.ya?ml\\'" . yaml-ts-mode) t)
  (delete '("\\.\\(e?ya?\\|ra\\)ml\\'" . yaml-mode) auto-mode-alist)
  :mode
  ("[.]list\\'" . yaml-ts-mode)
  ("control\\'" . yaml-ts-mode)
  ("info\\'" . yaml-ts-mode))

(use-package poly-org
  :defer 10)

(use-package markdown-ts-mode
  :defer nil)
(use-package poly-markdown
  :defer 10
  :hook ((poly-markdown-mode . visual-line-mode))
  :config
  (define-hostmode poly-markdown-hostmode
    :mode 'markdown-ts-mode))

(use-package groovy-mode
  :defer t)

(use-package pcre2el)
(use-package poly-ansible
  :config
  ;; there's no #'define-auto-hostmode for this pattern
  (define-hostmode poly-json-hostmode :mode #'json-ts-mode)
  (define-polymode poly-json-j2-mode :hostmode #'poly-json-hostmode :innermodes '(pm-inner/jinja2))
  (define-hostmode poly-conf-hostmode :mode #'conf-mode)
  (define-polymode poly-conf-j2-mode :hostmode #'poly-conf-hostmode :innermodes '(pm-inner/jinja2))
  (define-hostmode poly-xml-hostmode :mode #'nxml-mode)
  (define-polymode poly-xml-j2-mode :hostmode #'poly-xml-hostmode :innermodes '(pm-inner/jinja2))

  :mode
  ("[.]\\(env\\|service\\|conf\\|properties\\)[.]j2\\'" . poly-conf-j2-mode)
  ("[.]json[.]j2\\'" . poly-json-j2-mode)
  ("[.]xml[.]j2\\'" . poly-xml-j2-mode)

  ("[.]ya?ml[.]j2\\'" . poly-ansible-mode))
(use-package company-ansible)

(use-package orderless
  :custom (completion-styles '(basic partial-completion emacs22)))
(use-package avy
  :custom
  (avy-all-windows 'all-frames)
  :bind (("C-c j" . avy-goto-line)
         ("C-c z" . avy-goto-word-1)
         ("s-j"   . avy-goto-char-timer)))
(global-set-key [remap dabbrev-expand] #'hippie-expand)

;; Marginalia: annotations for minibuffer
(use-package marginalia
  :config
  (marginalia-mode))

(use-package eglot
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
(use-package just-mode)
(use-package justl)

(add-to-list 'magic-mode-alist '("^$TTL" . zone-mode))
(add-to-list 'magic-mode-alist '("^$ORIGIN" . zone-mode))
(add-to-list 'magic-mode-alist '("^# syntax=docker" . dockerfile-ts-mode))

;; after modes are ready
(desktop-save-mode t)
(server-start)

(setq gc-cons-threshold (or bedrock--initial-gc-threshold 800000))
;; refresh grammars
;; (dolist (grammar treesit-language-source-alist) (treesit-install-language-grammar (car grammar)))
