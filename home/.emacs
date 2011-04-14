; -*- mode: lisp -*-

(global-set-key [mouse-2] 'mouse-yank-primary)
(global-set-key (kbd "RET") 'reindent-then-newline-and-indent)
(global-set-key (kbd "C-x o") 'find-file-at-point)
(global-set-key (kbd "C-.") 'other-window)
(defalias 'rb 'revert-buffer)

(defun hdh/revert-some-ergoemacs ()
  (tool-bar-mode t)

  (global-set-key (kbd "C-x d") 'dired)
  (global-set-key (kbd "C-x h") 'mark-whole-buffer)
  (global-set-key (kbd "C-p") 'previous-line)
  (global-set-key (kbd "C-n") 'next-line)
  (global-set-key (kbd "C-f") 'forward-char)
  (global-set-key (kbd "C-b") 'backward-char)
  (global-set-key (kbd "M-f") 'forward-word)
  (global-set-key (kbd "M-b") 'backward-word)
  (global-set-key (kbd "M-<") 'beginning-of-buffer)
  (global-set-key (kbd "M->") 'end-of-buffer)
  (global-set-key (kbd "C-a") 'move-beginning-of-line)
  (global-set-key (kbd "C-e") 'move-end-of-line)
  (global-set-key (kbd "C-t") 'transpose-chars)
  (global-set-key (kbd "C-y") 'cua-paste)
  (global-set-key (kbd "C-d") 'delete-char)
  (global-set-key (kbd "M-;") 'comment-dwim))
(add-hook 'ergoemacs-mode-hook 'hdh/revert-some-ergoemacs)
(load-file "~/.emacs.d/ergoemacs/site-lisp/site-start.el")
(eval-after-load "ergoemacs/site-lisp/site-start.el" (tabbar-mode 0))
(defun hdh/ergoemacs-minibuffer-keys ()
  (local-set-key (kbd "RET") 'minibuffer-complete-and-exit)
  (local-set-key ergoemacs-previous-line-key 'previous-history-element)
  (local-set-key ergoemacs-next-line-key 'next-history-element))
(ergoemacs-add-hook 'minibuffer-setup-hook 'hdh/ergoemacs-minibuffer-keys)
(defun hdh/ergoemacs-comint-keys ()
  (local-set-key ergoemacs-scroll-down-key 'comint-previous-input)
  (local-set-key ergoemacs-scroll-up-key 'comint-next-input))
(ergoemacs-add-hook 'comint-mode-hook 'hdh/ergoemacs-comint-keys)
(defun hdh/ergoemacs-ido-keys ()
  (local-set-key ergoemacs-scroll-down-key 'ido-prev-work-directory)
  (local-set-key ergoemacs-scroll-up-key 'ido-magic-forward-char))
(ergoemacs-add-hook 'ido-minibuffer-setup-hook 'hdh/ergoemacs-ido-keys)

(custom-set-variables
  ;; custom-set-variables was added by Custom.
  ;; If you edit it by hand, you could mess it up, so be careful.
  ;; Your init file should contain only one such instance.
  ;; If there is more than one, they won't work right.
 '(TeX-PDF-mode t)
 '(TeX-engine (quote xetex))
 '(backup-directory-alist (quote (("." . "~/.emacs.d/backups"))))
 '(blink-cursor-mode nil)
 '(column-number-mode t)
 '(cua-mode t nil (cua-base))
 '(current-language-environment "UTF-8")
 '(default-buffer-file-coding-system (quote utf-8-unix) t)
 '(default-input-method "vietnamese-telex")
 '(describe-char-unidata-list (quote (name old-name general-category canonical-combining-class bidi-class decomposition decimal-digit-value digit-value numeric-value mirrored iso-10646-comment uppercase lowercase titlecase)))
 '(doc-view-continuous t)
 '(font-use-system-font t)
 '(ibus-python-shell-command-name "python2")
 '(ido-create-new-buffer (quote always))
 '(ido-enable-flex-matching t)
 '(ido-mode (quote both) nil (ido))
 '(indent-tabs-mode t)
 '(inhibit-startup-screen t)
 '(initial-scratch-message nil)
 '(make-pointer-invisible nil)
 '(mouse-avoidance-mode nil nil (avoid))
 '(mouse-avoidance-nudge-dist 30)
 '(package-archives (quote (("marmalade" . "http://marmalade-repo.org/packages/") ("gnu" . "http://elpa.gnu.org/packages/") ("tromey" . "http://tromey.com/elpa/"))))
 '(read-quoted-char-radix 16)
 '(reb-re-syntax (quote string))
 '(save-place t nil (saveplace))
 '(save-place-file "~/.emacs.d/places")
 '(select-active-regions t)
 '(show-paren-mode t)
 '(size-indication-mode t)
 '(text-mode-hook (quote (turn-on-auto-fill text-mode-hook-identify)))
 '(uniquify-buffer-name-style (quote post-forward-angle-brackets) nil (uniquify))
 '(vc-follow-symlinks t)
 '(x-select-enable-clipboard t))

(custom-set-faces
  ;; custom-set-faces was added by Custom.
  ;; If you edit it by hand, you could mess it up, so be careful.
  ;; Your init file should contain only one such instance.
  ;; If there is more than one, they won't work right.
 '(default ((t (:inherit nil :stipple nil :background "white" :foreground "black" :inverse-video nil :box nil :strike-through nil :overline nil :underline nil :slant normal :weight normal :height 113 :width normal :foundry "unknown" :family "monospace"))))
 '(completions-common-part ((t (:inherit default :foreground "red"))))
 '(diredp-ignored-file-name ((t (:foreground "#bebebe"))))
 '(isearch ((((class color) (min-colors 88) (background light)) (:background "black" :foreground "white"))))
 '(show-paren-match ((((class color) (background light)) (:background "azure2")))))

;; Major modes
(autoload 'pkgbuild-mode "pkgbuild-mode" "" t)
(add-to-list 'auto-mode-alist '("PKGBUILD" . pkgbuild-mode))
(add-to-list 'auto-mode-alist '("\\.install\\'" . sh-mode))
(add-to-list 'auto-mode-alist '("\\.do\\'" . sh-mode))
(mapcar (lambda (ext)
	  (add-to-list 'auto-mode-alist (cons (concat "\\." ext "\\'") 'conf-windows-mode)))
	'("service" "socket" "desktop" "directory"))
(add-hook 'conf-windows-mode-hook '(lambda () (setq comment-start "#") (glasses-mode t)))

(autoload 'magit-status "magit" nil t)
(global-set-key (kbd "C-x S") 'magit-status)
