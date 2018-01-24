table! {
    annotation (annotation_id) {
        annotation_id -> Text,
        trace_id -> Text,
        span_id -> Text,
    }
}

table! {
    endpoint (endpoint_id) {
        endpoint_id -> Text,
        service_name -> Nullable<Text>,
        ipv4 -> Nullable<Text>,
        ipv6 -> Nullable<Text>,
        port -> Nullable<Integer>,
    }
}

table! {
    ingest (id) {
        id -> Nullable<Text>,
        created_at -> Text,
        processed_at -> Nullable<Text>,
    }
}

table! {
    span (trace_id, id) {
        trace_id -> Text,
        id -> Text,
        parent_id -> Nullable<Text>,
        name -> Nullable<Text>,
        kind -> Nullable<Text>,
        duration -> Nullable<BigInt>,
        ts -> Nullable<BigInt>,
        debug -> Nullable<Bool>,
        shared -> Nullable<Bool>,
        local_endpoint_id -> Nullable<Text>,
        remote_endpoint_id -> Nullable<Text>,
    }
}

table! {
    tag (tag_id) {
        tag_id -> Text,
        name -> Text,
    }
}

table! {
    tag_value (tag_value_id) {
        tag_value_id -> Text,
        tag_id -> Text,
        trace_id -> Text,
        span_id -> Text,
        value -> Text,
    }
}

table! {
    test (id) {
        id -> Nullable<Text>,
        name -> Text,
    }
}

table! {
    test_result (id) {
        id -> Nullable<Text>,
        test_id -> Text,
        result -> Text,
        duration -> BigInt,
        ts -> BigInt,
    }
}

joinable!(tag_value -> tag (tag_id));

allow_tables_to_appear_in_same_query!(
    annotation,
    endpoint,
    ingest,
    span,
    tag,
    tag_value,
    test,
    test_result,
);
