use crate::{db::open_transaction, model::*};
use chrono::{NaiveDate, NaiveTime};
use log::{log, Level};
use sqlx::{query, query_as, Pool, Postgres};
use std::{
    iter::zip,
    time::{SystemTime, UNIX_EPOCH},
};

pub async fn parse(db: &Pool<Postgres>) -> Result<(), ()> {
    log!(Level::Info, "started parser");

    let time_started: u128 = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis();
    let mut terms_processed: i32 = 0;
    let mut courses_added: i32 = 0;
    let mut courses_updated: i32 = 0;
    let mut sections_added: i32 = 0;
    let mut sections_updated: i32 = 0;
    let mut failures: i32 = 0;

    // Update academicterms table
    log!(Level::Info, "updating academicterms table");

    let mut transaction = match open_transaction(db).await {
        Ok(t) => t,
        Err(e) => {
            log!(
                Level::Error,
                "failed to open transaction\nreturned error: {}",
                e.error().unwrap()
            );
            return Err(());
        }
    };

    let terms: Vec<SemTime> = match query_as!(
        TempSemTime,
        "SELECT academic_term, mode() WITHIN GROUP (ORDER BY start_date) as start_date, mode() WITHIN GROUP (ORDER BY end_date) as end_date FROM meetings GROUP BY academic_term",
    )
    .fetch_all(&mut *transaction)
    .await
    {
        Ok(t) => t
            .iter()
            .map(|x| SemTime {
                academic_term: x.academic_term,
                start_date: NaiveDate::parse_from_str(&x.start_date.clone().unwrap(), "%Y%m%d").unwrap(),
                end_date: NaiveDate::parse_from_str(&x.end_date.clone().unwrap(), "%Y%m%d").unwrap(),
            })
            .collect(),
        Err(e) => {
            log!(Level::Error, "failed to query meetings for academic terms\nreturned error: {}", e);
            return Err(());
        }
    };

    let mut updated_terms = vec![];
    for term in terms.iter() {
        terms_processed += 1;
        updated_terms.push(term.academic_term);

        match query!(
            "INSERT INTO academicterms (term, start_date, end_date) VALUES ($1, $2, $3) ON CONFLICT (term) DO UPDATE SET start_date = $2, end_date = $3",
            term.academic_term as i16, term.start_date, term.end_date,
        )
        .execute(&mut *transaction)
        .await {
            Ok(_) => {}
            Err(e) => {
                log!(Level::Error, "failed to insert into academicterms\nreturned error: {}", e);
                return Err(());
            }
        }
    }

    // Cancel all sections
    match query!(
        "UPDATE sections SET status = 'X' WHERE course IN (SELECT term FROM courses WHERE term IN (SELECT * FROM UNNEST(CAST($1 as int4[]))))",
        &updated_terms,
    )
    .execute(&mut *transaction)
    .await {
        Ok(_) => {},
        Err(e) => {
            log!(Level::Error, "failed to cancel all sections\nreturned error: {}", e);
            return Err(());
        }
    }

    // Update schools
    match query!(
        "INSERT INTO schools (code) (SELECT academic_group FROM classes WHERE academic_group NOT IN (SELECT code FROM schools)) ON CONFLICT (code) DO NOTHING"
    )
    .execute(&mut *transaction)
    .await {
        Ok(_) => log!(Level::Info, "successfully updated schools table"),
        Err(e) => {
            failures += 1;
            log!(Level::Warn, "failed to update schools table\nreturned error: {}", e);
        },
    }

    // Select departments to add/update
    match query!(
        "INSERT INTO departments (code, school) (SELECT DISTINCT c.academic_org, s.id FROM classes c, schools s WHERE s.code = c.academic_group) ON CONFLICT (code) DO NOTHING"
    )
    .execute(&mut *transaction)
    .await {
        Ok(_) => {},
        Err(e) => {
            failures += 1;
            log!(Level::Warn, "failed to find departments to update\nreturned error: {}", e);
        },
    }

    // Select each course from classes
    let mut courses: Vec<Course> = vec![];
    match query_as!(
        Course,
        "SELECT academic_term, subject, units, academic_org, catalog_number, description, course_description_long, course_id, course_offer_number, session_code FROM classes",
    ).fetch_all(&mut *transaction).await {
        Ok(res) => courses = res,
        Err(e) => {
            failures += 1;
            log!(Level::Warn, "failed to select courses\nreturned error: {}", e);
        },
    }

    courses.sort();
    courses.dedup();

    for course in courses.iter() {
        println!("{:?}", course.clone());

        let c_inserted: bool;
        let course_id: i32;
        match query_as!(
            CourseIOUResult,
            "SELECT * FROM InsertOrUpdateCourse($1, $2, $3, $4, $5, $6)",
            course.academic_term,
            course.academic_org,
            course.catalog_number,
            course
                .units
                .chars()
                .take_while(|&c| c != '.')
                .collect::<String>()
                .parse::<i32>()
                .unwrap(),
            course.description,
            course.course_description_long
        )
        .fetch_one(&mut *transaction)
        .await
        {
            Ok(res) if res.insertorupdatecourse.is_some() => {
                c_inserted = res.insertorupdatecourse.clone().unwrap()[0] == 0;
                course_id = res.insertorupdatecourse.clone().unwrap()[1];

                log!(
                    Level::Trace,
                    "successfully {} course id {}",
                    if c_inserted { "inserted" } else { "updated" },
                    course_id
                );
            }
            Ok(_) => {
                failures += 1;
                log!(
                    Level::Warn,
                    "failed to insert/update course {} {}-{}\nreturned Ok(None)",
                    course.academic_term,
                    course.subject,
                    course.catalog_number
                );
                continue;
            }
            Err(e) => {
                failures += 1;
                log!(
                    Level::Warn,
                    "failed to insert/update course {} {}-{}\nreturned error: {}",
                    course.academic_term,
                    course.subject,
                    course.catalog_number,
                    e
                );
                continue;
            }
        }

        if c_inserted {
            courses_added += 1;
        } else {
            courses_updated += 1;
        }

        let sections: Vec<Section>;
        match query_as!(
            TempSection,
            "SELECT class_section, description, topic, enrollment_status, class_status, class_type, enrollment_capacity, enrollment_total, instruction_mode, schedule_print FROM classes WHERE course_id = $1 AND course_offer_number = $2 AND academic_term = $3 AND session_code = $4",
            course.course_id,
            course.course_offer_number,
            course.academic_term,
            course.session_code,
        ).fetch_all(&mut *transaction).await {
            Ok(res) => {
                sections = res.iter().map(|x| Section {
                    class_section: x.class_section.clone(),
                    description: x.description.clone(),
                    topic: x.topic.clone(),
                    enrollment_status: EnrollmentStatus::from_str(&x.enrollment_status),
                    class_status: ClassStatus::from_str(&x.class_status),
                    class_type: x.class_type.chars().next().unwrap(),
                    enrollment_capacity: x.enrollment_capacity,
                    enrollment_total: x.enrollment_total,
                    instruction_mode: x.instruction_mode.clone(),
                    schedule_print: SchedulePrint::from_str(&x.schedule_print),
                }).collect();
                log!(Level::Trace, "successfully retrieved sections for course {}", course.course_id);
            },
            Err(e) => {
                failures += 1;
                log!(Level::Warn, "failed to retrieve sections for course {}\nreturned error: {}", course.course_id, e);
                continue;
            },
        }

        for section in sections.iter() {
            println!("{:?}", section.clone());

            let mut instructor: String = String::default();
            match query_as!(
                Name,
                "SELECT CONCAT(first_name, ' ', last_name) AS name FROM instructors WHERE course_id = $1 AND course_offer_number = $2 AND academic_term = $3 AND session_code = $4 AND class_section = $5 LIMIT 1",
                course.course_id,
                course.course_offer_number,
                course.academic_term,
                course.session_code,
                section.class_section
            ).fetch_optional(&mut *transaction).await {
                Ok(Some(res)) => {
                    instructor = res.name.unwrap();
                    log!(Level::Trace, "successfully found instructor {} for course {}", instructor, course.course_id);
                },
                Ok(None) => {
                    instructor = String::from("TBA");
                    log!(Level::Trace, "successfully found TBA as instructor for course {}", course.course_id);
                },
                Err(e) => {
                    failures += 1;
                    log!(Level::Warn, "failed to find instructor for course {}\nreturned error: {}", course.course_id, e);
                },
            }

            let section_status = if section.class_status == ClassStatus::Cancelled
                || section.schedule_print == SchedulePrint::Hidden
            {
                EnrollmentStatus::Closed
            } else {
                section.enrollment_status.clone()
            };

            let section_type = if section.instruction_mode == "P" {
                "R".to_string()
            } else {
                section.instruction_mode.clone()
            };

            let s_inserted: bool;
            let section_id: i32;
            match query_as!(
                SectionIOUResult,
                "SELECT * FROM InsertOrUpdateSection($1, $2, $3, $4, $5, $6, $7, $8)",
                course_id,
                section.class_section,
                section.description,
                instructor,
                section_type,
                &section_status.to_string(),
                section.enrollment_total,
                section.enrollment_capacity,
            )
            .fetch_one(&mut *transaction)
            .await
            {
                Ok(res) if res.insertorupdatesection.is_some() => {
                    s_inserted = res.insertorupdatesection.clone().unwrap()[0] == 0;
                    section_id = res.insertorupdatesection.clone().unwrap()[1];
                    log!(
                        Level::Trace,
                        "successfully {} section id {}",
                        if s_inserted { "inserted" } else { "updated" },
                        section_id
                    );
                }
                Ok(_) => {
                    failures += 1;
                    log!(
                        Level::Warn,
                        "failed to insert/update section: {}\ngot Ok(None) response",
                        section.class_section
                    );
                    continue;
                }
                Err(e) => {
                    failures += 1;
                    log!(
                        Level::Warn,
                        "failed to insert/update section: {}\nreturned error: {}",
                        section.class_section,
                        e
                    );
                    continue;
                }
            }

            if s_inserted {
                sections_added += 1;
            } else {
                sections_updated += 1;
            }

            match query!("DELETE FROM times WHERE section = $1", section_id)
                .execute(&mut *transaction)
                .await
            {
                Ok(_) => log!(Level::Trace, "successfully deleted section times"),
                Err(e) => {
                    failures += 1;
                    log!(
                        Level::Warn,
                        "unable to delete section times\nreturned error: {}",
                        e
                    );
                    continue;
                }
            }

            let meetings: Vec<SectionMeeting>;
            match query_as!(
                    TempSectionMeeting,
                    "SELECT building, room_number, meeting_time_start, meeting_time_end, monday, tuesday, wednesday, thursday, friday, saturday, sunday FROM meetings WHERE course_id = $1 AND course_offer_number = $2 AND academic_term = $3 AND session_code = $4 AND class_section = $5",
                    course.course_id,
                    course.course_offer_number,
                    course.academic_term,
                    course.session_code,
                    section.class_section,
                )
                .fetch_all(&mut *transaction)
            .await {
                Ok(res) => {
                    meetings = res.iter().map(|x| SectionMeeting {
                        building: x.building.clone(),
                        room_number: x.room_number.clone(),
                        meeting_time_start: x.meeting_time_start.clone(),
                        meeting_time_end: x.meeting_time_end.clone(),
                        monday: WeekdayScheduled::from_str(&x.monday),
                        tuesday: WeekdayScheduled::from_str(&x.tuesday),
                        wednesday: WeekdayScheduled::from_str(&x.wednesday),
                        thursday: WeekdayScheduled::from_str(&x.thursday),
                        friday: WeekdayScheduled::from_str(&x.friday),
                        saturday: WeekdayScheduled::from_str(&x.saturday),
                        sunday: WeekdayScheduled::from_str(&x.sunday),
                    }).collect();
                    log!(Level::Trace, "successfully retrieved meeting times for section {}", section.class_section);
                },
                Err(e) => {
                    failures += 1;
                    log!(Level::Warn, "failed to retrieve meeting times for section {}\nreturned error: {}", section.class_section, e);
                    continue;
                },
            }

            for mut meeting in meetings.into_iter() {
                match meeting.building.as_str() {
                    "UNKNOWN" | "TBA" => {
                        meeting.building = String::from("TBA");
                        meeting.room_number = String::from("TBA");
                    }
                    "OFFC" => {
                        meeting.building = String::from("OFF");
                        meeting.room_number = String::from("SITE");
                    }
                    "ONLINE" => {
                        meeting.building = String::from("ON");
                        meeting.room_number = String::from("LINE");
                    }
                    _ => {}
                }

                if meeting.meeting_time_start.is_empty() && meeting.meeting_time_end.is_empty() {
                    continue;
                }

                let weekdays: Vec<&str> = vec!["0", "1", "2", "3", "4", "5", "6"];
                let weekdays_scheduled: Vec<WeekdayScheduled> = vec![
                    meeting.sunday,
                    meeting.monday,
                    meeting.tuesday,
                    meeting.wednesday,
                    meeting.thursday,
                    meeting.friday,
                    meeting.saturday,
                ];

                let days: Vec<WeekdayInfo> = zip(weekdays, weekdays_scheduled)
                    .map(|(x, y)| WeekdayInfo {
                        weekday: WeekDay::from_str(x),
                        scheduled: y.clone(),
                    })
                    .collect();

                for day in days {
                    if day.scheduled == WeekdayScheduled::Scheduled {
                        match query!(
                            "INSERT INTO times (section, day, start_time, end_time, building, room) VALUES ($1, $2, $3, $4, $5, $6)",
                            section_id,
                            day.weekday.to_string().parse::<i16>().unwrap(),
                            time_from_str(&meeting.meeting_time_start.chars().take_while(|&c| c != ' ').collect::<String>()),
                            time_from_str(&meeting.meeting_time_end.chars().take_while(|&c| c != ' ').collect::<String>()),
                            meeting.building,
                            meeting.room_number,
                        ).execute(&mut *transaction).await {
                            Ok(_) => log!(Level::Trace, "successfully inserted day {}", day.weekday),
                            Err(e) => {
                                failures += 1;
                                log!(Level::Warn, "failed to insert meeting times\nreturned error: {}", e);
                            }
                        }
                    }
                }
            }
        }
    }

    // clean up
    match query!(
        "INSERT INTO scrapelog (time_started, time_ended, terms_added, courses_added, courses_updated, sections_added, sections_updated, failures) VALUES ($1, $2, $3, $4, $5, $6, $7, $8)",
        time_started as i64,
        SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as i64,
        terms_processed as i16,
        courses_added,
        courses_updated,
        sections_added,
        sections_updated,
        failures,
    ).execute(&mut *transaction).await {
        Ok(_) => log!(Level::Trace, "successfully updated scrapelog table"),
        Err(e) => log!(Level::Warn, "failed to update scrapelog table\nreturned error: {}", e),
    }

    match transaction.commit().await {
        Ok(_) => {
            log!(Level::Trace, "changes committed to database successfully");
            Ok(())
        }
        Err(e) => {
            log!(
                Level::Error,
                "changes could not be committed to database\nreturned error: {}",
                e
            );
            Err(())
        }
    }
}

fn time_from_str(s: &str) -> i16 {
    ((NaiveTime::parse_from_str(s, "%R")
        .unwrap()
        .signed_duration_since(NaiveTime::MIN)
        .to_std()
        .unwrap()
        .as_secs() as i64)
        / 60) as i16
}
