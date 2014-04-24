; -*- mode: lisp -*-
; $h/emacs-prelude/personal/custom.el
(server-start)

(custom-set-variables
 ;; custom-set-variables was added by Custom.
 ;; If you edit it by hand, you could mess it up, so be careful.
 ;; Your init file should contain only one such instance.
 ;; If there is more than one, they won't work right.
 '(current-language-environment "UTF-8")
 '(custom-safe-themes
   (quote
    ("d677ef584c6dfc0697901a44b885cc18e206f05114c8a3b7fde674fce6180879" default)))
 '(default-input-method "vietnamese-telex")
 '(desktop-load-locked-desktop t)
 '(desktop-save-mode t)
 '(electric-pair-mode nil)
 '(explicit-shell-file-name "/bin/zsh")
 '(frame-title-format "%f" t)
 '(ido-use-virtual-buffers t)
 '(indicate-empty-lines t)
 '(initial-buffer-choice (quote remember-notes))
 '(org-M-RET-may-split-line nil)
 '(org-log-done nil)
 '(org-use-speed-commands t)
 '(pkgbuild-user-mail-address "arch@zahe.me")
 '(prelude-guru nil)
 '(sentence-end-double-space nil)
 '(solarized-use-more-italic t)
 '(solarized-use-variable-pitch nil)
 '(sql-connection-alist
   (quote
    (("selfoss"
      (sql-product
       (quote mysql))
      (sql-user "selfoss")
      (sql-database "selfoss")
      (sql-server "192.168.1.54")))))
 '(sql-mysql-options (quote ("mysql")))
 '(sql-mysql-program "fakecygpty")
 '(tool-bar-mode t)
 '(user-full-name "Hоàng Đức Hiếu")
 '(vc-follow-symlinks t)
 '(whitespace-style (quote (face tabs empty trailing)))
 '(windmove-wrap-around t))
(custom-set-faces
 ;; custom-set-faces was added by Custom.
 ;; If you edit it by hand, you could mess it up, so be careful.
 ;; Your init file should contain only one such instance.
 ;; If there is more than one, they won't work right.
 )

(global-set-key (kbd "C-M-h") 'backward-kill-word)
(global-set-key (kbd "C-\\") 'toggle-input-method)
(global-set-key (kbd "C-x M-t") 'ansi-term)
(global-set-key (kbd "C-x <up>") 'windmove-up)
(global-set-key (kbd "C-x <down>") 'windmove-down)

;; Major modes
(autoload 'pkgbuild-mode "pkgbuild-mode" "" t)
(add-to-list 'auto-mode-alist '("PKGBUILD" . pkgbuild-mode))
(add-to-list 'auto-mode-alist '("\\.install\\'" . sh-mode))
(add-to-list 'auto-mode-alist '("\\.do\\'" . sh-mode))
(mapcar (lambda (ext)
          (add-to-list 'auto-mode-alist
                       (cons (concat "\\." ext "\\'") 'conf-windows-mode)))
        '("service" "socket" "desktop" "directory"))
(add-hook 'conf-windows-mode-hook
          '(lambda () (setq comment-start "#") (glasses-mode t)))

(require 'prelude-ido)

(prelude-require-package 'solarized-theme)
(load-theme 'solarized-light)

(if (eq window-system-version 6)
    (set-default-font "Consolas-11"))
(if (eq system-type 'windows-nt)
    (custom-set-variables
     '(sql-mysql-options (quote ("mysql")))
     '(sql-mysql-program "fakecygpty")))

(remove-hook 'mouse-leave-buffer-hook 'prelude-auto-save-command)
(remove-hook 'git-commit-mode-hook 'flyspell-mode)
