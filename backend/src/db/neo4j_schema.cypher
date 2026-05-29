// Node constraints
CREATE CONSTRAINT event_uuid IF NOT EXISTS FOR (e:Event) REQUIRE e.uuid IS UNIQUE;
CREATE CONSTRAINT actor_uuid IF NOT EXISTS FOR (a:Actor) REQUIRE a.uuid IS UNIQUE;
CREATE CONSTRAINT location_uuid IF NOT EXISTS FOR (l:Location) REQUIRE l.uuid IS UNIQUE;

// Search indexes
CREATE INDEX event_title IF NOT EXISTS FOR (e:Event) ON (e.title);
CREATE INDEX actor_name IF NOT EXISTS FOR (a:Actor) ON (a.name);
CREATE INDEX location_name IF NOT EXISTS FOR (l:Location) ON (l.name);
CREATE INDEX event_hijri_year IF NOT EXISTS FOR (e:Event) ON (e.hijri_year);
CREATE INDEX event_gregorian_year IF NOT EXISTS FOR (e:Event) ON (e.gregorian_year);

// Fulltext search
CREATE FULLTEXT INDEX event_search IF NOT EXISTS FOR (e:Event) ON EACH [e.title, e.description];
CREATE FULLTEXT INDEX actor_search IF NOT EXISTS FOR (a:Actor) ON EACH [a.name];
