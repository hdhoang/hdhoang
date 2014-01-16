use selfoss;

update items set unread = 0 where
   (link like 'http://decorumcomics.com/%'
         and link < 'http://decorumcomics.com/comic.php?id=165')
or (link like 'http://drawingboardcomic.com/%'
         and link < 'http://drawingboardcomic.com/index.php?comic=205')
or (link like 'http://www.giantitp.com/%'
         and link < 'http://www.giantitp.com/comics/oots0935.html')
or source = (select id from sources where title like 'oglaf%');
