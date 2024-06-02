use crate::{
    api::AppState,
    model::{Building, CourseOption, Search, SectionTimeInfo, SingleSection, Time, WeekDay},
};
use actix_web::{
    post,
    web::{Data, Json},
    HttpResponse, Responder,
};
use log::{log, Level};
use sqlx::{Postgres, QueryBuilder};

#[utoipa::path(
    context_path = "/api",
    request_body(content=Search, content_type="application/json", description="Course Preferences"),
    responses(
        (status = 200, description = "List all course sections matching criteria", body = [CourseOption]),
        (status = 500, description = "Error Created by Query"),
    )
)]
#[post("/generate/getCourseOpts")]
pub async fn get_course_options(state: Data<AppState>, options: Json<Search>) -> impl Responder {
    log!(Level::Info, "GET /generate/getCourseOpts");
    let course_number = format!("{}%", options.course).to_uppercase();

    let mut query_builder: QueryBuilder<Postgres> = QueryBuilder::new(
        "
            SELECT c.id AS course_id,
                c.course,
                c.title,
                c.description,
                c.credits,
                s.id AS section_id,
                s.curr_enroll,
                s.max_enroll,
                s.instructor,
                d.code AS dept_code
            FROM courses c, departments d, sections s
            WHERE
                c.id = s.course AND
                c.department = d.id AND
                s.status != 'X' AND
                c.term = $1 AND
                (d.code || '-' || c.course || '-' || s.section) LIKE '$2'",
    );

    query_builder.push_bind(options.term);
    query_builder.push_bind(course_number);

    let sections: Vec<SingleSection> = match query_builder
        .build_query_as::<SingleSection>()
        .fetch_all(&state.db)
        .await
    {
        Ok(sections) => sections,
        Err(e) => return HttpResponse::InternalServerError().json(e.to_string()),
    };

    let stem = "
        SELECT t.day,
            t.start_time,
            t.end_time,
            t.room,
            b.code,
            b.number,
            b.off_campus
        FROM courses c, sections s, buildings b, times t
        WHERE
            c.id = s.course AND
            t.section = s.id AND
            t.building = b.id";

    let mut qb: QueryBuilder<Postgres> = QueryBuilder::new(stem);

    if !options.ignore_full {
        qb.push(" AND s.max_enroll != s.curr_enroll");
    }

    if let Some(credit_hours) = options.credit_hours {
        if (0..=12).contains(&credit_hours) {
            qb.push(" AND c.credits = $1");
            qb.push_bind(credit_hours);
        }
    }

    if let Some(title) = &options.title {
        qb.push(" AND UPPER(c.title) LIKE '$1'");
        qb.push_bind(format!("%{}%", title.to_uppercase()));
    }

    if let Some(professor_name) = &options.professor_name {
        qb.push(" AND UPPER(c.instructor) LIKE '$1'");
        qb.push_bind(format!("%{}%", professor_name.to_uppercase()));
    }

    if let Some(keywords) = &options.keywords {
        for keyword in keywords {
            qb.push(" AND UPPER(c.description) LIKE '$1'");
            qb.push_bind(keyword);
        }
    }

    if let Some(false) = options.honors {
        qb.push(" AND NOT c.description LIKE 'honors'");
    }

    qb.push(" AND c.id = $1 AND s.id = $2");

    let courses: Vec<CourseOption> = sections
        .into_iter()
        .map(|section| {
            qb.push_bind(section.course_id);
            qb.push_bind(section.section_id);

            if let Some(days) = &options.days {
                for (i, day) in days.iter().enumerate() {
                    if *day {
                        qb.push(" AND t.day = $1");
                        qb.push_bind(i as i32);
                    }
                }
            }

            if let Some(false) = options.online {
                qb.push(" AND NOT b.code = 'ON'");
            }

            if let Some(false) = options.off_campus {
                qb.push(" AND NOT b.code = 'OFFC'");
            }

            let section_times: Vec<SectionTimeInfo> = match futures::executor::block_on(
                qb.build_query_as::<SectionTimeInfo>().fetch_all(&state.db),
            ) {
                Ok(times) => times,
                Err(_) => vec![],
            };

            let times: Vec<Time> = section_times
                .into_iter()
                .map(|sec_time| {
                    let building: Building = Building {
                        code: sec_time.code,
                        number: sec_time.number,
                    };

                    let weekday: WeekDay = match sec_time.day {
                        0 => WeekDay::Sunday,
                        1 => WeekDay::Monday,
                        2 => WeekDay::Tuesday,
                        3 => WeekDay::Wednesday,
                        4 => WeekDay::Thursday,
                        5 => WeekDay::Friday,
                        6 => WeekDay::Saturday,
                        _ => panic!("invalid value received from times.day"),
                    };

                    Time {
                        building,
                        day: weekday,
                        start: sec_time.start_time as u32,
                        end: sec_time.end_time as u32,
                        room: sec_time.room,
                        off_campus: sec_time.off_campus,
                    }
                })
                .collect::<Vec<Time>>();

            qb.reset();

            CourseOption {
                course_num: format!("{}-{}", section.dept_code, section.course_id),
                title: section.title,
                description: section.description,
                credits: section.credits,
                enrolled_students: section.curr_enroll,
                enrollment_capacity: section.max_enroll,
                instructor_name: section.instructor,
                online: times[0].building.code == "ON",
                times,
            }
        })
        .collect::<Vec<CourseOption>>();

    HttpResponse::Ok().json(courses)
}
