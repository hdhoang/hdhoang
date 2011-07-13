// ==UserScript==
// @name        everySub
// @namespace	http://erinsparling.com/everySub/
// @description everySub style for Google Reader
// @include     https://*.google.com/reader/view/*
// @include     http://*.google.com/reader/view/*
// @include     htt*://*.google.*/reader/view*
// @author      everySub design by Khoi Vinh, css/js by Erin Sparling, with points of origins near http://www.hicksdesign.co.uk
// @require     http://ajax.googleapis.com/ajax/libs/jquery/1.5.2/jquery.min.js
// ==/UserScript==

var favvy = document.createElement('link');
favvy.setAttribute('type', 'image/x-icon');
favvy.setAttribute('rel', 'shortcut icon');
favvy.setAttribute('href', 'http://erinsparling.com/everySub/img/favicon-g.ico');
var head = document.getElementsByTagName('head')[0];
head.appendChild(favvy);

var cssNode = document.createElement('link');
cssNode.type = 'text/css';
cssNode.rel = 'stylesheet';
cssNode.href = 'http://erinsparling.com/everySub/css/everySub.css';
cssNode.media = 'screen';
cssNode.title = 'dynamicLoadedSheet';
document.getElementsByTagName("head")[0].appendChild(cssNode);

(function() {

 		//layers toggle
		$("#logo-container").removeAttr("href");
		
		$("#logo-container").click(function() {
			$("body").toggleClass("fancy");
		});
		
		$("#viewer-all-new-links").after("<div id='moving-viewer-all-new-links'/>");
		$("#moving-viewer-all-new-links").append($("#show-new"),$("#show-all"));
		$("#viewer-all-new-links").html("Show:");		

		// $("#stream-prefs-menu").after("<div id='moving-viewer-footer'/>");
		// $("#moving-viewer-footer").append($("#viewer-footer"));
			

}());