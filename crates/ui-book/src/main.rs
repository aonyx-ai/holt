mod stories;
mod stories_docs;

fn main() {
    holt_book::run_book(&stories_docs::STORY_DOCS);
}
