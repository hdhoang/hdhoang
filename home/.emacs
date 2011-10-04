; -*- mode: lisp -*-

(defalias 'rb 'revert-buffer)
(defalias 'dabbrev-expand 'hippie-expand)

(load "~/.emacs.d/init")

(setq frame-title-format "%b")
(custom-set-variables
 ;; custom-set-variables was added by Custom.
 ;; If you edit it by hand, you could mess it up, so be careful.
 ;; Your init file should contain only one such instance.
 ;; If there is more than one, they won't work right.
 '(TeX-PDF-mode t)
 '(TeX-engine (quote xetex))
 '(backup-directory-alist (quote (("." . "~/.emacs.d/backups"))))
 '(blink-cursor-mode nil)
 '(calendar-date-style (quote iso))
 '(calendar-week-start-day 1)
 '(column-number-mode t)
 '(cua-mode t nil (cua-base))
 '(current-language-environment "UTF-8")
 '(default-buffer-file-coding-system (quote utf-8-unix) t)
 '(default-input-method "vietnamese-telex")
 '(describe-char-unidata-list (quote (name old-name general-category canonical-combining-class bidi-class decomposition decimal-digit-value digit-value numeric-value mirrored iso-10646-comment uppercase lowercase titlecase)))
 '(doc-view-continuous t)
 '(electric-indent-mode t)
 '(electric-layout-mode t)
 '(electric-pair-mode t)
 '(font-use-system-font t)
 '(ibus-python-shell-command-name "python2")
 '(ido-create-new-buffer (quote always))
 '(ido-enable-flex-matching t)
 '(ido-mode (quote both) nil (ido))
 '(indent-tabs-mode nil)
 '(indicate-empty-lines t)
 '(inhibit-startup-screen t)
 '(initial-scratch-message nil)
 '(magit-remote-ref-format (quote remote-slash-branch))
 '(make-pointer-invisible nil)
 '(mouse-avoidance-mode nil nil (avoid))
 '(mouse-avoidance-nudge-dist 30)
 '(org-M-RET-may-split-line (quote ((default))))
 '(org-startup-folded nil)
 '(package-archives (quote (("marmalade" . "http://marmalade-repo.org/packages/") ("gnu" . "http://elpa.gnu.org/packages/") ("tromey" . "http://tromey.com/elpa/"))))
 '(pkgbuild-user-mail-address "arch@zahe.me")
 '(ps-paper-type (quote a4))
 '(read-quoted-char-radix 16)
 '(reb-re-syntax (quote string))
 '(require-final-newline t)
 '(save-place t nil (saveplace))
 '(save-place-file "~/.emacs.d/places")
 '(select-active-regions t)
 '(selection-coding-system (quote utf-8))
 '(show-paren-mode t)
 '(show-paren-style (quote mixed))
 '(size-indication-mode t)
 '(tab-width 4)
 '(text-mode-hook (quote (turn-on-auto-fill text-mode-hook-identify)))
 '(truncate-partial-width-windows nil)
 '(uniquify-buffer-name-style (quote post-forward-angle-brackets) nil (uniquify))
 '(user-full-name "Hoàng Đức Hiếu")
 '(vc-follow-symlinks t)
 '(x-select-enable-clipboard t))

(custom-set-faces
 ;; custom-set-faces was added by Custom.
 ;; If you edit it by hand, you could mess it up, so be careful.
 ;; Your init file should contain only one such instance.
 ;; If there is more than one, they won't work right.
 '(completions-common-part ((t (:inherit default :foreground "red"))))
 '(diredp-compressed-file-suffix ((t (:foreground "#7b68ee"))))
 '(diredp-ignored-file-name ((t (:foreground "#aaaaaa"))))
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
(global-set-key (kbd "C-c m") 'magit-status)
