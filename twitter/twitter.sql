--
-- The Pulsar source version
--
CREATE SOURCE twitter (
    data STRUCT < created_at TIMESTAMP,
    id VARCHAR,
    text VARCHAR,
    lang VARCHAR >,
    author STRUCT < created_at TIMESTAMP,
    id VARCHAR,
    name VARCHAR,
    username VARCHAR,
    followers INT >
) WITH (
    connector = 'pulsar',
    pulsar.topic = 'twitter',
    pulsar.admin.url = 'http://message_queue:8080',
    pulsar.service.url = 'pulsar://message_queue:6650'
) ROW FORMAT JSON;

--
-- The Kafka source version
--
CREATE SOURCE twitter (
    data STRUCT < created_at TIMESTAMP,
    id VARCHAR,
    text VARCHAR,
    lang VARCHAR >,
    author STRUCT < created_at TIMESTAMP,
    id VARCHAR,
    name VARCHAR,
    username VARCHAR,
    followers INT >
) WITH (
    connector = 'kafka',
    kafka.topic = 'twitter',
    kafka.brokers = 'message_queue:29092',
    kafka.scan.startup.mode = 'earliest'
) ROW FORMAT JSON;

--
-- The CREATE TABLE version
--
CREATE TABLE twitter (
    data STRUCT < created_at TIMESTAMP,
    id VARCHAR,
    text VARCHAR,
    lang VARCHAR >,
    author STRUCT < created_at TIMESTAMP,
    id VARCHAR,
    name VARCHAR,
    username VARCHAR,
    followers INT >
);

--
-- Find the influencers
--
CREATE MATERIALIZED VIEW influencer_tweets AS
SELECT
    (author).id as author_id,
    (data).text as tweet
FROM
    twitter
WHERE
    (author).followers > 5000
    AND (data).lang = 'English';

--
-- Find the top10 hotest hashtags.
--
CREATE MATERIALIZED VIEW hot_hashtags AS WITH tags AS (
    SELECT
        unnest(regexp_matches((data).text, '#\w+', 'g')) AS hashtag,
        (data).created_at AS created_at
    FROM
        twitter
)
SELECT
    hashtag,
    COUNT(*) AS hashtag_occurrences,
    window_start
FROM
    TUMBLE(tags, created_at, INTERVAL '1 day')
GROUP BY
    hashtag,
    window_start
ORDER BY
    hashtag_occurrences;

--
-- The Postgres version. The two user-defined types correspond with the struct types in RisingWave.
--
CREATE TYPE twitter_user AS (
    created_at TIMESTAMP,
    id VARCHAR,
    name VARCHAR,
    username VARCHAR,
    followers INT
);

CREATE TYPE tweet_data AS (
    created_at TIMESTAMP,
    id VARCHAR,
    text VARCHAR,
    lang VARCHAR
);

CREATE TABLE twitter (data tweet_data, author twitter_user);