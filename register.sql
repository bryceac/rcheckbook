BEGIN TRANSACTION;
DROP TABLE IF EXISTS "categories";
CREATE TABLE "categories" (
	"id"	INTEGER,
	"category"	TEXT NOT NULL,
	PRIMARY KEY("id" AUTOINCREMENT)
) STRICT;
DROP TABLE IF EXISTS "trades";
CREATE TABLE "trades" (
	"id"	TEXT,
	"date"	TEXT NOT NULL DEFAULT (DATE('now')),
	"check_number"	INTEGER DEFAULT NULL,
	"vendor"	TEXT DEFAULT '',
	"memo"	TEXT DEFAULT '',
	"amount"	REAL NOT NULL DEFAULT 0.0,
	"category"	INTEGER DEFAULT NULL,
	"reconciled"	INTEGER NOT NULL DEFAULT 0 CHECK("reconciled" IN (0, 1)),
	PRIMARY KEY("id")
) STRICT;
INSERT INTO "categories" VALUES (1,'Utilities'),
 (2,'Gifts'),
 (3,'Groceries'),
 (4,'Dining'),
 (5,'Recreation'),
 (6,'Subscriptions'),
 (7,'Opening Balance');
DROP VIEW IF EXISTS "ledger";
CREATE VIEW ledger AS
SELECT t.id AS 'id',
DATE(t.date) AS 'date', 
t.check_number, 
CASE t.reconciled
	WHEN 1
		THEN 'Y'
	ELSE 'N'
END AS 'reconciled',
t.vendor, 
t.memo,
c.category,
t.amount,
SUM(t.amount) OVER (
	ORDER BY DATE(t.date) ASC
	ROWS BETWEEN
	UNBOUNDED PRECEDING
	AND CURRENT ROW) AS 'balance'
FROM trades t
LEFT JOIN categories c 
ON t.category = c.id 
ORDER BY DATE(t.date) ASC;
COMMIT;
