-- @automodel
--    description: Batch insert articles with nullable JSONB columns using multiunzip
--    expect: multiple
--    multiunzip: true
--    types:
--      metadata: "Vec<sqlx::types::Json<crate::models::ArticleMetadata>>@native"
--      public.articles.metadata: "sqlx::types::Json<crate::models::ArticleMetadata>@native"
--      contributors: "Vec<sqlx::types::Json<Vec<crate::models::ArticleContributor>>>@native"
--      public.articles.contributors: "sqlx::types::Json<Vec<crate::models::ArticleContributor>>@native"
-- @end
INSERT INTO public.articles (title, metadata, contributors)
SELECT title, metadata, contributors
FROM UNNEST(
        #{title}::text [],
        #{metadata?}::jsonb [],
        #{contributors?}::jsonb []
    ) AS t(title, metadata, contributors)
RETURNING id, title, metadata, contributors;
