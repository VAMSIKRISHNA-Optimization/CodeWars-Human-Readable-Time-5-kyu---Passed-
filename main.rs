fn make_readable(seconds: u32) -> String 
{
    if seconds == 0 { return "00:00:00".to_string(); }
    
    match seconds
    {
        seconds if seconds > 0 && seconds < 60 => 
        {
            let ans = format!("00:00:{:02}", seconds); 
            return ans;
        }
        seconds if seconds >= 60 && seconds < 3600 =>
        {
            let ans = format!("00:{:02}:{:02}", seconds/60, seconds%60); 
            return ans;
        }
        seconds if seconds >= 3600 =>
        {
            let ans = format!("{:02}:{:02}:{:02}", seconds/3600, (seconds%3600)/60, (seconds%3600)%60); 
            return ans;
        }
        _=> return "Unknown".to_string(),
    }
    
}
 
    


fn main ()
{
    println!("{:?}", make_readable(86400));
}
