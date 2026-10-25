DROP TABLE IF EXISTS payments, tickets, seats, flights,
                     crew_assignments, crew, aircraft,
                     airlines, airports, passengers
CASCADE;


CREATE TABLE passengers (
	id SERIAL PRIMARY KEY,
	first_name VARCHAR(50),
	last_name VARCHAR(50),
	passport VARCHAR(20) UNIQUE,
	phone VARCHAR(20),
	email VARCHAR(100) UNIQUE,
	birthday DATE
);

CREATE TABLE crew (
	id SERIAL PRIMARY KEY,
	first_name VARCHAR(50),
	last_name VARCHAR(50),
	role VARCHAR(30) CHECK (role IN ('pilot', 'co-pilot', 'flight_attendant'))
);

CREATE TABLE airports (
	id SERIAL PRIMARY KEY,
	iata_code CHAR(3) UNIQUE, 
	name VARCHAR(100),
	city VARCHAR(50),
	country VARCHAR(50),
	timezone VARCHAR(50)
);

CREATE TABLE airlines (
	id SERIAL PRIMARY KEY,
	name VARCHAR(100),
	iata_code CHAR(2) UNIQUE,
	country VARCHAR(50)
);

CREATE TABLE aircraft (
	id SERIAL PRIMARY KEY,
	airline INT REFERENCES airlines(id),
	model VARCHAR(50),
	tail_number VARCHAR(20) UNIQUE,
	seats_count INT,
	status VARCHAR(50) CHECK (status IN ('active', 'maintenance', 'retired'))
);

CREATE TABLE flights (
	id SERIAL PRIMARY KEY,
	number VARCHAR(10),
	airline INT REFERENCES airlines(id),
	aircraft INT REFERENCES aircraft(id),
	from_airport INT REFERENCES airports(id),
	to_airport INT REFERENCES airports(id), 
	departure TIMESTAMP,
	arrival TIMESTAMP,
	status VARCHAR(50) CHECK (status IN ('scheduled', 'arrived' ,'departed')),
	UNIQUE (number, departure)
);

CREATE TABLE crew_assignments (
    id             SERIAL PRIMARY KEY,
    flight         INT REFERENCES flights(id),
    crew_member    INT REFERENCES crew(id),
    UNIQUE (flight, crew_member)
);

CREATE TABLE seats (
	id SERIAL PRIMARY KEY,
	aircraft INT REFERENCES aircraft(id),
	number VARCHAR(5),
	type VARCHAR(50) CHECK (type IN ('middle', 'window', 'aisle')),
	UNIQUE (aircraft, number)
);

CREATE TABLE tickets (
	id SERIAL PRIMARY KEY,
	passenger INT REFERENCES passengers(id),
	flight INT REFERENCES flights(id),
	seat INT REFERENCES seats(id),
	price FLOAT8,
	status VARCHAR(20) CHECK (status IN ('booked', 'paid', 'issued', 'used')),
	UNIQUE (flight, seat)
);

CREATE TABLE payments (
	id SERIAL PRIMARY KEY,
	ticket INT REFERENCES tickets(id),
	amount FLOAT8,
	date TIMESTAMP DEFAULT NOW(),
    method VARCHAR(20) CHECK (method IN ('card', 'cash')),
    status VARCHAR(20) CHECK (status IN ('completed', 'failed'))
);