use selfoss;

create procedure read_all (pattern char(30))
       update items set unread = 0 where link like concat('%',pattern,'%');
create procedure mark_5_unread (pattern char(30))
       update items set unread = 1 where link like concat('%',pattern,'%')
                    limit 5;

call read_all('drawingboard');
call mark_5_unread('drawingboard');
call read_all('oglaf');
call mark_5_unread('oglaf');
call read_all('oots');
call mark_5_unread('oots');
call read_all('satw');
call mark_5_unread('satw');
call read_all('stupidfox');
call mark_5_unread('stupidfox');
