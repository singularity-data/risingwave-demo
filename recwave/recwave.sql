-- the user history table
create table actionhistory (
  id integer primary key,
  userid varchar(30),
  itemid varchar(30),
  action integer,  -- 0: view 1: click 2: purchase
  timestamp_ bigint
);

insert into actionhistory values
    (1, 'user1', 'item1', 0, 123456789),
    (2, 'user1', 'item1', 1, 123456790),
    (3, 'user1', 'item2', 0, 123456791),
    (4, 'user1', 'item2', 0, 123456792),
    (5, 'user1', 'item2', 1, 123456793),
    (6, 'user2', 'item1', 0, 123456794),
    (7, 'user2', 'item1', 0, 123456795),
    (8, 'user2', 'item2', 0, 123456796),
    (9, 'user2', 'item2', 0, 123456797),
    (10, 'user2', 'item2', 0, 123456798),
    (11, 'user2', 'item2', 1, 123456799),
    (12, 'user2', 'item2', 1, 123456800);


-- recent most viewed items

create materialized view user_most_interacted_item as
select max(userid), max(itemid), max(count) as max_count from
 (select userid, itemid, count(itemid) over (partition by userid) as count
 from actionhistory where timestamp_ > max(timestamp_) - 86400000
    group by itemid, userid) max_counts
group by userid;


-- recent most interacted user for each item
create materialized view item_most_accessed_user as
select max(itemid), max(userid), max(count) as max_count from
 (select userid, itemid, count(itemid) over (partition by itemid) as count
 from actionhistory where timestamp_ > max(timestamp_) - 86400000
    group by itemid, userid) max_counts
group by itemid;

