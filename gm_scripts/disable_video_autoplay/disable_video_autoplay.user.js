// ==UserScript==
// @name           Disable audio/video autoplay
// @namespace      http://diveintomark.org/projects/greasemonkey/
// @description    Ensures that HTML5 audio/video elements do not autoplay
// @include        *
// ==/UserScript==

var arAudioVideos = document.getElementsByTagName('audio') + document.getElementsByTagName('video');
for (var i = arAudioVideos.length - 1; i >= 0; i--) {
    var elm = arAudioVideos[i];
    elm.autoplay = false;
}