use super::util::establish_connection;
use crate::schema::posts;
use crate::schema::posts::dsl;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Serialize, Deserialize)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
}

#[derive(Insertable)]
#[table_name = "posts"]
struct NewPost<'a> {
    pub title: &'a str,
    pub body: &'a str,
}

impl Post {
    pub fn all() -> Vec<Post> {
        let connection = establish_connection();
        dsl::posts
            .load::<Post>(&connection)
            .expect("Error loading posts")
    }

    pub fn published_all() -> Vec<Post> {
        let connection = establish_connection();
        dsl::posts
            .filter(dsl::published.eq(true))
            .limit(5)
            .load::<Post>(&connection)
            .expect("Error loading posts")
    }

    pub fn find(id: i32) -> Post {
        let connection = establish_connection();
        dsl::posts
            .find(id)
            .first::<Post>(&connection)
            .expect("Error finding posts")
    }

    pub fn create(title: &str, body: &str) {
        let new_post = NewPost {
            title: title,
            body: body,
        };
        let connection = establish_connection();
        diesel::insert_into(posts::table)
            .values(&new_post)
            .execute(&connection)
            .expect("Error saving new post");
    }

    pub fn publish(&mut self) {
        let connection = establish_connection();
        let num_updated = diesel::update(dsl::posts.find(self.id))
            .set(dsl::published.eq(true))
            .execute(&connection)
            .expect(&format!("Unable to find post {}", self.id));

        if num_updated > 0 {
            self.published = true;
        }
    }

    pub fn destroy(&self) {
        let connection = establish_connection();
        diesel::delete(dsl::posts.find(self.id))
            .execute(&connection)
            .expect("Error deleting posts");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unit_sample() {
        let post = Post {
            id: 1,
            title: "title".to_string(),
            body: "body".to_string(),
            published: false,
        };

        assert_eq!(post.id, 1);
        assert_eq!(post.title, "title");
        assert_eq!(post.body, "body");
        assert_eq!(post.published, false);
    }
}
