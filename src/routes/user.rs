use crate::core::Database;
use crate::core::DatabaseExt;
use crate::core::Gym;
use crate::core::Tab;
use crate::core::Package;
use crate::core::User;
use rocket::response::{Flash, Redirect};
use rocket::Route;
use rocket::State;
use rocket_contrib::templates::tera::Context;
use rocket_contrib::templates::tera::Tera;
use rocket_contrib::templates::Template;

pub fn user() -> Vec<Route> {
    routes![user_home, user_index, update_user_skill, update_user_tab_package, skill_details]
}

#[get("/user?<u>", rank = 2)]
fn user_home(db: State<Database>, u: String) -> Template {
    let user: User = db.get_user(&u).unwrap().unwrap();
    let package = Package::new(&user.packagepath);
    let packages = Package::all();
    let mut context: Context = Context::new();
    context.insert("name", &user.name);
    context.insert("userhash", &user.userurl);
    context.insert("packagepath", &user.packagepath);
    context.insert("package", &package);
    context.insert("packages", &packages);
    Template::render("userhome", &context)
}

#[get("/user?<u>&<s>")]
fn user_index(
    db: State<Database>,
    u: String,
    s: String,
) -> rocket::response::content::Html<String> {
    let user = db.get_user(&u).unwrap().unwrap();
    let package = Package::new(&user.packagepath);
    let tabpath = package.get_tab_path(&s);

    let mut tera = Tera::default();
    tera.add_template_file("./templates/layout.html.tera", Some("layout"))
        .unwrap();
    tera.add_template_file("./templates/user.html.tera", Some("user"))
        .unwrap();
    tera.add_template_file(tabpath, Some(&s)).unwrap();

    let mut context = Context::new();
    context.insert("username", &user.name);
    context.insert("userhash", &u);
    context.insert("package", &package);

    context.insert("skills", &user.skills.clone());

    rocket::response::content::Html(tera.render(&s, &context).unwrap())
}

#[put("/update?<u>&<s>&<v>")]
fn update_user_skill(
    mut db: State<Database>,
    u: String,
    s: String,
    v: usize,
) -> rocket::response::status::Accepted<String> {
    db.update_user_skill(&u, &s, v).unwrap();
    rocket::response::status::Accepted(Some("Success".to_string()))
}

#[put("/package?<u>&<p>")]
fn update_user_tab_package(
    mut db: State<Database>,
    u: String,
    p: String,
) -> rocket::response::status::Accepted<String> {
    db.update_user_tab_package(&u, &p).unwrap();
    rocket::response::status::Accepted(Some("Success".to_string()))
}

#[get("/skill?<s>")]
fn skill_details(s: String) -> rocket::response::content::Html<String> {
    let mut tera = Tera::default();
    let mut context = Context::new();
    context.insert("skill", &s);
    tera.add_template_file("./templates/docs.html.tera", Some("docs")).unwrap();
    tera.add_template_file(format!("./templates/pages/{}", &s), Some("skill")).unwrap();
    rocket::response::content::Html(tera.render("skill", &context).unwrap())
}
