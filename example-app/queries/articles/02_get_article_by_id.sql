-- @automodel
--    description: Get an article by id
--    expect: exactly_one
--    types:
--      public.articles.metadata: "sqlx::types::Json<crate::models::ArticleMetadata>@native"
--      public.articles.contributors: "sqlx::types::Json<Vec<crate::models::ArticleContributor>>@native"
-- @end
SELECT id, title, metadata, contributors
FROM public.articles
WHERE id = #{id};
