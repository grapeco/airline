mod backend;

use chrono::NaiveDateTime;
use sqlx::PgPool;
use std::{env, io::{self, Write}};
use backend::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok(); // Читает файл .env
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL не задан в .env"); // Достает строку
    let pool = PgPool::connect(&db_url).await?; // Подключается к БД

    loop {
        println!("\n=== Авиакомпания ===");
        println!(" 1. Аэропорты");
        println!(" 2. Авиакомпании");
        println!(" 3. Самолёты");
        println!(" 4. Рейсы");
        println!(" 5. Места");
        println!(" 6. Пассажиры");
        println!(" 7. Билеты");
        println!(" 8. Платежи");
        println!(" 9. Экипаж");
        println!("10. Назначения экипажа");
        println!("11. 🎫 Купить билет");
        println!(" 0. Выход");

        match ask("> ").as_str() {
            "1"  => airports_menu(&pool).await,
            "2"  => airlines_menu(&pool).await,
            "3"  => aircraft_menu(&pool).await,
            "4"  => flights_menu(&pool).await,
            "5"  => seats_menu(&pool).await,
            "6"  => passengers_menu(&pool).await,
            "7"  => tickets_menu(&pool).await,
            "8"  => payments_menu(&pool).await,
            "9"  => crew_menu(&pool).await,
            "10" => assignments_menu(&pool).await,
            "11" => book_menu(&pool).await,
            "0"  => return Ok(()),
            _    => println!("Нет пункта"),
        }
    }
}

// ═══════════ 1. Аэропорты ═══════════
async fn airports_menu(pool: &PgPool) {
    loop {
        println!("\n--- Аэропорты ---\n1. Список  2. Добавить  0. Назад");
        match ask("> ").as_str() {
            "1" => match get_airports(pool).await {
                Ok(v) => for r in &v {
                    println!("#{} {} — {} ({}, {})", r.id, r.iata_code, r.city, r.name, r.country)
                },
                Err(e) => println!("Ошибка: {e}"),
            },
            "2" => {
                let iata = ask("IATA (3): ");
                let name = ask("Название: ");
                let city = ask("Город: ");
                let country = ask("Страна: ");
                let tz = ask("Timezone: ");
                match create_airport(pool, &iata, &name, &city, &country, &tz).await {
                    Ok(id) => println!("✅ Аэропорт #{id}"),
                    Err(e) => println!("Ошибка: {e}"),
                }
            }
            "0" => return,
            _ => println!("Нет пункта"),
        }
    }
}

// ═══════════ 2. Авиакомпании ═══════════
async fn airlines_menu(pool: &PgPool) {
    loop {
        println!("\n--- Авиакомпании ---\n1. Список  2. Добавить  0. Назад");
        match ask("> ").as_str() {
            "1" => match get_airlines(pool).await {
                Ok(v) => for r in &v { println!("#{} {} ({}) — {}", r.id, r.name, r.iata_code, r.country) },
                Err(e) => println!("Ошибка: {e}"),
            },
            "2" => {
                let name = ask("Название: ");
                let iata = ask("IATA (2): ");
                let country = ask("Страна: ");
                match create_airline(pool, &name, &iata, &country).await {
                    Ok(id) => println!("✅ Авиакомпания #{id}"),
                    Err(e) => println!("Ошибка: {e}"),
                }
            }
            "0" => return,
            _ => println!("Нет пункта"),
        }
    }
}

// ═══════════ 3. Самолёты ═══════════
async fn aircraft_menu(pool: &PgPool) {
    loop {
        println!("\n--- Самолёты ---\n1. Список  2. Добавить  0. Назад");
        match ask("> ").as_str() {
            "1" => match get_aircraft(pool).await {
                Ok(v) => for r in &v {
                    println!("#{} {} [{}] airline={} мест={} {}", r.id, r.model, r.tail_number, r.airline, r.seats_count, r.status)
                },
                Err(e) => println!("Ошибка: {e}"),
            },
            "2" => {
                println!("(ID авиакомпаний — в меню «Авиакомпании» п.1)");
                let airline = ask_i32("ID авиакомпании: ");
                let model = ask("Модель: ");
                let tail = ask("Бортовой номер: ");
                let seats = ask_i32("Кол-во мест: ");
                let status = ask("Статус (active/maintenance/retired): ");
                match create_aircraft(pool, airline, &model, &tail, seats, &status).await {
                    Ok(id) => println!("✅ Самолёт #{id}"),
                    Err(e) => println!("Ошибка: {e}"),
                }
            }
            "0" => return,
            _ => println!("Нет пункта"),
        }
    }
}

// ═══════════ 4. Рейсы ═══════════
async fn flights_menu(pool: &PgPool) {
    loop {
        println!("\n--- Рейсы ---\n1. Список  2. Добавить  0. Назад");
        match ask("> ").as_str() {
            "1" => match get_flights(pool).await {
                Ok(v) => for r in &v {
                    println!("#{} {} {}→{} {} [{}/{}] {}", r.id, r.number, r.from_city, r.to_city,
                             r.departure, r.seats_taken, r.seats_total, r.status)
                },
                Err(e) => println!("Ошибка: {e}"),
            },
            "2" => {
                println!("(ID авиакомпаний, самолётов и аэропортов — в соответствующих меню)");
                let number = ask("Номер рейса: ");
                let airline = ask_i32("ID авиакомпании: ");
                let aircraft = ask_i32("ID самолёта: ");
                let from = ask_i32("ID аэропорта вылета: ");
                let to = ask_i32("ID аэропорта прилёта: ");
                let dep = ask_dt("Вылет (YYYY-MM-DD HH:MM): ");
                let arr = ask_dt("Прилёт (YYYY-MM-DD HH:MM): ");
                let status = ask("Статус (scheduled/departed/arrived/cancelled): ");
                match create_flight(pool, &number, airline, aircraft, from, to, dep, arr, &status).await {
                    Ok(id) => println!("✅ Рейс #{id}"),
                    Err(e) => println!("Ошибка: {e}"),
                }
            }
            "0" => return,
            _ => println!("Нет пункта"),
        }
    }
}

// ═══════════ 5. Места ═══════════
async fn seats_menu(pool: &PgPool) {
    loop {
        println!("\n--- Места ---\n1. По самолёту  2. Добавить  0. Назад");
        match ask("> ").as_str() {
            "1" => {
                let ac = ask_i32("ID самолёта: ");
                match get_seats_by_aircraft(pool, ac).await {
                    Ok(v) if v.is_empty() => println!("(нет мест)"),
                    Ok(v) => for r in &v { println!("#{} {} ({})", r.id, r.number, r.seat_type) },
                    Err(e) => println!("Ошибка: {e}"),
                }
            }
            "2" => {
                let ac = ask_i32("ID самолёта: ");
                let number = ask("Номер места: ");
                let t = ask("Тип (middle/window/aisle): ");
                match create_seat(pool, ac, &number, &t).await {
                    Ok(id) => println!("✅ Место #{id}"),
                    Err(e) => println!("Ошибка: {e}"),
                }
            }
            "0" => return,
            _ => println!("Нет пункта"),
        }
    }
}

// ═══════════ 6. Пассажиры ═══════════
async fn passengers_menu(pool: &PgPool) {
    loop {
        println!("\n--- Пассажиры ---\n1. Список  0. Назад (создаются при покупке билета)");
        match ask("> ").as_str() {
            "1" => match get_passengers(pool).await {
                Ok(v) => for r in &v {
                    println!("#{} {} {} — {}", r.id, r.first_name, r.last_name, r.passport)
                },
                Err(e) => println!("Ошибка: {e}"),
            },
            "0" => return,
            _ => println!("Нет пункта"),
        }
    }
}

// ═══════════ 7. Билеты ═══════════
async fn tickets_menu(pool: &PgPool) {
    loop {
        println!("\n--- Билеты ---\n1. По рейсу  0. Назад (создаются при покупке)");
        match ask("> ").as_str() {
            "1" => {
                let f = ask_i32("ID рейса: ");
                match get_tickets_by_flight(pool, f).await {
                    Ok(v) if v.is_empty() => println!("(нет билетов)"),
                    Ok(v) => for r in &v {
                        println!("#{} {} место {} — {} ₽ [{}]", r.id, r.passenger_name, r.seat_number, r.price, r.status)
                    },
                    Err(e) => println!("Ошибка: {e}"),
                }
            }
            "0" => return,
            _ => println!("Нет пункта"),
        }
    }
}

// ═══════════ 8. Платежи ═══════════
async fn payments_menu(pool: &PgPool) {
    loop {
        println!("\n--- Платежи ---\n1. По билету  0. Назад");
        match ask("> ").as_str() {
            "1" => {
                let t = ask_i32("ID билета: ");
                match get_payments_by_ticket(pool, t).await {
                    Ok(v) if v.is_empty() => println!("(нет платежей)"),
                    Ok(v) => for r in &v {
                        println!("#{} {} ₽ {} {} [{}]", r.id, r.amount, r.method, r.date, r.status)
                    },
                    Err(e) => println!("Ошибка: {e}"),
                }
            }
            "0" => return,
            _ => println!("Нет пункта"),
        }
    }
}

// ═══════════ 9. Экипаж ═══════════
async fn crew_menu(pool: &PgPool) {
    loop {
        println!("\n--- Экипаж ---\n1. Список  2. Добавить  0. Назад");
        match ask("> ").as_str() {
            "1" => match get_crew(pool).await {
                Ok(v) => for r in &v {
                    println!("#{} {} {} — {}", r.id, r.first_name, r.last_name, r.role)
                },
                Err(e) => println!("Ошибка: {e}"),
            },
            "2" => {
                let f = ask("Имя: ");
                let l = ask("Фамилия: ");
                let role = ask("Роль (pilot/co-pilot/flight_attendant): ");
                match create_crew(pool, &f, &l, &role).await {
                    Ok(id) => println!("✅ Член экипажа #{id}"),
                    Err(e) => println!("Ошибка: {e}"),
                }
            }
            "0" => return,
            _ => println!("Нет пункта"),
        }
    }
}

// ═══════════ 10. Назначения экипажа ═══════════
async fn assignments_menu(pool: &PgPool) {
    loop {
        println!("\n--- Назначения экипажа ---\n1. По рейсу  2. Назначить  0. Назад");
        match ask("> ").as_str() {
            "1" => {
                let f = ask_i32("ID рейса: ");
                match get_crew_for_flight(pool, f).await {
                    Ok(v) if v.is_empty() => println!("(никто не назначен)"),
                    Ok(v) => for r in &v {
                        println!("#{} {} {} — {}", r.id, r.first_name, r.last_name, r.role)
                    },
                    Err(e) => println!("Ошибка: {e}"),
                }
            }
            "2" => {
                let f = ask_i32("ID рейса: ");
                let c = ask_i32("ID члена экипажа: ");
                match create_crew_assignment(pool, f, c).await {
                    Ok(id) => println!("✅ Назначение #{id}"),
                    Err(e) => println!("Ошибка: {e}"),
                }
            }
            "0" => return,
            _ => println!("Нет пункта"),
        }
    }
}

// ═══════════ 11. Купить билет ═══════════
async fn book_menu(pool: &PgPool) {
    println!("\n--- Рейсы ---");
    let flights = match get_flights(pool).await {
        Ok(v) if v.is_empty() => { println!("Рейсов нет"); return; }
        Ok(v) => v,
        Err(e) => { println!("Ошибка: {e}"); return; }
    };
    for r in &flights {
        println!("#{} {} {}→{} {} [{}/{}]",
                 r.id, r.number, r.from_city, r.to_city, r.departure,
                 r.seats_taken, r.seats_total);
    }

    let flight_id = ask_i32("\nID рейса: ");
    let aircraft_id = match sqlx::query_scalar::<_, i32>(
        "SELECT aircraft FROM flights WHERE id = $1").bind(flight_id)
        .fetch_one(pool).await {
        Ok(v) => v,
        Err(e) => { println!("Рейс не найден: {e}"); return; }
    };

    println!("\n--- Свободные места ---");
    match get_free_seats(pool, flight_id, aircraft_id).await {
        Ok(v) if v.is_empty() => { println!("Свободных мест нет"); return; }
        Ok(v) => for r in &v { println!("#{} {} ({})", r.id, r.number, r.seat_type) },
        Err(e) => { println!("Ошибка: {e}"); return; }
    }

    let seat_id = ask_i32("\nID места: ");
    let price = ask_f64("Цена: ");

    println!("\n--- Пассажир ---");
    let p = NewPassenger {
        first_name: ask("Имя: "),
        last_name: ask("Фамилия: "),
        passport: ask("Паспорт: "),
        phone: ask("Телефон: "),
        email: ask("Email: "),
        birthday: ask("ДР (YYYY-MM-DD): "),
    };

    match book_ticket(pool, flight_id, seat_id, price, p).await {
        Ok(id) => println!("\n✅ Билет #{id} куплен"),
        Err(e) => println!("\nОшибка: {e}"),
    }
}

// ═══════════ Утилиты ═══════════
fn ask(prompt: &str) -> String {
    print!("{prompt}");
    io::stdout().flush().ok();
    let mut s = String::new();
    io::stdin().read_line(&mut s).ok();
    s.trim().to_string()
}

fn ask_i32(prompt: &str) -> i32 {
    ask(prompt).parse().unwrap_or(0)
}

fn ask_f64(prompt: &str) -> f64 {
    ask(prompt).parse().unwrap_or(0.0)
}

fn ask_dt(prompt: &str) -> NaiveDateTime {
    NaiveDateTime::parse_from_str(&ask(prompt), "%Y-%m-%d %H:%M")
        .expect("формат: YYYY-MM-DD HH:MM")
}