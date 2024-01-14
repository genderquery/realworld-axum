// @generated automatically by Diesel CLI.

diesel::table! {
    article_tags (article_id, tag_id) {
        article_id -> Int4,
        tag_id -> Int4,
    }
}

diesel::table! {
    articles (id) {
        id -> Int4,
        slug -> Text,
        title -> Text,
        description -> Text,
        body -> Text,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        author_id -> Int4,
    }
}

diesel::table! {
    comments (id) {
        id -> Int4,
        article_id -> Int4,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        body -> Text,
        author_id -> Int4,
    }
}

diesel::table! {
    favorites (user_id, article_id) {
        user_id -> Int4,
        article_id -> Int4,
    }
}

diesel::table! {
    follows (follower, followee) {
        follower -> Int4,
        followee -> Int4,
    }
}

diesel::table! {
    tags (id) {
        id -> Int4,
        tag -> Text,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        email -> Text,
        username -> Text,
        password_hash -> Text,
        bio -> Text,
        image -> Text,
    }
}

diesel::joinable!(article_tags -> articles (article_id));
diesel::joinable!(article_tags -> tags (tag_id));
diesel::joinable!(articles -> users (author_id));
diesel::joinable!(comments -> articles (article_id));
diesel::joinable!(comments -> users (author_id));
diesel::joinable!(favorites -> articles (article_id));
diesel::joinable!(favorites -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    article_tags,
    articles,
    comments,
    favorites,
    follows,
    tags,
    users,
);
