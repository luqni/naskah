use ast::LiteralValue;

named!(
  boolean<bool>,
  alt!(map!(tag!("benar"), |_| true) | map!(tag!("salah"), |_| false))
);

named!(
  pub boolean_literal<LiteralValue>,
  map!(boolean, |b| LiteralValue::Boolean(b))
);

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_boolean() {
    assert_eq!(boolean(&b"benar"[..]), Ok((&b""[..], true)));
    assert_eq!(boolean(&b"salah"[..]), Ok((&b""[..], false)));
  }

  #[test]
  fn test_boolean_literal() {
    assert_eq!(
      boolean_literal(&b"benar"[..]),
      Ok((&b""[..], LiteralValue::Boolean(true)))
    );
    assert_eq!(
      boolean_literal(&b"salah"[..]),
      Ok((&b""[..], LiteralValue::Boolean(false)))
    );
  }
}