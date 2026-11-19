use super::{error, Error, Span, MAX_ELEMENTS};

pub(super) fn area(rows: usize, cols: usize, span: &Span) -> Result<usize, Error> {
    if rows == 0 || cols == 0 {
        return Err(error(span, "E_SHAPE", "tensor dimensions must be positive"));
    }
    rows.checked_mul(cols)
        .filter(|n| *n <= MAX_ELEMENTS)
        .ok_or_else(|| {
            error(
                span,
                "E_LIMIT",
                "tensor allocation exceeds element limit or overflows",
            )
        })
}
pub(super) fn product_area(
    left: (usize, usize),
    right: (usize, usize),
    span: &Span,
) -> Result<usize, Error> {
    let (m, k) = left;
    let (j, n) = right;
    use crate::shape::{require_equal, Dimension::Known, Equality};
    if require_equal(Known(k), Known(j)) != Equality::Proven {
        return Err(error(
            span,
            "E_SHAPE",
            format!("[{m}, {k}] @ [{j}, {n}]: inner dimensions {k} and {j} are incompatible"),
        ));
    }
    area(m, n, span)
}

pub(super) fn allocate(size: usize, span: &Span) -> Result<Vec<f64>, Error> {
    size.checked_mul(std::mem::size_of::<f64>())
        .ok_or_else(|| error(span, "E_ALLOCATION", "allocation byte size overflow"))?;
    if size > MAX_ELEMENTS {
        return Err(error(
            span,
            "E_LIMIT",
            "tensor allocation exceeds element limit or overflows",
        ));
    }
    let mut data = Vec::new();
    data.try_reserve_exact(size)
        .map_err(|_| error(span, "E_ALLOCATION", "tensor allocation failed"))?;
    data.resize(size, 0.0);
    Ok(data)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn buffer_allocator_defends_the_final_reservation_boundary() {
        let span = Span::default();
        assert_eq!(
            allocate(MAX_ELEMENTS + 1, &span).unwrap_err().code,
            "E_LIMIT"
        );
        assert_eq!(
            allocate(usize::MAX, &span).unwrap_err().code,
            "E_ALLOCATION"
        );
        assert_eq!(area(1, MAX_ELEMENTS, &span).unwrap(), MAX_ELEMENTS);
        assert_eq!(
            product_area((MAX_ELEMENTS, 1), (1, 2), &span)
                .unwrap_err()
                .code,
            "E_LIMIT"
        );
    }
}
