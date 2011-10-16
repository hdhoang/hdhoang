; -*- mode: lisp -*-

(defalias 'rb 'revert-buffer)
(global-set-key (kbd "C-\\") 'toggle-input-method)

(setq frame-title-format "%b")

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
(custom-set-variables
 ;; custom-set-variables was added by Custom.
 ;; If you edit it by hand, you could mess it up, so be careful.
 ;; Your init file should contain only one such instance.
 ;; If there is more than one, they won't work right.
 '(default-input-method "vietnamese-telex")
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
