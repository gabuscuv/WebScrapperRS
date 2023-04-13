use chrono::NaiveDate;

pub struct Event 
    {
        Name : String,
        Type : String,
        StartDate: NaiveDate,
        EndDate: NaiveDate,
        Location: String,
    }
