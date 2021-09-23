macro_rules! def_endpoint {
  ($($doc:expr)*, $name:ident, $route:expr, $ret:ty) => {
      $(#[doc = $doc])*
      pub async fn $name(&self) -> Result<$ret> {
        self.get($route).await
      }
  };
  ($($doc:expr)*, $name:ident, $param:ident, $ptype:ty, $route:expr, $ret:ty) => {
    $(#[doc = $doc])*
    pub async fn $name(&self, $param: $ptype) -> Result<$ret> {
      let route = format!($route, $param = $param);
      self.get(&route).await
    }
  };
}

macro_rules! endpoints {
  ($($(#[doc = $doc:expr])* $name:ident($($param:ident: $ptype:ty)?) -> $ret:ty => $route:expr; ($link:tt)$(,)?)*) => {
    $(
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
        )?
        $route,
        $ret
    })*
  };
}