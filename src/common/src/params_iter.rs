/// Helper function to reduce boilerplate when doing raw postgres streaming queries.
pub fn params_iter<'a>(
    s: &'a [&'a (dyn postgres_types::ToSql + Sync)],
) -> impl ExactSizeIterator<Item = &'a dyn postgres_types::ToSql> + 'a {
    s.iter().map(|s| *s as _)
}
