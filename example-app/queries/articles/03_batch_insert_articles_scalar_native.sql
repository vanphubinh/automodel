-- @automodel
--    description: Batch insert articles with scalar @native JSONB types using multiunzip (regression test for type truncation)
--    expect: multiple
--    multiunzip: true
--    types:
--      metadata: "sqlx::types::Json<crate::models::ArticleMetadata>@native"
--      public.articles.metadata: "sqlx::types::Json<crate::models::ArticleMetadata>@native"
--      contributors: "sqlx::types::Json<Vec<crate::models::ArticleContributor>>@native"
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
