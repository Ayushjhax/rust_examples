pub struct Post {
    state: Option<Box<dyn State>>,
    content: String,
}

impl Post {
    pub fn new() -> Post {
        Post {
            state: Some(Box::new(Draft {})),
            content: String::new(),
        }
    }

    pub fn add_text(&mut self, text: &str) {
        self.state.as_mut().unwrap().add_text(self, text);
    }

    pub fn content(&self) -> &str {
        self.state.as_ref().unwrap().content(self)
    }

    pub fn request_review(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.request_review());
        }
    }

    pub fn approve(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.approve());
        }
    }
}   

pub struct DraftPost {
    content: String,
}

impl State for DraftPost {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview { content: self.content })
    }
}

struct PendingReview {
    content: String,
}

impl State for PendingReview {
    fn approve(self: Box<Self>) -> Box<dyn State> {
        Box::new(Published { content: self.content })
    }
}

struct Published {
    content: String,
}

impl State for Published {
    fn content<'a>(&'a self, post: &'a Post) -> &'a str {
        &self.content
    }
}


    trait State {
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>) -> Box<dyn State>;
    fn add_text<'a>(&'a self, post: &'a mut Post, text: &'a str);
    fn content<'a>(&'a self, post: &'a Post) -> &'a str {
        ""
    }
}

struct Draft {
    content: String,
}

impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview {})
    }

    fn content<'a>(&'a self, post: &'a Post) -> &'a str {
        &self.content
    }

    fn add_text<'a>(&'a self, post: &'a mut Post, text: &'a str) {
        post.content.push_str(text);
    }
}

struct PendingReview {
    content: String,
}

impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        Box::new(Published {})
    }

    fn add_text<'a>(&'a self, post: &'a mut Post, text: &'a str) {
        post.content.push_str(text);
    }

    fn content<'a>(&'a self, post: &'a Post) -> &'a str {
        &self.content
    }
}

struct Published {}

impl State for Published {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn content<'a>(&'a self, post: &'a Post) -> &'a str {
        &post.content
    }
}