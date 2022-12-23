-- Migration number: 0000 	 2022-12-23T09:27:53.745Z
DROP TABLE IF EXISTS Measurements;
CREATE TABLE Measurements (Timestamp INTEGER PRIMARY KEY ASC, Temperature Real, Humidity Real);
INSERT INTO Measurements VALUES (1662352916,15.2,70), (1662353229,16.4,75), (1662353540,17,78.2);
