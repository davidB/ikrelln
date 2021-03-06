CREATE TABLE endpoint
(
    endpoint_id VARCHAR(36) NOT NULL PRIMARY KEY,
    service_name VARCHAR(255),
    ipv4 VARCHAR(30),
    ipv6 VARCHAR(50),
    port INT
);
CREATE TABLE span
(
    trace_id VARCHAR(36) NOT NULL,
    id VARCHAR(36) NOT NULL,
    parent_id VARCHAR(36),
    name VARCHAR(36),
    kind VARCHAR(10),
    duration BIGINT,
    ts BIGINT,
    debug BOOLEAN NOT NULL,
    shared BOOLEAN NOT NULL,
    local_endpoint_id VARCHAR(36),
    remote_endpoint_id VARCHAR(36),
    PRIMARY KEY (trace_id, id),
    FOREIGN KEY (local_endpoint_id) REFERENCES endpoint (endpoint_id),
    FOREIGN KEY (remote_endpoint_id) REFERENCES endpoint (endpoint_id)
);
CREATE TABLE annotation
(
    annotation_id VARCHAR(36) NOT NULL PRIMARY KEY,
    trace_id VARCHAR(36) NOT NULL,
    span_id VARCHAR(36) NOT NULL,
    ts BIGINT NOT NULL,
    value VARCHAR(255) NOT NULL,
    FOREIGN KEY (trace_id, span_id) REFERENCES span (trace_id, id)
);
CREATE TABLE tag
(
    tag_id VARCHAR(36) NOT NULL PRIMARY KEY,
    trace_id VARCHAR(36) NOT NULL,
    span_id VARCHAR(36) NOT NULL,
    name VARCHAR(255) NOT NULL,
    value VARCHAR(255) NOT NULL,
    FOREIGN KEY (trace_id, span_id) REFERENCES span (trace_id, id)
);
