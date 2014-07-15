; ~/.emacs.d/init.el
(add-hook 'after-init-hook
	  (lambda ()
	    (org-babel-load-file
	     (expand-file-name "emacs.org"
			       (file-name-directory (file-truename user-init-file))))))
