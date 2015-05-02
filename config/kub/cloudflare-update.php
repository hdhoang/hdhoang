<?php 
// /srv/http/cloudflare-update.php
print $ip = rtrim(file_get_contents('https://vorme/wanip.sh', false,
  stream_context_create(array('ssl' =>
  array('verify_peer' => false, 'verify_peer_name' => false)))));
print file_get_contents('https://cloudflare.com/api_json.html?z=zahe.me&a=rec_edit&id=149862094&type=A&name=@&ttl=1&email=...&tkn=...&content=' . $ip);
?>
