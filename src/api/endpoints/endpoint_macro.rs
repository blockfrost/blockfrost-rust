/// A helpful macro for defining one specific endpoint
macro_rules! def_endpoint {
  // Matches:
  // - Any number of doc comments
  // - A function name
  // - Any number of parameters
  // - A route
  // - A return type
  ($($doc:expr)*, $name:ident, $($param:ident, $ptype:ty,)*; $route:expr, $ret:ty) => {
    // Preserve the doc comments
    $(#[doc = $doc])*
    // Define the simple, unpaginated function, expanding out any parameters
    pub async fn $name(&self$(, $param: $ptype)*) -> Result<$ret> {
      // Build a paginated route, substituting in the params
      // A cool property here is that the parameters defined in the macro will get checked against the route provided
      let route = format!($route $(, $param = $param)*);
      // Make a GET request!
      // TODO: add support for HTTP method?
      self.get(&route).await
    }
  };
}

/// A helpful macro for defining a bunch of simple OpenAPI endpoints
macro_rules! endpoints {
  // Matches:
  // - any number of doc comments
  // - a function name
  // - any number of parameters (names must be in the upcoming route)
  // - an arrow (->)
  // - a return type
  // - a thick arrow (=>)
  // - A string route to hit
  // - A semicolon (;)
  // - The URL to the docs
  ($($(#[doc = $doc:expr])* $name:ident($($param:ident: $ptype:ty$(,)?)*) -> $ret:ty => $route:expr; ($link:tt)$(,)?)*) => {
    $(
      // Forward the above parameters to define a specific endpoint,
      // expanding the URL and route into a reference to the OpenAPI docs.
      def_endpoint! {
        concat!(
          $($doc,)*
          "\n\nOpenAPI endpoint reference: [`", $route, "`].",
          "\n\n[`", $route, "`]: ", $link
        ),
        $name,
        $(
          $param,
          $ptype,
        )*;
        $route,
        $ret
    })*
  };
}
