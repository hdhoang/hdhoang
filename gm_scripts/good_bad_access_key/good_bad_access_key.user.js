// ==UserScript==
// @name           Good Bad access key
// @namespace      zahe.me/hdhoang
// @description    add accesskey to (y)ape normal training page
// @include        http://hb4pm4eznzhd6mts.onion/pe/train.php
// ==/UserScript==

// sometimes the accesskeys are added but can't be triggered

// can't use b because of fx's bookmark menu
// mimicking vi movement, k/up/good, j/down/bad
document.querySelector('[value="Good"]').accessKey = "k";
document.querySelector('[value="Bad"]').accessKey = "j";
