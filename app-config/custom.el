; -*- mode: lisp -*-

(custom-set-variables
 ;; custom-set-variables was added by Custom.
 ;; If you edit it by hand, you could mess it up, so be careful.
 ;; Your init file should contain only one such instance.
 ;; If there is more than one, they won't work right.
 '(custom-safe-themes (quote ("baed08a10ff9393ce578c3ea3e8fd4f8c86e595463a882c55f3bd617df7e5a45" "485737acc3bedc0318a567f1c0f5e7ed2dfde3fb" default)))
 '(default-input-method "vietnamese-telex")
 '(electric-pair-mode nil)
 '(ido-use-virtual-buffers t)
 '(indicate-empty-lines t)
 '(pkgbuild-user-mail-address "arch@zahe.me")
 '(sentence-end-double-space nil)
 '(user-full-name "Hоàng Đức Hiếu")
 '(vc-follow-symlinks t))
(custom-set-faces
 ;; custom-set-faces was added by Custom.
 ;; If you edit it by hand, you could mess it up, so be careful.
 ;; Your init file should contain only one such instance.
 ;; If there is more than one, they won't work right.
 )

(defalias 'rb 'revert-buffer)
(global-set-key (kbd "C-\\") 'toggle-input-method)

(setq frame-title-format "%f")

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

(remove-hook 'text-mode-hook 'prelude-turn-on-flyspell)
(add-hook 'prog-mode-hook 'prelude-turn-off-whitespace t)
(prelude-restore-arrow-keys)

(load-theme 'solarized-light)
