; -*- mode: lisp -*-
; $h/emacs-prelude/personal/custom.el
(server-start)

(custom-set-variables
 ;; custom-set-variables was added by Custom.
 ;; If you edit it by hand, you could mess it up, so be careful.
 ;; Your init file should contain only one such instance.
 ;; If there is more than one, they won't work right.
 '(current-language-environment "UTF-8")
 '(custom-safe-themes (quote ("d677ef584c6dfc0697901a44b885cc18e206f05114c8a3b7fde674fce6180879" default)))
 '(default-input-method "vietnamese-telex")
 '(electric-pair-mode nil)
 '(explicit-shell-file-name "/bin/zsh")
 '(frame-title-format "%f")
 '(global-hl-line-mode nil)
 '(ido-use-virtual-buffers t)
 '(indicate-empty-lines t)
 '(org-M-RET-may-split-line nil)
 '(pkgbuild-user-mail-address "arch@zahe.me")
 '(sentence-end-double-space nil)
 '(tool-bar-mode t)
 '(user-full-name "Hоàng Đức Hiếu")
 '(vc-follow-symlinks t)
 '(windmove-wrap-around t))
(custom-set-faces
 ;; custom-set-faces was added by Custom.
 ;; If you edit it by hand, you could mess it up, so be careful.
 ;; Your init file should contain only one such instance.
 ;; If there is more than one, they won't work right.
 )

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

(remove-hook 'text-mode-hook 'flyspell-mode)
(add-hook 'prog-mode-hook 'turn-off-flyspell t)
(add-hook 'prog-mode-hook 'whitespace-turn-off t)
(add-hook 'prog-mode-hook 'turn-off-guru-mode t)

(load-theme 'solarized-light)

(if (eq window-system-version 6)
    (set-default-font "Consolas-11"))
