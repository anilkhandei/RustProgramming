pub struct Clock{
    pub hours:i32,
    pub minutes:i32
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let my_clock=Clock{
            hours:hours,
            minutes:minutes
        };
        my_clock
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let new_clock=Clock{
            hours:self.hours,
            minutes: self.minutes+minutes
        };
        new_clock
    }

    pub fn to_string()->String{
        let clock_string=String::from(self.hours+":"+self.minutes);
        clock_string
    }



}
