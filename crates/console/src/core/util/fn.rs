// source: core/src/util/fn.ts
//! 1:1 port of the schema-validating `fn` wrapper.
//! Source pin: v1.18.30 @3104c14.

/// A validator produced by [`schema`], standing in for the `zod` schema object
/// the source passes around. `parse` is the whole of the schema's runtime
/// behavior at this call site.
pub trait Schema<T> {
    fn parse(&self, input: &T) -> Result<T, String>;
}

/// `fn(schema, cb)` — the returned value validates on `call` and skips
/// validation on `force`, exactly as the source's `result` / `result.force`.
pub struct Fn<S, T, R> {
    schema: S,
    cb: Box<dyn Fn(T) -> R>,
}

impl<S, T, R: Clone> Fn<S, T, R> {
    /// `fn(schema, cb)`.
    pub fn new(schema: S, cb: impl Fn(T) -> R + 'static) -> Self {
        Self {
            schema,
            cb: Box::new(cb),
        }
    }

    /// `result(input)` — `schema.parse(input)` then `cb(parsed)`.
    pub fn call(&self, input: T) -> Result<R, String>
    where
        S: Schema<T>,
    {
        let parsed = self.schema.parse(&input)?;
        Ok((self.cb)(parsed))
    }

    /// `result.force(input)` — `cb(input)` with no validation.
    pub fn force(&self, input: T) -> R {
        (self.cb)(input)
    }

    /// `result.schema`.
    pub fn schema(&self) -> &S {
        &self.schema
    }
}
