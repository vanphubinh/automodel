-- @automodel
--    description: Update user with optional-nullable age - demonstrates ?? suffix for Option<Option<T>>
--    expect: exactly_one
--    return_type: UserModel
--    error_type: UserContentConstraints
-- @end

UPDATE public.users 
SET updated_at = NOW() 
  #[, name = #{name?}] 
  #[, age = #{age??}] 
WHERE id = #{id} 
RETURNING id, name, email, age
