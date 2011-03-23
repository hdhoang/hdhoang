// ==UserScript==
// @name           Un-UTM Location Bar
// @description    Strip UTM parameters from the location bar
// @include        http://*utm_*
// @include        https://*utm_*
// @version        1.02
// ==/UserScript==

(function() {

  var loc = window.location.toString();
    
  loc = loc.replace(/utm_.*?(&|$)/g,'');
  loc = loc.replace(/\?$/,'');

  // If browser supports html5 history change, use it
  // so the page won't reload
  if (window.history.replaceState) {
    window.history.replaceState('Removed UTM Parameters', '', loc)
  } else {
    window.location = loc;
  }

})();