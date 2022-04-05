pub type HtmlDoc = Vec<Html>;
pub enum Html {
    Line(String),
    Block(HtmlDoc),
    Indent,
    Deindent,
}

#[macro_export]
macro_rules! html {
    ( $x:expr ) => { Html::Line($x) };
    ( $x:expr , $( $xs:expr ),* $(,)? ) => { Html::Block(vec![$x, $($xs),*]) };
}
