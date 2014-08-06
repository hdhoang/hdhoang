; ~/.emacs.d/init.el
(funcall #'package-initialize)
(require 'org)
(org-babel-load-file
 (expand-file-name "emacs.org"
		   (file-name-directory (file-truename user-init-file))))
