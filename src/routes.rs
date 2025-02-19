use crate::data::{RecordEn, STAFF};
use crate::error::AppError;
use axum::extract::Path;
use axum::response::{Html, IntoResponse, Response};
use rinja::Template;

macro_rules! simple {
    ($addr: tt, $path: literal) => {
        paste::paste! {
            #[derive(Template)]
            #[template(path = $path)]
            struct [<$addr Template>] {
            }
            pub async fn [<$addr:lower>]() -> Result<Response, AppError> {
                let temp = [<$addr Template>]{};
                let layout = LayoutTemplate { entry: &temp.render()? };
                Ok(Html(layout.render()?).into_response())
            }
        }
    };
}

macro_rules! division {
    ($addr: tt, $path: literal, $idx: literal) => {
        paste::paste! {
            #[derive(Template)]
            #[template(path = $path)]
            struct [<$addr Template>]<'a> {
                entry: &'a [RecordEn],
            }
            pub async fn [<$addr:lower>]() -> Result<Response, AppError> {
                let mut entry: Vec<RecordEn> = vec![];
                for staff in STAFF.get().unwrap() {
                    if let Some(val) = staff.div {
                        if val == $idx {
                            let r = RecordEn::from_record(staff);
                            entry.push(r);
                        }
                    }
                }
                let temp = [<$addr Template>] { entry: &entry };
                let layout = LayoutTemplate {
                    entry: &temp.render()?,
                };
                Ok(Html(layout.render()?).into_response())
            }
        }
    };
}


#[derive(Template)]
#[template(path = "../templates/_layout.html")]
struct LayoutTemplate<'a> {
    entry: &'a str,
}

#[derive(Template)]
#[template(path = "../templates/faculty.html")]
struct FacultyTemplate<'a> {
    entry: &'a [RecordEn],
}

#[derive(Template)]
#[template(path = "../templates/search.html")]
struct SearchTemplate<'a> {
    entry: &'a str,
}

#[derive(Template)]
#[template(path = "../templates/_staff.html")]
struct StaffTemplate<'a> {
    entry: &'a RecordEn,
}

simple!(Index, "../templates/index.html");
simple!(About, "../templates/about.html");
simple!(Mission, "../templates/mission.html");
simple!(Privacy, "../templates/privacy.html");
simple!(Contact, "../templates/contact.html");
division!(Gs, "../templates/divisions/gs.html", 1);
division!(Ns, "../templates/divisions/ns.html", 2);
division!(Cs, "../templates/divisions/cs.html", 3);
division!(Ts, "../templates/divisions/ts.html", 4);
division!(Vs, "../templates/divisions/vs.html", 5);
division!(Pls, "../templates/divisions/pls.html", 6);
division!(Peds, "../templates/divisions/peds.html", 7);

pub async fn faculty() -> Result<Response, AppError> {
    let mut entry: Vec<RecordEn> = vec![];
    for staff in STAFF.get().unwrap() {
        let r = RecordEn::from_record(staff);
        entry.push(r);
    }
    let temp = FacultyTemplate { entry: &entry };
    let layout = LayoutTemplate {
        entry: &temp.render()?,
    };
    Ok(Html(layout.render()?).into_response())
}

pub async fn staff(Path(user_id): Path<String>) -> Result<Response, AppError> {
    let staff = STAFF.get().unwrap();
    let idx = user_id.parse::<usize>()?;
    let idx = if idx > 0 && idx <= staff.len() {
        idx
    } else {
        1
    };
    let staff = &staff[idx - 1];
    let entry = RecordEn::from_record(staff);
    let temp = StaffTemplate { entry: &entry };
    let layout = LayoutTemplate {
        entry: &temp.render()?,
    };
    Ok(Html(layout.render()?).into_response())
}

pub async fn search() -> Result<Response, AppError> {
    let temp = SearchTemplate { entry: "Some name" };
    let layout = LayoutTemplate {
        entry: &temp.render()?,
    };
    Ok(Html(layout.render()?).into_response())
}
