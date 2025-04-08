use crate::database::DB;
use futures_util::StreamExt;
use rauthy_error::ErrorResponse;
use tokio::pin;
use tokio_pg_mapper::FromTokioPostgresRow;

/// Helper function to reduce boilerplate when doing raw postgres streaming queries.
/// This is needed to make postgres `query_raw()` work easily.
#[inline]
pub fn params_iter<'a>(
    s: &'a [&'a (dyn postgres_types::ToSql + Sync)],
) -> impl ExactSizeIterator<Item = &'a dyn postgres_types::ToSql> + 'a {
    s.iter().map(|s| *s as _)
}

#[inline]
pub async fn pg_execute(
    stmt: &str,
    params: &[&(dyn postgres_types::ToSql + Sync)],
) -> Result<u64, ErrorResponse> {
    let cl = DB::pg().await?;
    let st = cl.prepare(stmt).await?;
    let rows_affected = cl.execute(&st, params).await?;
    Ok(rows_affected)
}

#[inline]
pub async fn pg_query_one<T: FromTokioPostgresRow>(
    stmt: &str,
    params: &[&(dyn postgres_types::ToSql + Sync)],
) -> Result<T, ErrorResponse> {
    let cl = DB::pg().await?;
    let st = cl.prepare(stmt).await?;
    let row = cl.query_one(&st, params).await?;
    Ok(T::from_row(row)?)
}

#[inline]
pub async fn pg_query_opt<T: FromTokioPostgresRow>(
    stmt: &str,
    params: &[&(dyn postgres_types::ToSql + Sync)],
) -> Result<Option<T>, ErrorResponse> {
    let cl = DB::pg().await?;
    let st = cl.prepare(stmt).await?;
    let row = cl.query_opt(&st, params).await?;
    match row {
        None => Ok(None),
        Some(row) => Ok(Some(T::from_row(row)?)),
    }
}

/// If you can roughly estimate how many rows would be returned from the given query, provide a
/// proper `expected_size_hint` to reserve memory in advance. This will provide a small performance
/// boost.
#[inline]
pub async fn pg_query<'a, T: FromTokioPostgresRow>(
    stmt: &str,
    params: &'a [&'a (dyn postgres_types::ToSql + Sync)],
    expected_rows_size_hint: usize,
) -> Result<Vec<T>, ErrorResponse> {
    let cl = DB::pg().await?;
    let st = cl.prepare(stmt).await?;
    let s = cl.query_raw(&st, params_iter(params)).await?;
    pin!(s);

    let mut res: Vec<T> = Vec::with_capacity(expected_rows_size_hint);
    while let Some(row) = s.next().await {
        res.push(T::from_row(row?)?);
    }

    Ok(res)
}
