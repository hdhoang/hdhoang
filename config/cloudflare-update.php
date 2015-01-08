<?php 
// /srv/http/cloudflare-update.php
print $ip = rtrim(file_get_contents('https://icanhazip.com'));
print file_get_contents('https://cloudflare.com/api_json.html?z=zahe.me&a=rec_edit&id=149862094&type=A&name=@&ttl=1&email=...&tkn=...&content=' . $ip);
?>