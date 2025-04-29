/// ## Data Enum
/// Holds the data of the field in the csv.
/// Allows to handle different datatypes, parsing it in runtime.
#[derive(Debug)]
#[allow(dead_code)]
pub enum Data{
    Text(String),
    Byte(i8),
    UByte(u8),
    Short(i16),
    UShort(u16),
    Integer(i32),
    UInteger(u32),
    Long(i64),
    ULong(u64),
    Float(f32),
    Double(f64),
    Boolean(bool),
    Empty,
    AutoDetect(&'static str)
}


impl std::fmt::Display for Data{
    /// ## Display implementation
    /// Implements Display trait , for al the fields of the enum `Data`
    ///
    /// ### Use case:
    /// ```
    /// use csv_lib::models::data::Data;
    /// let d = Data::Text(String::from("hello"));
    /// println!("{}", d);
    /// ```
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Data::Text(v) => {
                write!(f, "{}", v)}   
            Data::Byte(v) => {
                write!(f, "{}", v)}
            Data::UByte(v) => {
                write!(f, "{}", v)}
            Data::Short(v) => {
                write!(f, "{}", v)}
            Data::UShort(v) => {
                write!(f, "{}", v)}
            Data::Integer(v) => {
                write!(f, "{}", v)}
            Data::UInteger(v) => {
                write!(f, "{}", v)}
            Data::Long(v) => {
                write!(f, "{}", v)}
            Data::ULong(v) => {
                write!(f, "{}", v)}
            Data::Float(v) => {
                write!(f, "{}", v)}
            Data::Double(v) => {
                write!(f, "{}", v)}
            Data::Boolean(v) => {
                write!(f, "{}", v) }
            Data::Empty => {
                write!(f, "")
            } 
            Data::AutoDetect(v) => {
                write!(f, "{}",v)
            }
        }
    }
}
mod test_data{
    #[test]
    fn test_display_data(){
        let d = crate::models::data::Data::Text(String::from("hello"));
        assert_eq!("hello", format!("{}", d));
    }
}