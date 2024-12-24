
pub struct Minutes(pub i32);
pub struct Hours(pub i32);
pub struct Days(pub i32);

impl From<Minutes> for Hours {
    fn from(minutes: Minutes) -> Hours {
        Hours(minutes.0 / 60)
    }
}

impl From<Hours> for Days {
    fn from(value: Hours) -> Days {
        Days(value.0 / 24)
    }
}


impl From<Minutes> for Days {
    fn from(value: Minutes) -> Days {
        Days(value.0 / (60 * 24))
    }
}
 

impl From<Days> for Hours {
    fn from(value: Days) -> Hours {
        Hours(value.0 * 24)
    }
}


