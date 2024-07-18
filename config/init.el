;; bedrock early-init.el
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
     (hcl-mode . hcl-ts-mode)
     (js-json-mode . json-ts-mode)
     (python-mode . python-ts-mode)
     (rust-mode . rust-ts-mode)
     (sh-mode . bash-ts-mode)
     (yaml-mode . yaml-ts-mode)))
 '(menu-bar-mode t)
 '(package-selected-packages
   '(polymode hcl-ts-mode company-ansible terraform-doc terraform-mode treesit-ispell kdl-ts-mode pcre2el apheleia justl just-mode marginalia avy rustic which-key orderless fira-code-mode combobulate treesit expand-region groovy-mode magit-delta rainbow-delimiters use-package poly-ansible poly-markdown poly-org))
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
     (gosum "https://github.com/tree-sitter-grammars/tree-sitter-go-sum")
     (hcl "https://github.com/tree-sitter-grammars/tree-sitter-hcl")
     (html "https://github.com/tree-sitter/tree-sitter-html")
     (javascript "https://github.com/tree-sitter/tree-sitter-javascript" "master" "src")
     (json "https://github.com/tree-sitter/tree-sitter-json")
     (kdl "https://github.com/tree-sitter-grammars/tree-sitter-kdl")
     (python "https://github.com/tree-sitter/tree-sitter-python")
     (rust "https://github.com/tree-sitter/tree-sitter-rust")
     (toml "https://github.com/tree-sitter/tree-sitter-toml")
     (yaml "https://github.com/tree-sitter-grammars/tree-sitter-yaml")) t)
 '(warning-suppress-types '((use-package)))
 '(whitespace-style
   '(face trailing tabs missing-newline-at-eof indentation::space))
 '(x-underline-at-descent-line nil))
;; refresh grammars
;; (dolist (grammar treesit-language-source-alist) (treesit-install-language-grammar (car grammar)))

(global-unset-key (kbd "C-x m"))
(global-auto-revert-mode)

(require 'package)
(add-to-list 'package-archives '("melpa" . "https://melpa.org/packages/") t)

(use-package fira-code-mode
  :ensure
  :hook ((fundamental-mode . fira-code-mode)))
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

(use-package kdl-ts-mode
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

(use-package treesit-ispell
  :ensure
  :bind ("C-x s" . #'treesit-ispell-run-at-point))

(global-set-key (kbd "C-x f") #'apheleia-format-buffer) ; enable apheleia on-demand
(use-package apheleia
  :ensure
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
  (add-to-list 'apheleia-mode-alist '(markdown-mode . dprint))

  (add-to-list 'apheleia-mode-alist '(yaml-ts-mode . prettier-yaml))
  (add-to-list 'apheleia-mode-alist '(poly-terraform-yaml-mode . terraform))

  (add-to-list 'apheleia-mode-alist '(python-mode . ruff))
  (add-to-list 'apheleia-mode-alist '(python-ts-mode . ruff)))

(use-package hcl-ts-mode
  ;; https://github.com/arkbriar/hcl-ts-mode
  :load-path "../../gh/hcl-ts-mode/"
  )
(use-package terraform-mode
  :ensure
  :config
  (add-to-list 'apheleia-mode-alist '(terraform-mode . terraform)))
(use-package terraform-doc
  :ensure)

(use-package polymode
  :ensure
  :config
  (pm-around-advice '(apheleia-format-buffer) #'polymode-with-current-base-buffer)
  ;; (defun poly-apheleia-format-chunk (beg end msg)
  ;;   (apheleia-format

  (define-hostmode poly-terraform-hostmode :mode #'terraform-mode)
  (define-innermode poly-yaml-terraform-innermode :mode #'yaml-ts-mode
    :adjust-face 5
    :head-matcher "<<EO\\(YAML\\|T\\)\n"
    :tail-matcher " +EO\\(YAML\\|T\\)"
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
    :can-nest nil
    :head-matcher "^  .+[.]yaml: |\n"
    :tail-matcher #'pm-same-indent-tail-matcher
    :head-mode 'host
    :tail-mode 'host
    )
  (define-polymode poly-yaml-mode :hostmode #'poly-yaml-hostmode
    :innermodes '(poly-yaml-yaml-innermode
                  poly-yaml-conf-innermode
                  poly-yaml-jinja2-innermode
                  poly-yaml-sh-innermode)
    )

  :mode
  ("/k8s-manifest/.+[.]ya?ml\\'" . poly-yaml-mode)
  ("[.]tf\\'" . poly-terraform-yaml-mode)
  )
(use-package yaml-ts-mode
  :mode
  ("[.]list\\'" . yaml-ts-mode)
  ("control\\'" . yaml-ts-mode)
  ("info\\'" . yaml-ts-mode))
(delete '("\\.ya?ml\\'" . yaml-ts-mode) auto-mode-alist)
(use-package poly-org
  :ensure)
(use-package poly-markdown
  :ensure
  :hook ((poly-markdown-mode . visual-line-mode)))

(use-package groovy-mode
  :ensure)

(use-package pcre2el
  :ensure)
(use-package poly-ansible
  :ensure
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
(use-package company-ansible
  :ensure)

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
