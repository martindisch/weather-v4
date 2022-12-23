-- Migration number: 0000 	 2022-12-23T09:27:53.745Z
DROP TABLE IF EXISTS measurements;
CREATE TABLE measurements (timestamp INTEGER PRIMARY KEY ASC, temperature Real, humidity Real);
