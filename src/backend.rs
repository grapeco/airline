use sqlx::{PgPool, FromRow};
use chrono::NaiveDateTime;

// ========== Row-структуры ==========

#[derive(FromRow, Debug, Clone)]
pub struct Airport {
    pub id: i32, pub iata_code: String, pub name: String,
    pub city: String, pub country: String, pub timezone: String,
}

#[derive(FromRow, Debug, Clone)]
pub struct Airline {
    pub id: i32, pub name: String, pub iata_code: String, pub country: String,
}

#[derive(FromRow, Debug, Clone)]
pub struct Aircraft {
    pub id: i32, pub airline: i32, pub model: String,
    pub tail_number: String, pub seats_count: i32, pub status: String,
}

#[derive(FromRow, Debug, Clone)]
pub struct Flight {
    pub id: i32, pub number: String,
    pub from_city: String, pub to_city: String,
    pub departure: NaiveDateTime, pub arrival: NaiveDateTime,
    pub status: String, pub aircraft: i32,
    pub seats_total: i64, pub seats_taken: i64,
}

#[derive(FromRow, Debug, Clone)]
pub struct Seat { pub id: i32, pub number: String, pub seat_type: String }

#[derive(FromRow, Debug, Clone)]
pub struct Passenger {
    pub id: i32, pub first_name: String, pub last_name: String, pub passport: String,
}

#[derive(FromRow, Debug, Clone)]
pub struct Ticket {
    pub id: i32, pub passenger_id: i32, pub passenger_name: String,
    pub seat_number: String, pub price: f64, pub status: String,
}

#[derive(FromRow, Debug, Clone)]
pub struct Payment {
    pub id: i32, pub ticket: i32, pub amount: f64,
    pub date: NaiveDateTime, pub method: String, pub status: String,
}

#[derive(FromRow, Debug, Clone)]
pub struct Crew {
    pub id: i32, pub first_name: String, pub last_name: String, pub role: String,
}

pub struct NewPassenger {
    pub first_name: String, pub last_name: String, pub passport: String,
    pub phone: String, pub email: String, pub birthday: String,
}

// ========== Airports ==========

pub async fn get_airports(pool: &PgPool) -> Result<Vec<Airport>, sqlx::Error> {
    sqlx::query_as("SELECT id, iata_code, name, city, country, timezone
                    FROM airports ORDER BY city")
        .fetch_all(pool).await
}

pub async fn create_airport(
    pool: &PgPool, iata: &str, name: &str, city: &str, country: &str, tz: &str
) -> Result<i32, sqlx::Error> {
    sqlx::query_scalar(
        "INSERT INTO airports (iata_code, name, city, country, timezone)
         VALUES ($1,$2,$3,$4,$5) RETURNING id")
        .bind(iata).bind(name).bind(city).bind(country).bind(tz)
        .fetch_one(pool).await
}

// ========== Airlines ==========

pub async fn get_airlines(pool: &PgPool) -> Result<Vec<Airline>, sqlx::Error> {
    sqlx::query_as("SELECT id, name, iata_code, country FROM airlines ORDER BY name")
        .fetch_all(pool).await
}

pub async fn create_airline(
    pool: &PgPool, name: &str, iata: &str, country: &str
) -> Result<i32, sqlx::Error> {
    sqlx::query_scalar(
        "INSERT INTO airlines (name, iata_code, country)
         VALUES ($1,$2,$3) RETURNING id")
        .bind(name).bind(iata).bind(country)
        .fetch_one(pool).await
}

// ========== Aircraft ==========

pub async fn get_aircraft(pool: &PgPool) -> Result<Vec<Aircraft>, sqlx::Error> {
    sqlx::query_as(
        "SELECT id, airline, model, tail_number, seats_count, status
         FROM aircraft ORDER BY id")
        .fetch_all(pool).await
}

pub async fn create_aircraft(
    pool: &PgPool, airline: i32, model: &str, tail: &str, seats: i32, status: &str
) -> Result<i32, sqlx::Error> {
    sqlx::query_scalar(
        "INSERT INTO aircraft (airline, model, tail_number, seats_count, status)
         VALUES ($1,$2,$3,$4,$5) RETURNING id")
        .bind(airline).bind(model).bind(tail).bind(seats).bind(status)
        .fetch_one(pool).await
}

// ========== Flights ==========

pub async fn get_flights(pool: &PgPool) -> Result<Vec<Flight>, sqlx::Error> {
    sqlx::query_as(r#"
        SELECT f.id, f.number,
               ap1.city AS from_city, ap2.city AS to_city,
               f.departure, f.arrival, f.status, f.aircraft,
               COUNT(DISTINCT s.id) AS seats_total,
               COUNT(DISTINCT t.id) AS seats_taken
        FROM flights f
        JOIN airports ap1 ON f.from_airport = ap1.id
        JOIN airports ap2 ON f.to_airport   = ap2.id
        LEFT JOIN seats s   ON s.aircraft = f.aircraft
        LEFT JOIN tickets t ON t.flight = f.id AND t.seat = s.id
        GROUP BY f.id, ap1.city, ap2.city
        ORDER BY f.departure"#)
        .fetch_all(pool).await
}

pub async fn create_flight(
    pool: &PgPool, number: &str, airline: i32, aircraft: i32,
    from: i32, to: i32, dep: NaiveDateTime, arr: NaiveDateTime,
    status: &str
) -> Result<i32, sqlx::Error> {
    sqlx::query_scalar(
        "INSERT INTO flights (number, airline, aircraft, from_airport, to_airport,
                              departure, arrival, status)
         VALUES ($1,$2,$3,$4,$5,$6,$7,$8) RETURNING id")
        .bind(number).bind(airline).bind(aircraft).bind(from).bind(to)
        .bind(dep).bind(arr).bind(status)
        .fetch_one(pool).await
}

// ========== Seats ==========

pub async fn get_seats_by_aircraft(
    pool: &PgPool, aircraft_id: i32
) -> Result<Vec<Seat>, sqlx::Error> {
    sqlx::query_as("SELECT id, number, type AS seat_type
                    FROM seats WHERE aircraft = $1 ORDER BY id")
        .bind(aircraft_id).fetch_all(pool).await
}

pub async fn create_seat(
    pool: &PgPool, aircraft: i32, number: &str, seat_type: &str
) -> Result<i32, sqlx::Error> {
    sqlx::query_scalar(
        "INSERT INTO seats (aircraft, number, type) VALUES ($1,$2,$3) RETURNING id")
        .bind(aircraft).bind(number).bind(seat_type)
        .fetch_one(pool).await
}

pub async fn get_free_seats(
    pool: &PgPool, flight_id: i32, aircraft_id: i32
) -> Result<Vec<Seat>, sqlx::Error> {
    sqlx::query_as(r#"
        SELECT s.id, s.number, s.type AS seat_type
        FROM seats s
        WHERE s.aircraft = $1
          AND s.id NOT IN (SELECT seat FROM tickets WHERE flight = $2)
        ORDER BY s.id"#)
        .bind(aircraft_id).bind(flight_id)
        .fetch_all(pool).await
}

// ========== Passengers ==========

pub async fn get_passengers(pool: &PgPool) -> Result<Vec<Passenger>, sqlx::Error> {
    sqlx::query_as("SELECT id, first_name, last_name, passport
                    FROM passengers ORDER BY id")
        .fetch_all(pool).await
}

// ========== Tickets ==========

pub async fn get_tickets_by_flight(
    pool: &PgPool, flight_id: i32
) -> Result<Vec<Ticket>, sqlx::Error> {
    sqlx::query_as(r#"
        SELECT t.id, t.passenger AS passenger_id,
               p.first_name || ' ' || p.last_name AS passenger_name,
               s.number AS seat_number, t.price, t.status
        FROM tickets t
        JOIN passengers p ON t.passenger = p.id
        JOIN seats s      ON t.seat      = s.id
        WHERE t.flight = $1 ORDER BY t.id"#)
        .bind(flight_id).fetch_all(pool).await
}

// ========== Payments ==========

pub async fn get_payments_by_ticket(
    pool: &PgPool, ticket_id: i32
) -> Result<Vec<Payment>, sqlx::Error> {
    sqlx::query_as("SELECT id, ticket, amount, date, method, status
                    FROM payments WHERE ticket = $1 ORDER BY id")
        .bind(ticket_id).fetch_all(pool).await
}

// ========== Crew ==========

pub async fn get_crew(pool: &PgPool) -> Result<Vec<Crew>, sqlx::Error> {
    sqlx::query_as("SELECT id, first_name, last_name, role FROM crew ORDER BY id")
        .fetch_all(pool).await
}

pub async fn create_crew(
    pool: &PgPool, first: &str, last: &str, role: &str
) -> Result<i32, sqlx::Error> {
    sqlx::query_scalar(
        "INSERT INTO crew (first_name, last_name, role) VALUES ($1,$2,$3) RETURNING id")
        .bind(first).bind(last).bind(role)
        .fetch_one(pool).await
}

// ========== Crew assignments ==========

pub async fn get_crew_for_flight(
    pool: &PgPool, flight_id: i32
) -> Result<Vec<Crew>, sqlx::Error> {
    sqlx::query_as(r#"
        SELECT c.id, c.first_name, c.last_name, c.role
        FROM crew c
        JOIN crew_assignments ca ON c.id = ca.crew_member
        WHERE ca.flight = $1"#)
        .bind(flight_id).fetch_all(pool).await
}

pub async fn create_crew_assignment(
    pool: &PgPool, flight: i32, crew: i32
) -> Result<i32, sqlx::Error> {
    sqlx::query_scalar(
        "INSERT INTO crew_assignments (flight, crew_member) VALUES ($1,$2) RETURNING id")
        .bind(flight).bind(crew)
        .fetch_one(pool).await
}

// ========== Транзакция покупки ==========

pub async fn book_ticket(
    pool: &PgPool, flight_id: i32, seat_id: i32,
    price: f64, p: NewPassenger
) -> Result<i32, sqlx::Error> {
    let mut tx = pool.begin().await?;

    let passenger_id: i32 = sqlx::query_scalar(
        "INSERT INTO passengers (first_name, last_name, passport, phone, email, birthday)
         VALUES ($1,$2,$3,$4,$5,$6::date) RETURNING id")
        .bind(&p.first_name).bind(&p.last_name).bind(&p.passport)
        .bind(&p.phone).bind(&p.email).bind(&p.birthday)
        .fetch_one(&mut *tx).await?;

    let ticket_id: i32 = sqlx::query_scalar(
        "INSERT INTO tickets (passenger, flight, seat, price, status)
         VALUES ($1,$2,$3,$4,'paid') RETURNING id")
        .bind(passenger_id).bind(flight_id).bind(seat_id).bind(price)
        .fetch_one(&mut *tx).await?;

    sqlx::query(
        "INSERT INTO payments (ticket, amount, method, status)
         VALUES ($1,$2,'card','completed')")
        .bind(ticket_id).bind(price)
        .execute(&mut *tx).await?;

    tx.commit().await?;
    Ok(ticket_id)
}