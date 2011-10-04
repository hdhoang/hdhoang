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
 '(column-number-mode t)
 '(indicate-empty-lines t)
 '(inhibit-startup-screen t)
 '(initial-scratch-message nil)
 '(magit-remote-ref-format (quote remote-slash-branch))
 '(org-startup-folded nil)
 '(package-archives (quote (("marmalade" . "http://marmalade-repo.org/packages/") ("gnu" . "http://elpa.gnu.org/packages/") ("tromey" . "http://tromey.com/elpa/"))))
 '(pkgbuild-user-mail-address "arch@zahe.me")
 '(require-final-newline t)
 '(size-indication-mode t)
 '(uniquify-buffer-name-style (quote post-forward-angle-brackets) nil (uniquify))
 '(user-full-name "Hoàng Đức Hiếu")
 '(vc-follow-symlinks t))

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
