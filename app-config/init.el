; ~/.emacs.d/init.el
(defun ensure-package (package)
  (unless (package-installed-p package)
    (package-install package)))
(package-initialize)

(ensure-package 'smex)
(ensure-package 'evil)
(evil-mode 1)
(dolist (state '(normal motion))
  (evil-define-key state global-map
    ":" 'smex
    (kbd "<SPC>") 'evil-scroll-down
    (kbd "S-<SPC>") 'evil-scroll-up
    ))
(dolist (state '(insert motion normal))
  (evil-define-key state global-map
    (kbd "C-d") 'delete-char
    (kbd "C-y") 'evil-paste-after
    (kbd "M-y") 'evil-paste-pop
    (kbd "M-Y") 'evil-paste-pop-next
    (kbd "C-e") 'end-of-line
    (kbd "C-n") 'next-line
    (kbd "C-p") 'previous-line
    ))
(evil-set-initial-state 'magit-log-edit-mode 'insert)

(defalias 'yes-or-no-p 'y-or-n-p)
(global-set-key (kbd "M-x") 'smex)
(global-set-key (kbd "M-/") 'hippie-expand)
(global-set-key (kbd "C-M-h") 'backward-kill-word)
(global-set-key (kbd "C-\\") 'toggle-input-method)
(global-set-key (kbd "C-x M-t") 'term)
(global-set-key (kbd "C-x M-m") 'shell)
(global-set-key (kbd "C-x p") 'proced)
(global-set-key (kbd "C-x g") 'magit-status)
(global-set-key (kbd "C-v") 'evil-scroll-down)
(global-set-key (kbd "M-v") 'evil-scroll-up)

;; Major modes
(dolist (assoc '(("PKGBUILD" . pkgbuild-mode)
		 ("\\.install\\'" . sh-mode)
		 ("\\.do\\'" . sh-mode)
		 ("\\.service\\'" . conf-windows-mode)
		 ("\\.socket\\'" . conf-windows-mode)
		 ("\\.timer\\'" . conf-windows-mode)
		 ("\\.directory\\'" . conf-windows-mode)
		 ))
  (add-to-list 'auto-mode-alist assoc))
(add-hook 'conf-windows-mode-hook
          '(lambda ()
             (setq comment-start "#")
             (glasses-mode t)))
(if (eq system-type 'windows-nt)
    (setq sql-mysql-options '("mysql")
	  sql-mysql-program "fakecygpty"
	  ))

;; Appearance
(ensure-package 'color-theme-solarized)
(load-theme 'solarized-light t)

(if (eq window-system-version 6)
    (set-default-font "Consolas-11"))

(add-hook 'post-command-hook
          '(lambda ()
             (unless window-system
	       (send-string-to-terminal (concat "\033]2; " (buffer-name) "\007")))))

(custom-set-variables
 ;; custom-set-variables was added by Custom.
 ;; If you edit it by hand, you could mess it up, so be careful.
 ;; Your init file should contain only one such instance.
 ;; If there is more than one, they won't work right.
 '(backup-directory-alist (quote (("." . "~/.emacs.d/backups"))))
 '(blink-cursor-mode nil)
 '(current-language-environment "UTF-8")
 '(default-input-method "vietnamese-telex")
 '(desktop-load-locked-desktop t)
 '(desktop-save-mode t)
 '(erc-autojoin-channels-alist (quote (("freenode" "#lojban" "#jbopre" "#vnluser"))))
 '(erc-hide-list (quote ("JOIN" "PART" "QUIT")))
 '(erc-prompt-for-password nil)
 '(explicit-shell-file-name "zsh")
 '(frame-title-format "%f" t)
 '(ido-mode (quote both) nil (ido))
 '(ido-use-virtual-buffers t)
 '(indicate-empty-lines t)
 '(initial-buffer-choice (quote remember-notes))
 '(org-M-RET-may-split-line nil)
 '(org-babel-load-languages (quote ((emacs-lisp . t) (sql . t))))
 '(org-log-done nil)
 '(org-src-fontify-natively t)
 '(org-use-speed-commands t)
 '(package-archives
   (quote
    (("gnu" . "http://elpa.gnu.org/packages/")
     ("marmalade" . "http://marmalade-repo.org/packages/"))))
 '(pkgbuild-user-mail-address "arch@zahe.me")
 '(proced-filter (quote all))
 '(proced-tree-flag t)
 '(sentence-end-double-space nil)
 '(server-mode t)
 '(show-paren-mode t)
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
 '(user-full-name "Hоàng Đức Hiếu")
 '(user-mail-address "hdhoang@zahe.me")
 '(vc-follow-symlinks t)
 '(whitespace-style (quote (face tabs empty trailing)))
 '(windmove-wrap-around t)
 '(xterm-mouse-mode t))
(custom-set-faces
 ;; custom-set-faces was added by Custom.
 ;; If you edit it by hand, you could mess it up, so be careful.
 ;; Your init file should contain only one such instance.
 ;; If there is more than one, they won't work right.
 )
