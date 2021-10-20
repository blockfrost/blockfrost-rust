// Useful for testing our types against JSON examples in the documentation.
#[cfg(test)]
macro_rules! test_example {
    { $test_name:tt, $schema_name:ty, $text:expr } => {
        #[test]
        fn $test_name() {
            let parsed = serde_json::from_str::<$schema_name>($text);
            // Expected not to crash
            parsed.unwrap();
        }
    };
}

/// A helpful macro for defining one specific endpoint returning a singular object or a single page
macro_rules! def_endpoint {
  // Matches:
  // - A doc comment
  // - A function name
  // - Any number of parameters
  // - A route
  // - A return type
  ($doc:expr, $name:ident, $($param:ident, $ptype:ty,)*; $route:expr, $ret:ty) => {
    // Preserve the doc comments
    #[doc = $doc]
    // Define the simple, unpaginated function, expanding out any parameters
    pub fn $name<'api>(
        &'api self
        $(, $param: $ptype)*
    ) -> impl std::future::Future<Output = Result<$ret>> + Send + 'api {
        // Build a paginated route, substituting in the params
        // A cool property here is that the parameters defined in the macro will get checked against the route provided
        let route = format!($route $(, $param = $param)*);
        // Make a GET request!
        self.get_from_endpoint(&route)
    }
  };
}

/// A helpful macro for defining one specific endpoint, with an _all paginated variant
macro_rules! def_endpoint_paginated {
  // Matches:
  // - A doc comment for the singular method
  // - A doc comment for the "all" method
  // - A function name
  // - Any number of parameters
  // - A route
  // - A return type
  ($doc:expr, $all_doc:expr, $name:ident, $($param:ident, $ptype:ty,)*; $route:expr, $ret:ty) => {
    // Define the singular version
    def_endpoint!(
      $doc,
      $name,
      $(
        $param,
        $ptype,
      )*;
      $route,
      $ret
    );

    // Use paste! to append _all to the name, for the paginated variant
    paste::paste! {
      #[doc = $all_doc]
      pub fn [<$name _all>]<'api>(
        &'api self
        $(, $param: $ptype)*
      ) -> Lister<'api, $ret> {
        let endpoint = format!($route $(, $param = $param)*);
        Lister::list_from_endpoint(self, endpoint)
      }
    }
  };
}

/// A helpful macro for defining a bunch of simple, unpaged OpenAPI endpoints
macro_rules! endpoints {
  ($($(#[doc = $doc:expr])* $name:ident($($param:ident: $ptype:ty$(,)?)*) -> $ret:ty => $route:expr; ($link:tt)$(,)?)*) => {
    $(
      // Forward the above parameters to define a specific endpoint,
      // expanding the URL and route into a reference to the OpenAPI docs.
      def_endpoint_paginated! {
        concat!(
          $($doc,)*
          "\n\nOpenAPI endpoint reference: [`", $route, "`].",
          "\n\n[`", $route, "`]: ", $link
        ),
        concat!(
          $($doc,)*
          "\n\nResults are paginated.",
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
  }
}

/// A helpful macro for defining a bunch of paginated OpenAPI endpoints
macro_rules! paged_endpoints {
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
      def_endpoint_paginated! {
        concat!(
          $($doc,)*
          "\n\nOpenAPI endpoint reference: [`", $route, "`].",
          "\n\n[`", $route, "`]: ", $link
        ),
        concat!(
          $($doc,)*
          "\n\nResults are paginated.",
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
