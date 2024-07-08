-- Add migration script here

CREATE TABLE "docs" (
	"id"	INTEGER,
	"name"	TEXT NOT NULL,
	"storagename"	TEXT NOT NULL,
	"date"	TEXT NOT NULL,
	"lastupdate"	TEXT NOT NULL,
	PRIMARY KEY("id" AUTOINCREMENT)
);

CREATE TABLE "tags" (
	"id"	INTEGER,
	"name"	TEXT NOT NULL,
	"color"	TEXT NOT NULL,
	PRIMARY KEY("id" AUTOINCREMENT)
);
CREATE TABLE "types" (
	"id"	INTEGER,
	"name"	TEXT NOT NULL,
	"color"	TEXT NOT NULL,
	PRIMARY KEY("id" AUTOINCREMENT)
);
CREATE TABLE "people" (
	"id"	INTEGER,
	"name"	TEXT NOT NULL,
	"color"	TEXT NOT NULL,
	PRIMARY KEY("id" AUTOINCREMENT)
);

CREATE TABLE "doctags" (
	"iddoc"	INTEGER,
	"idtag"	INTEGER,
	PRIMARY KEY("iddoc","idtag"),
	FOREIGN KEY("idtag") REFERENCES "tags"("id"),
	FOREIGN KEY("iddoc") REFERENCES "docs"("id")
);
CREATE TABLE "doctypes" (
	"iddoc"	INTEGER,
	"idtype"	INTEGER,
	PRIMARY KEY("iddoc","idtype"),
	FOREIGN KEY("idtype") REFERENCES "types"("id"),
	FOREIGN KEY("iddoc") REFERENCES "docs"("id")
);
CREATE TABLE "docpeople" (
	"iddoc"	INTEGER,
	"idperson"	INTEGER,
	PRIMARY KEY("iddoc","idperson"),
	FOREIGN KEY("idperson") REFERENCES "people"("id"),
	FOREIGN KEY("iddoc") REFERENCES "docs"("id")
);

CREATE TABLE "previews" (
	"iddoc"	INTEGER,
	"data"	BLOB,
	"lastupdate"	TEXT NOT NULL,
	PRIMARY KEY("iddoc"),
	FOREIGN KEY("iddoc") REFERENCES "docs"("id")
);
CREATE TABLE "ocrcontent" (
	"iddoc"	INTEGER,
	"data"	TEXT,
	"lastupdate"	TEXT NOT NULL,
	PRIMARY KEY("iddoc"),
	FOREIGN KEY("iddoc") REFERENCES "docs"("id")
);