use crate::data::{Record, STAFF};
use crate::error::AppError;
use crate::utils::{language, Language};
use axum::extract::{OriginalUri, Path};
use axum::http::Uri;
use axum::response::{Html, IntoResponse, Response};
use rinja::Template;

macro_rules! simple {
    ($addr: tt, $path: literal) => {
        paste::paste! {
            #[derive(Template)]
            #[template(path = $path)]
            struct [<$addr Template>] {
                lang: Language,
            }
            pub async fn [<$addr:lower>](OriginalUri(uri): OriginalUri) -> Result<Response, AppError> {
                let lang = language(&uri);
                let temp = [<$addr Template>]{ lang };
                let layout = LayoutTemplate::new(
                    lang,
                    temp.render()?,
                    &uri
                );
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
            struct [<$addr Template>] {
                lang: Language,
            }
            pub async fn [<$addr:lower>](OriginalUri(uri): OriginalUri) -> Result<Response, AppError> {
                let lang = language(&uri);
                let mut entry: Vec<Record> = vec![];
                for staff in STAFF.get().unwrap() {
                    if let Some(val) = staff.div {
                        if val == $idx {
                            let r = Record::from_data(staff, lang);
                            entry.push(r);
                        }
                    }
                }
                let temp = [<$addr Template>] { lang };
                let list = ListTemplate { lang, entry: &entry };
                let div = DivTemplate { lang, about: &temp.render()?, list: &list.render()? };
                let layout = LayoutTemplate::new(
                    lang,
                    div.render()?,
                    &uri
                );
                Ok(Html(layout.render()?).into_response())
            }
        }
    };
}

#[derive(Template)]
#[template(path = "../templates/_layout.html")]
struct LayoutTemplate {
    lang: Language,
    entry: String,
    corresponding: String,
}

impl LayoutTemplate {
    fn new(lang: Language, entry: String, uri: &Uri) -> Self {
        let corresponding = if lang == Language::English {
            format!("/ar{}", uri)
        } else {
            uri.path().strip_prefix("/ar").unwrap().to_string()
        };
        Self {
            lang,
            entry,
            corresponding,
        }
    }
}

#[derive(Template)]
#[template(path = "../templates/faculty.html")]
struct FacultyTemplate<'a> {
    lang: Language,
    list: &'a str,
}

#[derive(Template)]
#[template(path = "../templates/_staff.html")]
struct StaffTemplate<'a> {
    lang: Language,
    title: &'a str,
    entry: &'a Record,
}

#[derive(Template)]
#[template(path = "../templates/divisions/_division.html")]
struct DivTemplate<'a> {
    lang: Language,
    about: &'a str,
    list: &'a str,
}

#[derive(Template)]
#[template(path = "../templates/_list.html")]
struct ListTemplate<'a> {
    lang: Language,
    entry: &'a [Record],
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

pub async fn faculty(OriginalUri(uri): OriginalUri) -> Result<Response, AppError> {
    let lang = language(&uri);
    let mut entry: Vec<Record> = vec![];
    for staff in STAFF.get().unwrap() {
        let r = Record::from_data(staff, lang);
        entry.push(r);
    }
    let list = ListTemplate {
        lang,
        entry: &entry,
    };
    let temp = FacultyTemplate {
        lang,
        list: &list.render()?,
    };
    let layout = LayoutTemplate::new(lang, temp.render()?, &uri);
    Ok(Html(layout.render()?).into_response())
}

pub async fn staff(
    Path(user_id): Path<String>,
    OriginalUri(uri): OriginalUri,
) -> Result<Response, AppError> {
    let lang = language(&uri);
    let staff = STAFF.get().unwrap();
    let idx = user_id.parse::<usize>()?;
    let idx = if idx > 0 && idx <= staff.len() {
        idx
    } else {
        1
    };
    let staff = &staff[idx - 1];
    let entry = Record::from_data(staff, lang);
    let title = if entry.rank == "Professor" {
        if lang == Language::English {
            "Prof"
        } else {
            "أ"
        }
    } else if lang == Language::English {
        "Dr"
    } else {
        "د"
    };
    let temp = StaffTemplate {
        lang,
        title,
        entry: &entry,
    };
    let layout = LayoutTemplate::new(lang, temp.render()?, &uri);
    Ok(Html(layout.render()?).into_response())
}
