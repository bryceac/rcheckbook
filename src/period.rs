use clap::ValueEnum;

#[derive(ValueEnum, Clone, Debug)]
pub enum Period {
    All,
    Week,
    Month,
    Quarter,
    HalfYear,
    Year
}